# mod API: let client mods apply validated vanilla option changes

## Port follow-up — 2026-11-20

The native route now supplies a working partial fallback for this port. A
HEAD helper at `Game::idle` consumes queued mouse X/Y inversion, discrete
scrolling, auto-jump and bobbing changes and calls `Game::options_control`
with `options-set`. This retains engine validation, application and normal
settings persistence. Controller inversion uses its typed boolean field and
the normal `Event::Changed` handling, which saves controller settings too.
Horizon MCP menu clicks changed the live engine values and normal settings
files. The original public-API request remains relevant; sliders, key-binding
editing and the rest of the custom settings UI are not finished.

> 🤖 Filed by Jacob0225's AI agent (Codex, GPT-5), not by Jacob0225 personally.

## Summary

`ClientHost` exposes read-only snapshots of input, graphics, GUI, chat and audio preferences, but a client mod cannot apply a changed vanilla option through the game.

This blocks functional ports of custom settings screens. Legacy4J's Legacy Console-style Settings, Controls and Audio pages can read and draw the current values, but sliders and toggles cannot change sensitivity, FOV, GUI scale, volume, subtitles, render distance and the other options they represent.

## Tested environment

- Horizon commit: `0b14f8acfd97fa2c750e42795249dcc981178ca4`
- Minecraft data: 26.3
- Mod SDK / shader API: 0.2 nightly from 2026-11-18
- Native Rust `.hmod`, built by `horizon-launcher native-build`

## Current API

`ClientHost` provides setup-safe reads:

- `input_options() -> ClientInputOptions`
- `graphics_options() -> ClientGraphicsOptions`
- `gui_options() -> ClientGuiOptions`
- `chat_options() -> ClientChatOptions`
- `audio_options() -> ClientAudioOptions`

The snapshot types explicitly document that mods only read them. There is no corresponding validated write surface. Registered mod key mappings have their own persistence support, but that does not change vanilla preferences.

## Reproduction

1. Draw a mod-owned settings screen with an FOV slider and a Subtitles toggle.
2. Initialize them from `host.graphics_options().fov` and `host.gui_options().show_subtitles`.
3. On activation, try to apply and persist the new values through `ClientHost`.

There is no host operation for step 3.

## Expected

A client mod can request a validated change to a supported vanilla client option. The API could use typed option/value enums or focused methods; it should not require replacing an entire stale snapshot. For example:

```rust
host.set_client_option(ClientOption::Fov, ClientOptionValue::Float(82.0))?;
host.set_client_option(ClientOption::ShowSubtitles, ClientOptionValue::Bool(true))?;
```

The game should clamp or refuse invalid values, apply the same side effects as its own Options screen and save through its normal settings path.

## Actual

The Legacy-style screen can only be decorative, or it must send the player to Horizon's built-in Options screen and lose GUI parity. Editing `options.txt` directly is not a live-setting API: it bypasses validation and side effects, races the game's in-memory settings and may be overwritten on save.

## Existing issue check

- #9581/#9601 added the client option snapshots and persisted **mod key-binding** changes, but the released surface remains read-only for vanilla options.
- #3648/#3677 fixed navigation to a mod's own settings page from Horizon's Options screen; it did not add vanilla option setters.
- Agent control's `options-set` command can apply these options internally, but it is not exposed to mods.
- No exact issue for validated client-option writes from the mod API was found.

## Acceptance criteria

- A client mod can set the supported fields represented by the input, graphics, GUI, chat and audio option snapshots.
- Values are type checked and range/choice validated; unsupported or restart-only settings return a typed refusal.
- Changes trigger the same immediate effects as Horizon's own Options UI, including GUI rescale, renderer/view-distance refresh, fullscreen/VSync handling and audio updates where applicable.
- Accepted changes persist through Horizon's normal settings save path.
- A mod cannot replace a whole stale snapshot and unintentionally overwrite unrelated changes.
- Reads reflect an accepted change without restarting.
- The surface is client-only and works from a mod-owned title or in-world settings screen.
