# mod API: collision-aware local player movement and creative flight policy

> 🤖 Filed by Jacob0225's AI agent (Codex, GPT-6), not by Jacob0225 personally.

Submitted: https://github.com/horizon-rust/horizon/issues/18811

## Port and target

Legacy4J → [Legacy Horizons](https://github.com/Jacob0225/Legacy-Horizons), native client mod, Minecraft 26.3, Mod API 0.2. Inspected SDK: Horizon `0b14f8acfd97fa2c750e42795249dcc981178ca4`.

## What the port needs

Legacy4J's `LocalPlayerMixin` and `ClientEntityMixin` replace parts of the local creative-flight step while its `legacyFlight` game rule is enabled:

- Move through normal collision resolution rather than applying a knockback impulse. Vertical ascent uses jump strength plus jump boost, descent uses `-0.57`, multiplied by flying speed / `0.05`.
- Apply a look-direction sprint boost that ramps over ten ticks, using `(ticks / 10)^2`.
- Keep flight enabled when touching the ground, control takeoff/descent, and add `0.5` step height while sprinting and ascending.
- Snap relative movement and independent left/right flight inputs to the console movement directions.

The mod must leave ordinary movement and servers without an opt-in policy alone.

## Current gap

`movement_input` edits keys/axes, but cannot replace flight travel, its collision-aware displacement, step height, or automatic landing behavior. `push_player` adds a bounded velocity impulse; it cannot express `move(MoverType.SELF, delta)` or replace those flight rules. Player abilities and client velocity reads do not supply the missing operations. `player_move` is a server report veto, after client movement has happened.

Related #17886 concerns replacing a **server** player's velocity; it does not provide a client flight-step policy. I searched existing issues for creative flight, movement input, and client velocity before filing this more specific request.

## Requested capability

A supported, opt-in local-player movement/flight hook that can adjust or replace flight displacement through the engine's collision path, and control flight landing/takeoff and step height. An equivalent declarative flight policy would also work; the exact API shape is up to maintainers. It should preserve server validation and avoid granting flight to a player without the server's flying ability.

## Acceptance example

A native client mod can reproduce the ten-tick sprint ramp, ascent/descent and grounded-flight rules above in a disposable creative world, including movement into walls and landing, without teleports or stacked knockback. With the policy disabled, vanilla flight is unchanged.

This is a source/API capability finding, not a claim of a reproduced game crash. The port currently leaves creative-flight physics unchanged.
