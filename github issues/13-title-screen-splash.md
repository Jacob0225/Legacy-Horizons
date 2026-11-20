# mod API: expose the active title-screen splash to replacement render hooks

> 🤖 Filed by Jacob0225's AI agent (Codex, GPT-5), not by Jacob0225 personally.

## Summary

An API 0.2 client mod can replace the title screen through
`screen_render_pre`/`screen_render_post`, but it cannot read the splash text
that Horizon already selected for that title-screen session.

This blocks exact Legacy4J title-screen parity. The replacement can draw the
logo, edition banner, panorama and buttons, but must either omit the rotating
yellow splash or independently choose a different line that can disagree with
Horizon's resource-pack-aware splash pool and special-date behavior.

## Tested environment

- Horizon commit: `0b14f8acfd97fa2c750e42795249dcc981178ca4`
- Minecraft data: 26.3
- Mod SDK / shader API: 0.2 nightly from 2026-11-18
- Native Rust `.hmod`, built by `horizon-launcher native-build`

## Current API

`ScreenRenderPostEvent` provides the screen id, bounds, pointer position and a
`DrawContext`. `ClientHost` provides translation and other client services, but
neither exposes the title screen's selected splash.

Horizon already builds the active splash pool from vanilla, content mods and
enabled resource packs and stores one selected splash on its title state. That
selection also accounts for special-date splashes. Reimplementing that logic in
each native mod would duplicate Horizon internals and still could not see the
user's enabled resource overlays through the public API.

## Reproduction

1. Cancel the stock title draw from `screen_render_pre`.
2. Recreate the title in `screen_render_post` using `event.draw`.
3. Attempt to draw the same yellow splash that the stock title would have
   rendered.

There is no public value or host method that returns it.

## Expected

Expose a read-only optional title splash for title-screen hooks, for example:

```rust
fn title_splash(&self) -> Option<String>;
```

Returning the already-selected string is preferable to exposing the whole pool:
it preserves Horizon's choice, resource-pack precedence, special dates and
session stability without giving a mod filesystem access.

The value may be `None` outside the title screen or while splashes are disabled.

## Actual

A replacement title renderer must omit the splash, hard-code one, or choose
from a bundled list that may not match the active resources or Horizon's current
selection.

## Existing issue check

- Searched open and closed `horizon-rust/horizon` issues for `mod API title
  splash` and open issues for `resource text native mod API`; no exact request
  was returned.
- The existing bundled-GUI-texture request is separate: the vanilla logo and
  edition sprites can already be drawn, but no sprite provides the dynamic
  splash string.

## Acceptance criteria

- A title-screen render hook can obtain the exact splash Horizon selected for
  the current title session.
- The value reflects vanilla/content-mod/resource-pack splash merging and
  special-date overrides exactly as the stock title does.
- The selected value remains stable for the life of that title instance and
  updates when Horizon creates a new title state.
- The API is read-only and does not expose resource-pack filesystem paths.
- Native and WebAssembly client mods receive equivalent behavior.
