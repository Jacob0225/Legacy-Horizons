# Audio and settings validation

Target: Horizon `0b14f8acfd97fa2c750e42795249dcc981178ca4`, Minecraft 26.3, API 0.2.

- `cargo test --locked`: 14 passing tests, including independent cave/cart
  suppression, preserving other ambience, and the source elytra volume/pitch rule.
- Built and installed into the disposable `legacy-horizon-gui-test` instance.
- Horizon MCP exercised title → Help & Options → Settings → Audio. Clicking
  Cave Sounds and Minecart Sounds changed each from ON to OFF.
- Both OFF values were written to the scoped config from the title screen.
- A fresh launch of the final native build restored both OFF values before
  opening a world. Both were switched back ON after the persistence check.
- Opened the disposable world and sent cave/elytra sound commands successfully
  through Horizon MCP. This checks integration, not audible output.
- Setting test master volume to 0.35 through Horizon MCP changed the displayed
  value to `Sound: 35%`. Restored it to 1.0 after capturing evidence.
- `gui-audio-progress.png` is an actual game capture of those values.
- The isolated headless display has no audio output. This validates menu
  integration and the tested sound transformation, not an audible listening test.

Source research: Legacy4J's `BiomeAmbientSoundsHandlerMixin`,
`MinecartSoundInstanceMixin`, `RidingEntitySoundInstanceMixin` and
`ElytraOnPlayerSoundInstanceMixin`. The flight capability finding is recorded
in `github issues/15-client-creative-flight-policy.md` and upstream #18811.
