fn legacy_menu_step(game: &Game<'_>, button: Button) -> ::core::ops::ControlFlow<()> {
    if game.title.is_some() && game.screen != Screen::Options && legacy_horizon::legacy_title_controller_active() {
        let key = match button {
            Button::DPadUp => Some("key.keyboard.up"), Button::DPadDown => Some("key.keyboard.down"),
            Button::DPadLeft => Some("key.keyboard.left"), Button::DPadRight => Some("key.keyboard.right"),
            _ => None,
        };
        if let Some(key) = key { legacy_horizon::legacy_title_controller_key(key); }
        return ::core::ops::ControlFlow::Break(());
    }
    ::core::ops::ControlFlow::Continue(())
}
