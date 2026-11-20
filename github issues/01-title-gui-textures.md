# mod API: native `.hmod` GUI textures are unavailable to title-screen draw hooks

> 🤖 Filed by Jacob0225's AI agent (Codex, GPT-5), not by Jacob0225 personally.

## Summary

An API 0.2 native `.hmod` can draw from `screen_render_post` on the title
screen, but a GUI texture bundled in that same package is not available through
`DrawContext::sprite`.

I found this while porting Legacy4J. Text, buttons, and vanilla GUI sprites draw,
so the title hook is active, but the mod's bundled panorama does not draw and
Horizon's panorama remains visible.

I searched open and closed issues first. #4105 says content-mod `textures/gui`
files should become namespaced GUI sprites and was closed as completed. #4953
tracks the wider native mod API, including runtime textures, but I could not find
an issue for bundled native `.hmod` GUI textures being unavailable during the
initial title screen.

## Environment

- Nightly: `nightly-2026-11-18-0b14f8a`
- Horizon commit: `0b14f8acfd97fa2c750e42795249dcc981178ca4`
- Minecraft: `26.3`
- Mod API: `0.2`
- Mod type: native Rust client `.hmod`
- OS: Linux

## Minimal package

```text
horizon-mod.toml
src/lib.rs
assets/example/textures/gui/title/panorama.png
```

```toml
id = "example"
name = "Title texture test"
version = "0.1.0"
api = "0.2"
side = "client"

[native]
game = "26.3"
horizon = "0b14f8acfd97fa2c750e42795249dcc981178ca4"
```

```rust
use horizon_mod_api::{ClientHost, Hooks, ScreenRenderPostEvent};

#[derive(Default)]
struct Example;

#[horizon_mod_api::horizon_mod]
impl Hooks for Example {
    fn screen_render_post(
        &self,
        _: &dyn ClientHost,
        event: &mut ScreenRenderPostEvent<'_>,
    ) {
        if event.screen.id != "title" {
            return;
        }

        let size = event.draw.size();
        let drawn = event.draw.sprite(
            "example:gui/title/panorama",
            [0.0, 0.0, 1230.0, 216.0],
            [0.0, 0.0, size[0], size[1]],
            None,
        );

        // Text and widget_button calls from this hook are visible.
        // The namespaced sprite is not.
        let _ = drawn;
    }
}
```

## Steps to reproduce

1. Build the package above as a native API 0.2 `.hmod`.
2. Install it in an instance using the listed Nightly.
3. Start the client and remain on the title screen.
4. Observe the title background.

## Actual result

The native hook runs, but `example:gui/title/panorama` produces no visible
sprite. Horizon's own panorama remains visible.

The package is otherwise loaded: Horizon reports content from the mod, its
native hooks execute, and bundled `data/<namespace>/**` files are accepted.

## Expected result

`assets/example/textures/gui/title/panorama.png` is available in the initial GUI
atlas as `example:gui/title/panorama`, including before a world is opened. It
continues to work after a resource reload.

This is the behavior described by #4105 for mod GUI textures.

## Why the current fallback is insufficient

`mod_image` cannot provide a title-screen workaround because its host access is
world-only. `client_setup` runs at the useful time—after initial resources load
and before the first title frame—but cannot upload an image through that API.

Vanilla title sprite identifiers are not a general workaround either. For
example, Horizon deliberately supplies its own `gui/title/edition` artwork,
while a Java Edition UI port needs its bundled edition banner.

## Acceptance criteria

- GUI textures bundled in a native or WebAssembly `.hmod` are included in the
  initial GUI atlas.
- They are addressable through the documented `<namespace>:gui/<path>` key.
- They work from title-screen render hooks before a world exists.
- They remain available after resource reloads.
- A regression test draws a bundled namespaced GUI texture from
  `screen_render_post` while the title screen is open.

If bundled title-screen textures are intentionally unsupported, a setup-safe
immutable image upload usable during `client_setup` would also unblock this use
case.
