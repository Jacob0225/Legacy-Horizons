# Contributing

This repository contains the actual Horizon port at its root. It is an early
GUI parity implementation, not a complete playable replacement for Legacy4J.

Start with [AGENT-README.md](AGENT-README.md) for the owner's requirements and
the end-to-end workflow. The [issues README](github%20issues/README.md) records
report formatting and existing upstream reports; `CLAUDE.md` points Claude to
the same instructions. Button-list menus must remain clear over the panorama,
without white/gray cards, borders or dark group backings.

## Build

Use Horizon's native mod tooling to provision the SDK selected by
`horizon-mod.toml`. The current target is Minecraft 26.3, Horizon commit
`8f83e5c85ae3b9a117f742e8d91170c5c9080a06` (Nightly 2026-11-21 · 8f83e5c),
Mod API 0.2. The previous target was `5ef7734` (Nightly 2026-11-19), and
earlier validation used `0b14f8a`, a Nov 18 build replaced the
same day by `dd67b76`, which new launchers may no longer be able to install
(horizon-rust/horizon#18598).

Once `.horizon/sdk/api` exists:

```sh
cargo test --locked
horizon-launcher native-build --instance YOUR_TEST_INSTANCE .
```

Use a disposable instance/world for testing. Do not commit `.horizon`, `target`,
account data, game files, local worlds, tokens, or machine-specific settings.

## Status

- Title and Help navigation and the 43-section help index/article viewer work.
- Options pages read supported input preferences, key bindings, view bobbing
  and master volume. Native adapters apply/save mouse X/Y inversion, discrete
  scrolling, auto jump, view bobbing and controller Invert Look through the
  engine settings path. Other vanilla values remain read-only (issue 8);
  unsupported rows say `Unavailable` and are disabled.
- Cave and minecart sound switches work and persist in `config/legacy/audio.json`.
  Legacy elytra audio caps instance volume at 0.8 and fixes pitch at 1.0, matching
  `ElytraOnPlayerSoundInstanceMixin`. `legacy_elytra_sound: false` in that file
  disables this local sound behavior. Custom replacement sound events are not
  identified as vanilla minecart/cave instances.
- Display Hand works through the cancellable first-person hand hook and saves
  in `config/legacy/interface.json`. Live before/after captures are included
  as `gameplay-hand-on.png`, `gameplay-hand-off.png` and `gameplay-held-item-off.png`.
- World, server, store, skin and credits flows remain incomplete.
- Crafting currently adds a side browser to the vanilla slot layout. Selection
  and ingredient previews were observed live; recipe placement was refused in
  the tested build and needs diagnosis. Category tabs filter the live recipe
  table and were exercised through Horizon MCP. The side browser cannot fit
  beside stock slots at every GUI scale; it explicitly reports this limitation.
- Exact Legacy crafting slot placement remains blocked by the slot-layout API
  request documented in `github issues/07-vanilla-container-slot-layout.md`.
- Latest source passes 23 local tests and builds as a native mod. Live MCP
  screenshots of clear help, Play Game tabs and category-filtered
  crafting browser are included as `gui-*-progress.png`. Exact visual parity
  has not been achieved: bundled artwork, slot layout and title actions still
  have the upstream API limitations described in the issue reports.

Recent research used Legacy4J's HelpAndOptionsScreen, LegacyCreditsScreen and
WinScreenMixin, plus NeoLegacy's UIScene_Credits as a visual/behavior cross-check.
No NeoLegacy code or assets were copied into this repository. Credits uses
the packaged Legacy4J credits JSON and the player's active Minecraft resources.
Credits artwork is blocked by issue 1. Table recipe placement's exact observed
refusal is documented in issue 14 (#18799); it is not reported as functional.

Creative-flight source research is documented in issue 15 (#18811). The client
API lacks the collision-aware local movement and flight landing/takeoff policy
that Legacy4J needs, so this port leaves flight physics unchanged. Server
velocity APIs do not fill this gap.

Native function replacements now implement graceful quit, controller title
navigation, axial smooth movement and the source right-stick camera curve.
The pinned lower movement crate does not publish the inner flight/collision
targets the archived experiments attempted. Do not enable those experiments
or describe their arithmetic tests as working flight physics.

The natural-spawn subset is opt-in through `legacy:natural_mobcap_limits`.
Use `/legacy_mobcaps` to inspect the live census and verify the temporary
creature-gate override/restoration. Do not call this the complete source mob
cap policy: breeding/manual placement/despawn rules and scheduling parity
still require work. The potion-cauldron experiment is excluded from production
because vanilla-block ownership registration was refused at runtime (#19095).
See [GAMEPLAY-STATUS.md](GAMEPLAY-STATUS.md) for evidence and remaining scope.

## Visual references

The user-supplied screenshots in `references/` show the intended world/server
menus and Legacy crafting layout. Use these alongside the upstream Legacy4J
source for dimensions, tabs, recessed panels and ingredient grouping. Current
screenshots are progress evidence, not a claim of identical appearance.

## Upstream issues

Keep reports in `github issues/`. Check for duplicates before submitting a new
Horizon issue. Issues 1–12 were already submitted by the owner; issue 13 is
https://github.com/horizon-rust/horizon/issues/18655. Do not repost them.

The response accepting a separate read-only login/respawn event field is at
https://github.com/horizon-rust/horizon/issues/18651#issuecomment-7029386515.
