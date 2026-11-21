//! Legacy4J's pause menu (`PauseScreenMixin`): one 225-wide column of
//! 20-high buttons, 5 apart, from `width / 2 - 112`, `height / 3 + 5`, over
//! the stock pause screen, whose own drawing and buttons are replaced.
//!
//! Resume Game closes the screen as Escape does. Exit Game asks first, as
//! Legacy4J's `ExitConfirmationScreen` does without its save system, then
//! leaves as the stock Disconnect does (a single-player world is saved).
//! Help & Options, Leaderboards, Advancements and Save Game are shown
//! disabled: the pinned mod API can't open those screens from here.

use super::{
    BUTTON_GAP, BUTTON_HEIGHT, BUTTON_WIDTH, contains, play_button_sound, play_menu,
    translated_label, wrap_text,
};
use mod_api::{
    ClientHost, Flow, ScreenKeyPressedPreEvent, ScreenMouseButtonPressedPreEvent,
    ScreenRenderPostEvent, Text,
};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// `ScreenView::id` of the stock pause menu.
pub(super) const SCREEN: &str = "paused";

/// A pause-menu button: its label, translation key ("" for the literal
/// label) and whether it works in this port.
struct Button {
    label: &'static str,
    key: &'static str,
    active: bool,
}

const BUTTONS: [Button; 6] = [
    Button {
        label: "Resume Game",
        key: "",
        active: true,
    },
    Button {
        label: "Help & Options",
        key: "",
        active: false,
    },
    Button {
        label: "Leaderboards",
        key: "legacy.menu.leaderboards",
        active: false,
    },
    Button {
        label: "Advancements",
        key: "gui.advancements",
        active: false,
    },
    Button {
        label: "Save Game",
        key: "legacy.menu.save",
        active: false,
    },
    Button {
        label: "Exit Game",
        key: "",
        active: true,
    },
];
const RESUME: usize = 0;
const EXIT: usize = 5;

/// What the next client tick does; screen hooks can't close screens.
const NO_REQUEST: usize = 0;
const REQUEST_RESUME: usize = 1;
const REQUEST_EXIT: usize = 2;

const PANEL: [f32; 2] = [230.0, 145.0];

#[derive(Default)]
pub(super) struct PauseMenu {
    focus: AtomicUsize,
    /// The exit confirmation is open over the list.
    confirming: AtomicBool,
    request: AtomicUsize,
}

pub(super) fn button_rect(size: [f32; 2], index: usize) -> [f32; 4] {
    [
        (size[0] / 2.0).floor() - 112.0,
        (size[1] / 3.0 + 5.0).floor() + index as f32 * (BUTTON_HEIGHT + BUTTON_GAP),
        BUTTON_WIDTH,
        BUTTON_HEIGHT,
    ]
}

fn panel_rect(size: [f32; 2]) -> [f32; 4] {
    [
        ((size[0] - PANEL[0]) / 2.0).floor(),
        ((size[1] - PANEL[1]) / 2.0).floor(),
        PANEL[0],
        PANEL[1],
    ]
}

/// Cancel (0) and Exit Game (1) on the confirmation panel.
pub(super) fn confirm_button_rect(size: [f32; 2], index: usize) -> [f32; 4] {
    let panel = panel_rect(size);
    [
        panel[0] + 15.0,
        panel[1] + PANEL[1] - 52.0 + index as f32 * 22.0,
        200.0,
        20.0,
    ]
}

/// The next working button from `from` in direction `step` (+1 or -1),
/// wrapping; disabled buttons are skipped as Legacy4J skips inactive widgets.
pub(super) fn step_focus(from: usize, down: bool) -> usize {
    let count = BUTTONS.len();
    let mut index = from.min(count - 1);
    for _ in 0..count {
        index = if down {
            (index + 1) % count
        } else {
            (index + count - 1) % count
        };
        if BUTTONS[index].active {
            return index;
        }
    }
    from
}

fn is_activate(key: &str) -> bool {
    matches!(
        key,
        "key.keyboard.enter" | "key.keyboard.keypad.enter" | "key.keyboard.space"
    )
}

impl PauseMenu {
    /// Back to the list with Resume Game focused, for the next time the menu opens.
    pub(super) fn reset(&self) {
        self.focus.store(RESUME, Ordering::Relaxed);
        self.confirming.store(false, Ordering::Relaxed);
    }

    /// Runs the request a click or key made, from `client_tick_pre`.
    pub(super) fn tick(&self, host: &mut dyn ClientHost) {
        match self.request.swap(NO_REQUEST, Ordering::Relaxed) {
            REQUEST_RESUME => {
                self.reset();
                host.close_screen();
            }
            REQUEST_EXIT => {
                self.reset();
                if let Err(error) = host.disconnect(None) {
                    eprintln!("Legacy Horizon: Exit Game could not leave the world: {error:?}");
                }
            }
            _ => {}
        }
    }

    fn press(&self, host: &dyn ClientHost, index: usize) {
        play_button_sound(host);
        if self.confirming.load(Ordering::Relaxed) {
            if index == 0 {
                self.confirming.store(false, Ordering::Relaxed);
                self.focus.store(EXIT, Ordering::Relaxed);
            } else {
                self.request.store(REQUEST_EXIT, Ordering::Relaxed);
            }
        } else if index == RESUME {
            self.request.store(REQUEST_RESUME, Ordering::Relaxed);
        } else if index == EXIT {
            self.confirming.store(true, Ordering::Relaxed);
            self.focus.store(0, Ordering::Relaxed);
        }
    }

    pub(super) fn draw(&self, host: &dyn ClientHost, event: &mut ScreenRenderPostEvent<'_>) {
        let _ = host.apply_screen_blur();
        let size = event.draw.size();
        let focus = self.focus.load(Ordering::Relaxed);
        if self.confirming.load(Ordering::Relaxed) {
            let panel = panel_rect(size);
            play_menu::bevel(event.draw, panel, false);
            event.draw.text(
                "Exit Game",
                [panel[0] + 15.0, panel[1] + 15.0],
                0xFF48_4848,
                false,
            );
            // The stock Disconnect saves a single-player world, so this is
            // Legacy4J's message for a world it doesn't save itself.
            let message = translated_label(
                host,
                "legacy.menu.server_exit_message",
                "Are you sure you want to exit to the main menu?",
            );
            for (index, line) in wrap_text(event, &message, 200.0).iter().enumerate() {
                event.draw.text(
                    line,
                    [panel[0] + 15.0, panel[1] + 35.0 + index as f32 * 12.0],
                    0xFF48_4848,
                    false,
                );
            }
            for (index, label) in ["Cancel", "Exit Game"].iter().enumerate() {
                let rect = confirm_button_rect(size, index);
                let key = if index == 0 { "gui.cancel" } else { "" };
                let label = translated_label(host, key, label);
                let _ = event.draw.widget_button(
                    rect,
                    &Text::literal(label),
                    true,
                    focus == index || contains(rect, event.screen.mouse),
                );
            }
            return;
        }
        for (index, button) in BUTTONS.iter().enumerate() {
            let rect = button_rect(size, index);
            let label = translated_label(host, button.key, button.label);
            let highlighted =
                button.active && (focus == index || contains(rect, event.screen.mouse));
            let _ =
                event
                    .draw
                    .widget_button(rect, &Text::literal(label), button.active, highlighted);
        }
    }

    /// Every click on the pause screen stops here, so an undrawn stock
    /// button can never run.
    pub(super) fn mouse_pressed(
        &self,
        host: &dyn ClientHost,
        event: &ScreenMouseButtonPressedPreEvent<'_>,
    ) -> Flow {
        if event.button != 0 {
            return Flow::Cancel;
        }
        let size = [event.screen.rect[2], event.screen.rect[3]];
        if self.confirming.load(Ordering::Relaxed) {
            if let Some(index) =
                (0..2).find(|&index| contains(confirm_button_rect(size, index), event.screen.mouse))
            {
                self.press(host, index);
            }
        } else if let Some(index) = (0..BUTTONS.len()).find(|&index| {
            BUTTONS[index].active && contains(button_rect(size, index), event.screen.mouse)
        }) {
            self.focus.store(index, Ordering::Relaxed);
            self.press(host, index);
        }
        Flow::Cancel
    }

    pub(super) fn key_pressed(
        &self,
        host: &dyn ClientHost,
        event: &ScreenKeyPressedPreEvent<'_>,
    ) -> Flow {
        let confirming = self.confirming.load(Ordering::Relaxed);
        let focus = self.focus.load(Ordering::Relaxed);
        match event.key {
            "key.keyboard.escape" => {
                if confirming {
                    self.confirming.store(false, Ordering::Relaxed);
                    self.focus.store(EXIT, Ordering::Relaxed);
                } else {
                    // Also reached from a gamepad's B or Start, which the
                    // stock screen never sees while this menu is drawn.
                    self.request.store(REQUEST_RESUME, Ordering::Relaxed);
                }
            }
            "key.keyboard.up" | "key.keyboard.down" => {
                let down = event.key == "key.keyboard.down";
                let next = if confirming {
                    usize::from(down)
                } else {
                    step_focus(focus, down)
                };
                self.focus.store(next, Ordering::Relaxed);
            }
            key if is_activate(key) => {
                if event.repeat {
                    return Flow::Cancel;
                }
                if confirming || BUTTONS[focus.min(BUTTONS.len() - 1)].active {
                    self.press(host, focus);
                }
            }
            // The stock screen's own focus moves would act on undrawn buttons.
            "key.keyboard.tab" | "key.keyboard.left" | "key.keyboard.right" => {}
            _ => return Flow::Continue,
        }
        Flow::Cancel
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buttons_follow_legacy4j_layout() {
        // 640x360 GUI: x = 320 - 112, y = 120 + 5, then 25 apart.
        assert_eq!(button_rect([640.0, 360.0], 0), [208.0, 125.0, 225.0, 20.0]);
        assert_eq!(button_rect([640.0, 360.0], 5), [208.0, 250.0, 225.0, 20.0]);
        assert_eq!(button_rect([427.0, 240.0], 0), [101.0, 85.0, 225.0, 20.0]);
    }

    #[test]
    fn only_resume_and_exit_work() {
        let active: Vec<_> = BUTTONS.iter().map(|b| b.active).collect();
        assert_eq!(active, [true, false, false, false, false, true]);
        assert_eq!(BUTTONS[RESUME].label, "Resume Game");
        assert_eq!(BUTTONS[EXIT].label, "Exit Game");
    }

    #[test]
    fn focus_skips_disabled_buttons_and_wraps() {
        assert_eq!(step_focus(RESUME, true), EXIT);
        assert_eq!(step_focus(EXIT, true), RESUME);
        assert_eq!(step_focus(RESUME, false), EXIT);
        assert_eq!(step_focus(EXIT, false), RESUME);
    }

    #[test]
    fn confirmation_buttons_sit_inside_the_panel() {
        let size = [640.0, 360.0];
        let panel = panel_rect(size);
        for index in 0..2 {
            let rect = confirm_button_rect(size, index);
            assert!(rect[0] >= panel[0] && rect[0] + rect[2] <= panel[0] + panel[2]);
            assert!(rect[1] >= panel[1] && rect[1] + rect[3] <= panel[1] + panel[3]);
        }
        assert_eq!(confirm_button_rect(size, 0), [220.0, 200.0, 200.0, 20.0]);
    }

    #[test]
    fn reopening_starts_on_resume_without_the_confirmation() {
        let menu = PauseMenu::default();
        menu.focus.store(EXIT, Ordering::Relaxed);
        menu.confirming.store(true, Ordering::Relaxed);
        menu.reset();
        assert_eq!(menu.focus.load(Ordering::Relaxed), RESUME);
        assert!(!menu.confirming.load(Ordering::Relaxed));
    }
}
