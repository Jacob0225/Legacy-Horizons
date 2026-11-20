//! Legacy4J's cave/minecart sound switches, including live loop refreshes.
use mod_api::{ClientHost, ConfigPath, SoundPlay};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[derive(Default)]
pub struct Options {
    disabled: AtomicUsize,
    loaded: AtomicBool,
    dirty: AtomicBool,
}

impl Options {
    pub fn enabled(&self, index: usize) -> bool {
        self.disabled.load(Ordering::Relaxed) & (1 << index) == 0
    }

    pub fn toggle(&self, index: usize) {
        self.disabled.fetch_xor(1 << index, Ordering::Relaxed);
        self.dirty.store(true, Ordering::Relaxed);
    }

    // Called from normal screen/tick hooks (the World API phase also covers
    // title rendering). The scoped file service does not need a loaded level.
    pub fn sync(&self, host: &dyn ClientHost) {
        if self.loaded.load(Ordering::Relaxed) && !self.dirty.load(Ordering::Relaxed) {
            return;
        }
        let path = ConfigPath::new("legacy/audio.json").expect("owned config path");
        let files = host.config_files();
        if !self.loaded.swap(true, Ordering::Relaxed) && !self.dirty.load(Ordering::Relaxed) {
            if let Ok(bytes) = files.read(&path) {
                if let Ok(value) = serde_json::from_slice::<serde_json::Value>(&bytes) {
                    let cave = value["cave_sounds"].as_bool().unwrap_or(true);
                    let cart = value["minecart_sounds"].as_bool().unwrap_or(true);
                    let elytra = value["legacy_elytra_sound"].as_bool().unwrap_or(true);
                    self.disabled.store(
                        usize::from(!cave)
                            | (usize::from(!cart) << 1)
                            | (usize::from(!elytra) << 2),
                        Ordering::Relaxed,
                    );
                }
            }
        }
        if self.dirty.swap(false, Ordering::Relaxed) {
            let bytes = serde_json::to_vec_pretty(&serde_json::json!({
                "cave_sounds": self.enabled(0), "minecart_sounds": self.enabled(1),
                "legacy_elytra_sound": self.enabled(2)
            }))
            .expect("boolean config");
            if let Err(error) = files.write(&path, &bytes) {
                eprintln!("Legacy Horizon: could not save audio options: {error:?}");
            }
        }
    }

    pub fn filter(&self, sound: &mut Option<SoundPlay>) {
        let Some(current) = sound.as_ref() else {
            return;
        };
        if suppressed(&current.identifier, self.enabled(0), self.enabled(1)) {
            *sound = None;
        } else if self.enabled(2) {
            if let Some(current) = sound.as_mut() {
                legacy_elytra(current);
            }
        }
    }
}

fn legacy_elytra(sound: &mut SoundPlay) {
    // ElytraOnPlayerSoundInstanceMixin: change the instance's own volume and
    // pitch, before the player's mixer settings, on starts and live refreshes.
    if sound.identifier == "minecraft:item.elytra.flying" {
        sound.volume = sound.volume.min(0.8);
        sound.pitch = 1.0;
    }
}

fn suppressed(id: &str, cave: bool, cart: bool) -> bool {
    (!cave && id == "minecraft:ambient.cave")
        || (!cart
            && matches!(
                id,
                "minecraft:entity.minecart.riding"
                    | "minecraft:entity.minecart.inside"
                    | "minecraft:entity.minecart.inside.underwater"
            ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sound_switches_are_independent_and_do_not_mute_other_ambience() {
        assert!(suppressed("minecraft:ambient.cave", false, true));
        assert!(!suppressed(
            "minecraft:ambient.underwater.loop",
            false,
            false
        ));
        assert!(!suppressed("minecraft:entity.minecart.riding", false, true));
        assert!(suppressed(
            "minecraft:entity.minecart.inside.underwater",
            true,
            false
        ));
        assert!(!suppressed("another:entity.minecart.riding", false, false));
    }

    #[test]
    fn elytra_audio_caps_volume_without_raising_quiet_flight() {
        let mut sound = SoundPlay::ui("minecraft:item.elytra.flying");
        sound.volume = 0.95;
        sound.pitch = 1.4;
        legacy_elytra(&mut sound);
        assert_eq!((sound.volume, sound.pitch), (0.8, 1.0));
        sound.volume = 0.2;
        legacy_elytra(&mut sound);
        assert_eq!(sound.volume, 0.2);
        sound.identifier = "minecraft:entity.pig.ambient".into();
        sound.pitch = 0.7;
        legacy_elytra(&mut sound);
        assert_eq!(sound.pitch, 0.7);
    }
}
