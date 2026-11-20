# mod API: draw a player profile and skin in a GUI without an open world

> 🤖 Filed by Jacob0225's AI agent (Codex, GPT-5), not by Jacob0225 personally.

## Summary

`DrawContext::entity` can draw a new living entity type or an entity that already exists in the client's world, but a client mod cannot draw a player model from a profile/skin while no world is open.

This blocks title-screen skin selectors and profile previews. Legacy4J's **Change Skin** GUI needs to display the currently selected player skin and candidate skins before entering a world.

## Tested environment

- Horizon commit: `0b14f8acfd97fa2c750e42795249dcc981178ca4`
- Minecraft data: 26.3
- Mod SDK / shader API: 0.2 nightly from 2026-11-18
- Native Rust `.hmod`, built by `horizon-launcher native-build`

## Current API

`GuiEntity` has two forms:

- `GuiEntity::kind`, for a registered living entity type;
- `GuiEntity::existing`, for an entity handle in the client's currently open world.

A player is not a constructible entity type with an arbitrary `GameProfile`, and there is no existing player entity on the title screen. `ClientTabListEntry` exposes profile name and UUID in a world, but no skin/model snapshot suitable for `DrawContext::entity`.

## Reproduction

1. Open a mod-drawn skin-selection screen from the title menu.
2. Obtain or load a candidate 64×64 player skin and whether it uses the slim or wide model.
3. Try to render that candidate as the normal player model inside a GUI box.

There is no `GuiEntity` description or draw helper that accepts a player profile/skin, and `GuiEntity::existing` is unavailable because no world/player exists.

## Expected

A client mod can ask Horizon's existing player renderer to draw a GUI-only player preview without spawning or joining a world. One possible shape is:

```rust
let preview = GuiPlayer::profile(profile_uuid, profile_name);
draw.player(&preview, view)?;
```

For local/offline skin packs, an explicit validated skin form would also be useful:

```rust
let preview = GuiPlayer::skin(texture, PlayerModel::Slim);
```

The preview should use Horizon's normal player model, skin layers, lighting and `GuiEntityView` clipping/look behavior.

## Actual

The Change Skin screen can be laid out and receive input, but it cannot show the player or candidate skins. Substituting a zombie or a fixed Steve image would not be GUI parity.

## Existing issue check

- #9581/#9601 broadly tracked missing client Host services and mentioned player-list skin snapshots, but the resulting public surface still exposes no drawable profile/skin GUI entity and that issue is closed.
- #3122 concerned already loaded player skins disappearing after a resource reload; it did not concern mod-created GUI previews.
- No exact issue for a title-safe player/profile form of `DrawContext::entity` was found.

## Acceptance criteria

- A native client mod can draw the signed-in/local player's skin on a title-owned or mod-owned screen without an open world.
- A mod can draw another supplied profile UUID/name after Horizon resolves its skin through the normal skin cache.
- A bounded local skin texture plus slim/wide model can be previewed for an offline skin-pack selector.
- The preview uses the normal player renderer and outer skin layers.
- Bad textures/profiles return a typed refusal and do not panic.

