# mod API: allow a client mod to open its own screen from the title screen

> 🤖 Filed by Jacob0225's AI agent (Codex, GPT-5), not by Jacob0225 personally.

## Summary

A client mod can draw buttons over the title screen and receive their input,
but it cannot open one of its own screens from those buttons.

`ClientHost::mod_screen_open` is world-only and explicitly returns `false` over
the title screen. This prevents a port from implementing title-menu entries
whose destinations belong to the mod. I encountered it while porting Legacy4J:
its title menu has Leaderboards and Minecraft Store entries, both of which need
mod-owned screens before a world is open.

I searched open and closed issues first. #3648 covered Horizon's built-in
Options > Mods > Settings button failing at the title screen and was fixed by
#3660/#3677. That internal route now works, but the public mod API still
documents and implements title-screen refusal. I could not find an issue for
opening a mod's own screen directly from a title-screen hook.

## Environment

- Nightly: `nightly-2026-11-18-0b14f8a`
- Horizon commit: `0b14f8acfd97fa2c750e42795249dcc981178ca4`
- Minecraft: `26.3`
- Mod API: `0.2`
- Mod type: native Rust client `.hmod`
- OS: Linux

## Minimal reproduction

The mouse hook has a read-only host, so this example records the click and asks
to open the screen from the next client tick, where the host is mutable.

```rust
use horizon_mod_api::{
    ClientHost, ClientTickPreEvent, Flow, Hooks,
    ScreenMouseButtonPressedPreEvent,
};
use std::sync::atomic::{AtomicBool, Ordering};

const ID: &str = "example";

#[derive(Default)]
struct Example {
    open_requested: AtomicBool,
}

#[horizon_mod_api::horizon_mod]
impl Hooks for Example {
    fn screen_mouse_button_pressed_pre(
        &self,
        _: &dyn ClientHost,
        event: &mut ScreenMouseButtonPressedPreEvent<'_>,
    ) -> Flow {
        if event.screen.id == "title" && event.button == 0 {
            self.open_requested.store(true, Ordering::Relaxed);
            return Flow::Cancel;
        }
        Flow::Continue
    }

    fn client_tick_pre(
        &self,
        host: &mut dyn ClientHost,
        _: &mut ClientTickPreEvent,
    ) {
        if self.open_requested.swap(false, Ordering::Relaxed) {
            assert!(!host.mod_screen_open(ID));
        }
    }
}
```

## Steps to reproduce

1. Install the native client mod above.
2. Start the client and remain on the title screen.
3. Click while the title screen is open.
4. Observe the result of `mod_screen_open`.

## Actual result

`mod_screen_open` returns `false` and the title screen remains open.

The API documentation currently describes this result: the method opens a mod
screen only over a world and returns false over the title screen. Its access is
also marked `WorldOnly`.

The title-screen render and input hooks therefore permit a mod to draw an
interactive-looking menu entry without providing any supported route to its
destination.

## Expected result

A client mod can open its own screen from a title-menu button before a world is
loaded. Closing that screen with Escape, a Done button, or
`mod_screen_close(ID)` returns to the title screen and its previous mode.

This could be implemented by allowing `mod_screen_open` in `ClientSetup` while
the title is active, or by adding a separate title-safe method if the screen
lifecycle needs different semantics.

## Why existing hooks are insufficient

- `screen_render_post` can draw the button but cannot change screens.
- The mouse and key Pre hooks can consume input but receive a read-only host.
- Deferring to `client_tick_pre` provides a mutable host, but
  `mod_screen_open` still refuses because no world exists.
- `screen_opening` can react to a screen transition that Horizon has already
  started; it cannot initiate a mod screen from the custom button.
- Leaving the input uncancelled is not a safe workaround: the mod's replacement
  layout does not share Horizon's hidden title-button rectangles or actions.

## Acceptance criteria

- A client mod can open its own screen while any title mode is active.
- The screen's render, mouse, scroll, key, and character hooks work as they do
  for a mod screen opened over a world.
- Escape, Done, and `mod_screen_close` return to the title mode that opened it.
- No world or player handle is required.
- One mod cannot close or replace another mod's screen.
- A regression test opens a mod screen from the main title mode, interacts with
  it, closes it, and verifies that the title screen resumes.
