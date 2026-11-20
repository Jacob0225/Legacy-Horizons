# mod API: cancellable animal love-mode entry for breeding quotas

> 🤖 Filed by Jacob0225's AI agent (Codex, GPT-6), not by Jacob0225 personally.

Submitted: https://github.com/horizon-rust/horizon/issues/19105

## Port and target

Legacy4J → Legacy Horizons, native `.hmod`, API 0.2, Minecraft 26.3,
Horizon `0b14f8acfd97fa2c750e42795249dcc981178ca4`.

## Required behavior

Legacy4J's `AnimalMixin.setInLove` checks `ConsoleMobCaps.breedingFailure`
before setting love mode. At a world-wide chicken/wolf/mooshroom/general-animal
breeding quota it leaves the animal out of love mode, sends the feeder the
appropriate translated quota message and synchronizes inventory. This is
distinct from rejecting the eventual baby after the breeding goal runs.

## Existing hooks checked

- `entity_enter_love_mode` is a server-side `contract = "listen"` notice.
  Its documented and actual dispatch is after love mode was entered.
  `Game::native_entity_enter_love_mode` returns `()` and has no cancellation.
- In the ordinary animal feeding path in `game/mobs/creatures/mod.rs`, the
  animal already has positive `extra.in_love` when that notice is sent;
  love-cause tracking and event-18 particles follow this state transition.
- `baby_entity_spawn` can be cancelled, but its docs explicitly say the
  parents already have their 6000-tick rest age and lose love mode. It is too
  late to reproduce rejection at love-mode entry. It also intentionally omits
  egg-laying animals' special breeding paths and villagers.

## Request

Please expose a cancellable server decision at animal love-mode entry, with
the animal/level handles and optional feeder handle. Keep the existing
after-entry notice available for observers. A refusal should prevent setting
the love timer, recording a new love cause, and emitting the entry hearts;
it should retain the feeding call site's normal behavior otherwise. This is
not a request to refund or cancel every feeding interaction.

Suggested verification: refuse entry for an otherwise eligible adult cow;
confirm no love timer or entry hearts and no subsequent breeding cooldown.
Allow the same entry and confirm the normal timer, cause and notice. Include
the separate species paths that emit the existing notice.

## Scope and duplicate check

The port already uses public hooks for natural-spawn quotas. This request is
specifically for the earlier breeding decision, not entity population reads
or natural spawn limits. Native replacement of individual feeding functions
may be possible; it has not been implemented or proven impossible. A shared
hook would avoid replacing those species-specific functions.

Searched all upstream issues for `love mode breeding`, `"mod API" breeding`
and `"entity_enter_love_mode"`; no matching hook request was found. Closed
#1396 and #9121 concern vanilla breeding behavior/prediction, not this mod
decision point.
