# mod API: expose active key/controller glyphs for mod-drawn screens

> 🤖 Filed by Jacob0225's AI agent (Codex, GPT-5), not by Jacob0225 personally.

## Summary

A client mod can receive named keyboard, mouse and controller input, but it cannot draw the same contextual input glyphs Horizon's own screens draw for the active binding and controller family.

This blocks GUI parity for ports such as Legacy4J. Its How To Play pages and control tooltips embed actions such as movement, look, jump, attack, use and drop as keyboard/mouse/controller glyphs. Without a public glyph surface, the port can only show placeholder squares or hard-code one controller's artwork.

## Tested environment

- Horizon commit: `0b14f8acfd97fa2c750e42795249dcc981178ca4`
- Minecraft data: 26.3
- Mod SDK / shader API: 0.2 nightly from 2026-11-18
- Native Rust `.hmod`, built by `horizon-launcher native-build`

## Current API

- Input hooks expose vanilla key names and button numbers.
- `ClientHost::input_options` exposes preference snapshots.
- Mod key mappings can be registered/read.
- `DrawContext` can draw text, sprites and widgets.

There is no public operation that resolves an action or binding to Horizon's current keyboard label or controller-family glyph, nor one that draws that glyph with the dimensions Horizon uses internally.

Horizon already has this behavior internally: its controller documentation describes each screen's declared actions and the glyph sprite/rectangle drawn for the active pad family.

## Reproduction

1. Draw a mod-owned help or controls screen.
2. Add an instruction for the player's current Jump, Attack or Use binding.
3. Try to show the correct visual for keyboard/mouse, Xbox, PlayStation, Switch or another active controller family.

The mod cannot ask Horizon which glyph should be shown or draw the engine's resolved glyph.

## Expected

A read/draw surface resolves the current input presentation without exposing Horizon's internal atlas. For example:

```rust
let glyph = host.input_glyph(InputAction::Jump)?;
draw.input_glyph(&glyph, at, tint)?;
```

It should also support a concrete registered mapping or physical input when the screen is documenting something other than a vanilla action.

## Actual

The port displays real localized Legacy4J help text, but every `%s` control icon must currently become a placeholder square. Bundling static glyph sheets would still be wrong when the player changes bindings, input method or controller family.

## Existing issue check

- Horizon's controller documentation covers the engine's internal glyph system, not a mod API.
- The native-mod API coverage tracker #4953 does not list a public input-glyph resolver/drawer.
- No exact issue for mod-drawn key/controller glyphs was found.

## Acceptance criteria

- A client mod can resolve and draw the current glyph for a vanilla action such as jump, attack, use, movement or inventory.
- A client mod can resolve a registered mod mapping or a concrete physical key/button.
- The result follows the active keyboard/controller input mode and controller family.
- Rebinding is reflected without restarting.
- Drawing uses Horizon's normal glyph atlas, size and tint behavior.
- Unknown or unbound inputs return a typed absence/refusal instead of a placeholder or panic.

