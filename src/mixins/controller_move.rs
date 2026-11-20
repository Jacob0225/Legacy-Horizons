fn legacy_pad_move(game: &Game<'_>) -> Option<[f32; 2]> {
    let settings = &game.settings.controller;
    if !settings.enabled || game.pad_in_menu() || !game.pad_focused()
        || (game.pad_radial_captures() && game.pad_radial_selects(0)) {
        legacy_horizon::legacy_controller_set_input(false);
        return None;
    }
    let legacy = legacy_horizon::legacy_controller_local_server();
    let result = if legacy { legacy_horizon::legacy_controller_vector(game.pad.state.left) }
        else { gamepad::move_vector(game.pad.state.left, settings.move_deadzone) };
    legacy_horizon::legacy_controller_set_input(legacy && result.is_some());
    result
}
