# mod API: let client mods replace vanilla container screen slot layouts

> 🤖 Filed by Jacob0225's AI agent (Codex, GPT-5), not by Jacob0225 personally.

## Summary

A client mod can inspect the rectangles of an open menu's slots, draw over the screen and reserve room beside its window, but it cannot replace the positions, visibility or hitboxes of slots on an existing vanilla container screen.

This blocks GUI ports that preserve vanilla menu semantics while changing their presentation. Legacy4J replaces the inventory, crafting table, furnace-family, merchant, loom and stonecutter layouts; drawing the replacement artwork alone leaves Horizon's items and click targets at the vanilla coordinates.

## Tested environment

- Horizon commit: `0b14f8acfd97fa2c750e42795249dcc981178ca4`
- Minecraft data: 26.3
- Mod SDK / shader API: 0.2 nightly from 2026-11-18
- Native Rust `.hmod`, built by `horizon-launcher native-build`

## Current API

- `MenuRef::slot_boxes(host)` returns the open screen's slot rectangles in menu order. It is read-only.
- `MenuRef::slot_kinds(host)` identifies broad slot roles.
- `screen_room` can reserve a panel to the right and shift the entire vanilla window when it fits.
- Screen hooks can draw replacement backgrounds and widgets.

There is no public surface for changing an existing vanilla menu slot's screen rectangle or hiding it for the lifetime of that screen. The click and hover hitboxes therefore remain tied to Horizon's own layout.

## Reproduction

1. Open a vanilla crafting table, furnace or player inventory.
2. In a client mod, draw a replacement container background with the input, result and player-inventory slots at different coordinates.
3. Try to move the corresponding live menu slots and their click/hover hitboxes onto those coordinates.

The mod can read the old boxes but cannot supply replacement boxes. Items, hover highlights and mouse clicks remain at the vanilla positions.

## Expected

A client-only layout hook can return replacement rectangles for selected slots of the currently open menu, for example:

```rust
fn menu_slot_layout(&self, host: &dyn ClientHost, event: &mut MenuSlotLayoutEvent<'_>) {
    event.set_box(0, [input_x, input_y, 16.0, 16.0]);
    event.set_box(1, [result_x, result_y, 16.0, 16.0]);
}
```

The override should affect rendering, hover detection, tooltips and normal clicks together, while retaining the original menu slot indices, server rules and packet semantics. Omitting a slot should keep its native box; an explicit hidden value should hide and disable its hitbox.

## Actual

The mod can reproduce the frame and labels, but it cannot align the live items and interactions with that frame. Intercepting mouse events and synthesizing clicks would duplicate Horizon's menu behavior and is not an adequate layout API.

## Existing issue check

- #9606 asks for client menu interaction such as clicking the current slot/cursor; it does not provide screen layout or hitbox overrides.
- #4070 concerned declaring and opening mod-owned container menus, not changing an existing vanilla screen's layout.
- `screen_room` reserves adjacent space and shifts the whole window; it cannot independently place slots.
- No exact issue for replacing vanilla menu slot rectangles was found.

## Acceptance criteria

- A client mod can override the GUI rectangle of any slot in the currently open vanilla menu by menu-slot index.
- Rendering, hover state, tooltip targeting, dragging and normal click handling all use the same overridden rectangle.
- A slot can be explicitly hidden, removing both its item draw and hitbox.
- Unspecified slots keep Horizon's native layout.
- Overrides are scoped to the current screen/menu and disappear when it closes or changes.
- The API does not change backing storage, acceptance rules, slot indices or server packet semantics.
- Invalid rectangles or stale/foreign menus receive a typed refusal and cannot panic the client.
