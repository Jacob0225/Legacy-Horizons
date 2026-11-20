# Display Hand validation

Source: Legacy4J `GameRendererMixin.renderLevel`, conditional call to
`renderItemInHand` using `LegacyOptions.displayHand` (default on).

Target: Horizon `0b14f8acfd97fa2c750e42795249dcc981178ca4`, Minecraft 26.3,
Mod API 0.2. Native build `native-0b14f8a-9709c59f9ba2`; 14 tests pass.

Horizon MCP, disposable `legacy-horizon-parity-test` world:

1. Selected empty hotbar slot 9 and looked at yaw 0, pitch 10 in daylight.
   `gameplay-hand-on.png` shows the normal first-person arm.
2. Returned to title, opened Help & Options → Settings → User Interface and
   clicked Display Hand. The scoped config saved `display_hand: false`.
3. Reopened the same world and view. `gameplay-hand-off.png` shows no arm;
   world geometry and HUD remain visible.
4. Selected slot 1 (five acacia planks). `gameplay-held-item-off.png` shows
   the item in the hotbar without its first-person geometry.
5. Restarted the client. The menu restored `Display Hand: OFF` before world
   entry. Switched it back ON after testing.

These are actual game captures, not edited mockups. The backgrounds differ
slightly in cloud position because game time advances between captures.
