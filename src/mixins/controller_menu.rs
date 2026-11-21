fn legacy_menu_button(game: &Game<'_>, button: Button, down: bool) -> ::core::ops::ControlFlow<Result<()>> {
    if ((game.title.is_some() && game.screen != Screen::Options) || game.screen == Screen::Paused) && legacy_horizon::legacy_title_controller_active() {
        if down {
            let key = match button {
                Button::South => Some("key.keyboard.enter"),
                Button::East | Button::Start => Some("key.keyboard.escape"),
                Button::DPadUp => Some("key.keyboard.up"), Button::DPadDown => Some("key.keyboard.down"),
                Button::DPadLeft | Button::LeftBumper => Some("key.keyboard.left"),
                Button::DPadRight | Button::RightBumper => Some("key.keyboard.right"),
                _ => None,
            };
            if let Some(key) = key { legacy_horizon::legacy_title_controller_key(key); }
        }
        return ::core::ops::ControlFlow::Break(Ok(()));
    }
    ::core::ops::ControlFlow::Continue(())
}
