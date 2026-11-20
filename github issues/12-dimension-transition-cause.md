# mod API: distinguish respawn from login and other dimension transitions

> 🤖 Filed by Jacob0225's AI agent (Codex, GPT-5), not by Jacob0225 personally.

## Summary

`DimensionTransitionScreenDrawEvent` reports `NetherPortal`, `EndPortal` or `Other`. `Other` combines initial login, death respawn and every non-portal transition, so a registered screen cannot choose correct presentation or wording for those cases.

This blocks exact Legacy4J loading-screen parity. A death respawn must say **Respawning**, while command/plugin travel between dimensions should say **Entering …** or **Leaving …**, and initial login should retain the normal initial-world loading presentation.

## Tested environment

- Horizon commit: `0b14f8acfd97fa2c750e42795249dcc981178ca4`
- Minecraft data: 26.3
- Mod SDK / shader API: 0.2 nightly from 2026-11-18
- Native Rust `.hmod`, built by `horizon-launcher native-build`

## Current API

`DimensionTransitionScreenDrawEvent` provides:

- the registered screen id;
- `DimensionTransitionReason::{NetherPortal, EndPortal, Other}`;
- optional integrated-server progress;
- a draw context.

The registration key provides source/destination dimensions, but not the packet/cause that selected the transition. Source and destination are insufficient: a death in the End can respawn in the Overworld, which has the same pair as non-death travel out of the End.

## Reproduction

1. Register conditional screens for Overworld↔Nether and End→Overworld transitions.
2. In `dimension_transition_screen_draw`, inspect `event.reason`.
3. Compare an initial login, a death respawn and a command/plugin teleport that does not use a portal.

All are reported as `DimensionTransitionReason::Other`.

The Legacy4J port can correctly label portal travel and same-dimension respawns, but it cannot reliably label a death that changes dimension without also mislabeling ordinary non-portal travel.

## Expected

Expose a stable transition cause that distinguishes at least:

- initial login;
- death respawn;
- Nether portal;
- End portal;
- other travel.

This could be an expanded non-exhaustive cause enum, or a separate `respawning: bool`/`initial_login: bool` alongside the existing vanilla-compatible reason. It should describe presentation only and must not expose or mutate server-only state.

## Actual

The port must choose between incorrect **Respawning** text on ordinary travel and incorrect entering/leaving text on cross-dimension death respawns. It currently favors truthful source/destination wording for different-dimension travel and reserves **Respawning** for same-dimension registrations.

## Existing issue check

- The dimension-transition registration/draw API itself is already implemented; this report is only about the missing cause distinction in its event.
- #4953 and #6452 are broad API trackers and do not request this event field.
- Searches for `DimensionTransitionReason`, respawn/login transition reasons and registered transition screens found no exact issue.

## Acceptance criteria

- A selected dimension-transition screen can distinguish initial login, death respawn, Nether portal, End portal and other travel.
- Cross-dimension death respawns are distinguishable from command/plugin travel with the same source and destination.
- Existing `NetherPortal` and `EndPortal` behavior remains compatible, or the change is documented as the intended SDK break before the stable post-0.2 API.
- The cause is read-only and cannot alter readiness, timeout, player state or the server's transition.
- Integrated and remote worlds report equivalent causes for equivalent transitions.
