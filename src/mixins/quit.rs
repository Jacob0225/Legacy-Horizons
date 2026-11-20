fn legacy_idle(game: &mut Game<'_>, event_loop: &dyn EventLoopCtl) {
    while let Some((key, value)) = legacy_horizon::take_client_option_change() {
        let line = format!("options-set {key} {value}");
        if let Err(error) = game.options_control(&["options-set", key], &line) {
            eprintln!("Legacy Horizon: could not apply {key}: {error}");
        }
    }
    if legacy_horizon::legacy_controller_take_invert_request() {
        game.settings.controller.invert_y = !game.settings.controller.invert_y;
        if let Err(error) = game.handle_option_events(vec![horizon_client::options_ui::Event::Changed]) {
            eprintln!("Legacy Horizon: could not apply controller inversion: {error}");
        }
    }
    legacy_horizon::legacy_controller_report_inverted_look(game.settings.controller.invert_y);
    if legacy_horizon::take_quit_request() {
        event_loop.exit();
    }
}
