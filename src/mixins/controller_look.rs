fn legacy_pad_look(game: &mut Game<'_>, seconds: f32) {
    let settings = &game.settings.controller;
    if !settings.enabled || game.pad_in_menu() || game.player.is_none()
        || !game.pad_focused()
        || (game.pad_radial_captures() && game.pad_radial_selects(1)) {
        return;
    }
    let [mut yaw, mut pitch] = legacy_horizon::legacy_controller_look(
        game.pad.state.right, settings.look_sensitivity, seconds,
        game.camera_type.is_first_person() && game.scoping(),
    );
    if settings.invert_x { yaw = -yaw; }
    if settings.invert_y { pitch = -pitch; }
    if yaw != 0.0 || pitch != 0.0 { game.look_turn(yaw, pitch); }
}
