# mod API: expose the closing world-save screen to client mods

> 🤖 Filed by Jacob0225's AI agent (Codex, GPT-5), not by Jacob0225 personally.

## Summary

Horizon renders the closing world-save presentation through a private frame path that fires none of the screen, GUI or frame hooks. A client mod therefore cannot restyle the **Saving world...** screen shown while leaving a local world or quitting the client.

This blocks GUI parity for Legacy4J. Its save presentation uses the Legacy Console loading layout, translated preparing/finalizing stages and its own progress treatment. The port can replace registered dimension-transition screens, but the closing save is a separate path with no public draw surface.

## Tested environment

- Horizon commit: `0b14f8acfd97fa2c750e42795249dcc981178ca4`
- Minecraft data: 26.3
- Mod SDK / shader API: 0.2 nightly from 2026-11-18
- Native Rust `.hmod`, built by `horizon-launcher native-build`

## Current API

- Ordinary screens can be observed or replaced through `screen_render_pre` and the other screen render stages.
- `gui_render_post` explicitly excludes file/world loading and resource-reload overlays.
- `flip_frame` documents that the closing save screen and startup loading screens present frames of their own and fire none of the frame hooks.
- `register_dimension_transition_screen` and `dimension_transition_screen_draw` cover level transitions only.
- Horizon internally assigns the save presentation the stable widget screen id `saving`, computes coarse progress for chunks/entities/writes, and draws it through `render_closing_save`, but none of that reaches a mod.

## Reproduction

1. Open a local world with a client mod listening to every screen render stage, `gui_render_post` and `flip_frame`.
2. Leave the world or request a normal client quit.
3. Observe Horizon's **Saving world...** progress screen while chunk snapshots and writes finish.
4. Try to suppress that drawing and render a replacement.

None of the public draw hooks fires for that frame. The mod cannot identify the save stage, read its progress or draw the replacement presentation.

## Expected

Expose a dedicated, read-only closing-save draw hook, for example:

```rust
fn closing_save_screen_draw(
    &self,
    host: &dyn ClientHost,
    event: &mut ClosingSaveScreenDrawEvent<'_>,
)
```

The event should provide the GUI draw context and bounded presentation state such as:

- whether the operation will close the world, quit the client, or is an explicit save;
- a stable stage (`generation`, `chunks`, `entities`, `writes`, `server`, `quit`);
- normalized progress where Horizon can calculate it, or `None` otherwise.

A listener that owns the selected presentation should draw the complete frame, as `dimension_transition_screen_draw` does. Saving, durability, timeout and shutdown behavior must remain entirely Horizon-owned.

## Actual

The Legacy4J port must leave Horizon's closing save screen unchanged. Drawing from the last ordinary world frame is not sufficient: Horizon freezes simulation, pumps window events and presents separate loading frames until durable writes finish.

## Existing issue check

- #18389 requests the graceful quit action a replacement title button needs; it does not expose the save screen that action enters.
- #15868 requests background resource reload and resource-loading-overlay control, a separate reload lifecycle.
- #16674 lists resource-reload overlay gaps for an RRLS port, not the closing world-save frame.
- #4953 and #6452 are broad native-mod API trackers and do not define this draw contract.
- No exact issue for drawing or replacing Horizon's closing world-save screen was found through GitHub issue search.

## Acceptance criteria

- The hook fires once for each closing-save frame that Horizon presents, including leave-world and quit saves.
- It receives a GUI-pixel `DrawContext`, stable operation/stage values and optional normalized progress.
- A selected replacement can draw the complete frame without double-drawing Horizon's normal save presentation.
- The hook cannot shorten, skip, cancel or report completion for the underlying save.
- Window close and control-socket save paths retain their current durable-write guarantees.
- With no listener, the current save path, allocations and rendered output remain unchanged.
