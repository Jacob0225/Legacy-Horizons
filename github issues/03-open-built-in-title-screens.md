# mod API: let client mods open built-in title and options screens

> 🤖 Filed by Jacob0225's AI agent (Codex, GPT-5), not by Jacob0225 personally.

## Summary

A client mod replacing or extending the title menu can observe, draw, and consume input on the title screen, but it cannot ask Horizon to open one of Horizon's built-in screens. In particular, there is no public mod API equivalent of opening Select World, Multiplayer, the main Options screen, or a specific Options page such as Controls.

This blocks a Legacy4J port from wiring its visually ported `Play Game`, `Controls`, and `Settings` buttons to Horizon's real functionality.

## Tested environment

- Horizon commit: `0b14f8acfd97fa2c750e42795249dcc981178ca4`
- Minecraft data: 26.3
- Mod SDK / shader API: 0.2 nightly from 2026-11-18
- Native Rust `.hmod`, built by `horizon-launcher native-build`

## Current API

`ClientHost` exposes:

- `mod_screen_open`, which is `WorldOnly` and explicitly returns false over the title screen;
- `open_inventory`, which is also `WorldOnly`;
- `close_screen`, which does not open a destination screen.

There is no host method for the title's built-in destinations or Options pages. Horizon itself already has internal routes such as `Game::open_options_at(from_title, page)`, and agent control can open those same pages, but a mod cannot request them.

## Reproduction

1. In a native client mod, replace or add a button to the title menu using the title screen render and input hooks.
2. Add a `Controls` or `Settings` button.
3. Try to make the button open Horizon's Controls page or main Options page through `ClientHost`.

There is no API call that can perform step 3. Calling `mod_screen_open` is not an equivalent and returns false on the title screen.

## Expected

A narrowly scoped client API can request safe built-in navigation, including from the title screen. For example, the exact shape could be an enum rather than stringly typed names:

```rust
host.open_builtin_screen(BuiltinScreen::Options(OptionsPage::Controls));
host.open_builtin_screen(BuiltinScreen::WorldSelect);
```

The call should return `false` when the requested transition is unavailable and otherwise follow the same screen-opening/closing hooks and return behavior as Horizon's own title buttons.

## Actual

The mod can render a pixel-matching button and consume its input, but it must either leave the button inert or reproduce Horizon's screen inside the mod. Reproducing Select World or Options would duplicate game-owned behavior and drift from Horizon.

## Why this is separate from existing issues

- #3648, fixed by #3677, covered Horizon's own **Options → Mods → Settings** route doing nothing at the title screen.
- That fix did not add a public `ClientHost` navigation method for a mod-owned title menu.
- The separate mod-screen limitation is covered by the companion report in this port, but opening a built-in Horizon screen should not require opening a mod-owned screen.

## Acceptance criteria

- A native client mod can request the main Options screen and a supported Options page from the title screen.
- A native client mod can request the title's world-select and multiplayer screens.
- Navigation uses Horizon's existing screens rather than a mod copy.
- Opening and closing hooks observe the transition normally.
- The API refuses invalid contexts without panicking.

