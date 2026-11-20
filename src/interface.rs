//! Local interface switches supported by Horizon's render hooks.
use mod_api::text::TextContent;
use mod_api::{ClientHost, ConfigPath, Text};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub struct Options {
    disabled: AtomicUsize,
    loaded: AtomicBool,
    dirty: AtomicBool,
}

#[derive(Clone, Copy)]
pub enum Setting {
    Hud,
    Hand,
    GameMessages,
    DeathMessages,
    AdvancementMessages,
    SmoothControllerMovement,
}

const SETTINGS: [(Setting, &str); 6] = [
    (Setting::Hud, "display_hud"),
    (Setting::Hand, "display_hand"),
    (Setting::GameMessages, "display_game_messages"),
    (Setting::DeathMessages, "death_messages"),
    (Setting::AdvancementMessages, "announce_advancements"),
    (
        Setting::SmoothControllerMovement,
        "smooth_controller_movement",
    ),
];

impl Default for Options {
    fn default() -> Self {
        Self {
            disabled: AtomicUsize::new(1 << Setting::AdvancementMessages as usize),
            loaded: AtomicBool::new(false),
            dirty: AtomicBool::new(false),
        }
    }
}

impl Options {
    pub fn enabled(&self, setting: Setting) -> bool {
        self.disabled.load(Ordering::Relaxed) & (1 << setting as usize) == 0
    }

    pub fn toggle(&self, setting: Setting) {
        self.disabled
            .fetch_xor(1 << setting as usize, Ordering::Relaxed);
        self.dirty.store(true, Ordering::Relaxed);
        crate::controller::set_smooth(self.enabled(Setting::SmoothControllerMovement));
    }

    pub fn display_hand(&self) -> bool {
        self.enabled(Setting::Hand)
    }

    pub fn suppress_message(&self, system: bool, text: &Text) -> bool {
        system
            && (!self.enabled(Setting::GameMessages)
                || (!self.enabled(Setting::DeathMessages) && is_death_message(text))
                || (!self.enabled(Setting::AdvancementMessages) && is_advancement(text)))
    }

    pub fn sync(&self, host: &dyn ClientHost) {
        if self.loaded.load(Ordering::Relaxed) && !self.dirty.load(Ordering::Relaxed) {
            return;
        }
        let path = ConfigPath::new("legacy/interface.json").expect("owned config path");
        let files = host.config_files();
        if !self.loaded.swap(true, Ordering::Relaxed) && !self.dirty.load(Ordering::Relaxed) {
            if let Ok(bytes) = files.read(&path) {
                if let Ok(value) = serde_json::from_slice::<serde_json::Value>(&bytes) {
                    let mut disabled = 0;
                    for (setting, key) in SETTINGS {
                        let default = !matches!(setting, Setting::AdvancementMessages);
                        if !value[key].as_bool().unwrap_or(default) {
                            disabled |= 1 << setting as usize;
                        }
                    }
                    self.disabled.store(disabled, Ordering::Relaxed);
                }
            }
        }
        if self.dirty.swap(false, Ordering::Relaxed) {
            let value: serde_json::Map<String, serde_json::Value> = SETTINGS
                .into_iter()
                .map(|(setting, key)| (key.into(), self.enabled(setting).into()))
                .collect();
            let bytes = serde_json::to_vec_pretty(&value).expect("boolean config");
            if let Err(error) = files.write(&path, &bytes) {
                eprintln!("Legacy Horizon: could not save interface options: {error:?}");
            }
        }
        crate::controller::set_smooth(self.enabled(Setting::SmoothControllerMovement));
    }
}

fn is_death_message(text: &Text) -> bool {
    let mut pending = vec![text];
    while let Some(node) = pending.pop() {
        if let TextContent::Translatable { key, arguments, .. } = &node.content {
            if key.starts_with("death.") {
                return true;
            }
            pending.extend(arguments);
        }
        pending.extend(&node.children);
    }
    false
}

fn is_advancement(text: &Text) -> bool {
    matches!(&text.content, TextContent::Translatable { key, .. } if key.starts_with("chat.type.advancement."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn death_filter_uses_component_keys_and_keeps_player_chat() {
        let options = Options::default();
        options.toggle(Setting::DeathMessages);
        let death = Text::literal("").append(Text::translate(
            "wrapper",
            vec![Text::translate("death.attack.fall", vec![])],
        ));
        assert!(options.suppress_message(true, &death));
        assert!(!options.suppress_message(false, &death));
        assert!(!options.suppress_message(true, &Text::literal("death.attack.fall")));
        assert!(!options.suppress_message(true, &Text::translate("commands.time.set", vec![])));
    }

    #[test]
    fn defaults_and_game_message_switch_match_source() {
        let options = Options::default();
        let advancement = Text::translate("chat.type.advancement.task", vec![]);
        assert!(options.suppress_message(true, &advancement));
        options.toggle(Setting::AdvancementMessages);
        assert!(!options.suppress_message(true, &advancement));
        options.toggle(Setting::GameMessages);
        assert!(options.suppress_message(true, &Text::literal("Server notice")));
        assert!(!options.suppress_message(false, &Text::literal("Player chat")));
    }
}
