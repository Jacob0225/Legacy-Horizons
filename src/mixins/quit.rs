fn legacy_quit(event_loop: &dyn EventLoopCtl) {
    if legacy_horizon::take_quit_request() {
        event_loop.exit();
    }
}
