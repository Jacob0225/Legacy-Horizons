# Contributing

This repository contains the actual Horizon port at its root. It is an early
GUI parity implementation, not a complete playable replacement for Legacy4J.

## Build

Use Horizon's native mod tooling to provision the SDK selected by
`horizon-mod.toml`. The current target is Minecraft 26.3, Horizon commit
`0b14f8acfd97fa2c750e42795249dcc981178ca4`, Mod API 0.2.

Once `.horizon/sdk/api` exists:

```sh
cargo test --locked
horizon-launcher native-build --instance YOUR_TEST_INSTANCE .
```

Use a disposable instance/world for testing. Do not commit `.horizon`, `target`,
account data, game files, local worlds, tokens, or machine-specific settings.

## Status

- Title and Help navigation and the 43-section help index/article viewer work.
- Additional options pages are visual scaffolding with hard-coded displayed
  values. They do not read or apply the player's actual settings.
- World, server, store, skin and credits flows remain incomplete.
- Crafting currently adds a side browser to the vanilla slot layout. Selection
  and ingredient previews were observed live; recipe placement was refused in
  the tested build and needs diagnosis. Category icons are visual scaffolding.
- Exact Legacy crafting slot placement remains blocked by the slot-layout API
  request documented in `github issues/07-vanilla-container-slot-layout.md`.
- Latest source passes 11 local tests. The tabbed Play Game slice builds as a
  native mod; its latest visual check is pending because the launcher stalled
  at sign-in/asset checks. Exact visual parity has not been verified.

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
