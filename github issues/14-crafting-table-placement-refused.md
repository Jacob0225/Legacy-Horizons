# mod API: place_recipe refuses an open vanilla crafting table from client_tick_pre

> 🤖 Filed by Jacob0225's AI agent (Codex, GPT-6), not by Jacob0225 personally.

## Summary

A native client mod calls `ClientHost::place_recipe` from `client_tick_pre` while the vanilla crafting-table screen is visibly open. Horizon refuses it with:

```text
Legacy Horizon: place_recipe refused minecraft:acacia_planks: no screen that takes recipes is open
```

The screen render/input hooks identify that same screen as `id = "inventory"`, `menu = Some("minecraft:crafting")`. The mod does not cancel the stock crafting screen or replace its live slots.

## Tested environment

- Horizon commit: `0b14f8acfd97fa2c750e42795249dcc981178ca4`
- Minecraft data 26.3; Mod API 0.2
- Linux, native client .hmod, built with horizon-launcher native-build
- Observed build: `native-0b14f8a-a8dec384a48b`
- Integrated test world, survival player; all recipes previously unlocked with `recipe give @s *`

This report is verified on that pinned nightly; current main has not been retested.

## Reproduction

1. Subscribe to screen_render_post and screen_mouse_button_pressed_pre. Leave the stock crafting table screen rendering and input active.
2. Open a vanilla crafting table. Confirm the screen hook reports `inventory` and `minecraft:crafting`.
3. In the mouse hook, queue the selected recipe id; do not call a mutable host service from the read-only input hook.
4. In client_tick_pre, resolve `host.recipe_book_ids("minecraft:acacia_planks")`. It returns at least one id.
5. Call `host.place_recipe(book_ids[0], false)` and log the Result.

## Actual

The Result is Err("no screen that takes recipes is open"). The table remains visible and its crafting grid does not change. The screen is not closed or replaced by the mod between the click and this tick.

## Expected

The service recognizes the currently open vanilla crafting table. Missing ingredients may produce a ghost/refusal appropriate to the recipe, but should not classify the open table as absent. A recipe with available ingredients should take the normal recipe-book placement path.

## Source investigation

In the pinned source, `recipe_place` first tests `mod_recipe_screen()`; that helper tests `self.screen == Screen::Inventory`, the active crafting book/custom crafting role, and spectator state. The exact first refusal above is observed. I have not isolated which predicate differs in the mutable client-tick host context, so this is evidence of the boundary failure, not a claimed root cause.

## Existing issue check

Searched open and closed issues for place_recipe and the exact refusal string. Inspected #10864 (audit infrastructure failure) and #12009 (furnace packet placement adapter); neither describes this native client-host table refusal.

The separate slot-layout request #18397 affects exact LegacyCraftingScreen geometry, not this placement call.

## Acceptance criteria

- The mutable client-tick host recognizes an open vanilla crafting table consistently with its screen hooks.
- A valid known display id can place available ingredients using place_recipe.
- Personal inventory, table and custom crafting menus retain their intended checks.
- Missing ingredients and stale/unknown recipe ids retain correct behavior.

