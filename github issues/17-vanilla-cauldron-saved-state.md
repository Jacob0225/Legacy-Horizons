# mod API: saved and client-synced potion data on vanilla water cauldrons

> 🤖 Filed by Jacob0225's AI agent (Codex, GPT-6), not by Jacob0225 personally.

Submitted: https://github.com/horizon-rust/horizon/issues/19095

## Port and target

Legacy4J → Legacy Horizons, native `.hmod`, Minecraft 26.3, API 0.2,
Horizon `0b14f8acfd97fa2c750e42795249dcc981178ca4`.

## Source behavior

Legacy4J attaches `WaterCauldronBlockEntity` to the existing
`minecraft:water_cauldron` block. It stores complete potion contents and the
last bottle's item type, saves them in the chunk and sends an update tag to
clients. This supports pouring normal/splash/lingering potions, filling bottles,
rejecting incompatible mixtures, and tipping arrows. Rain/drip eligibility and
the liquid's color also depend on those contents. The vanilla block identity
and its level property remain intact.

## Reproduction

In `Hooks::register`:

```rust
registrar.block_entity_type("water_cauldron")
    .blocks(&["minecraft:water_cauldron"])
    .build();
```

The native build succeeds, but the actual client refuses preparation and exits:

```text
mod legacy: block entity type legacy:water_cauldron cannot own vanilla block minecraft:water_cauldron
```

Observed in an isolated disposable Horizon MCP test instance. The failing
registration has been removed from the runnable port. The archived interaction
code and tested transition math remain in the repository for follow-up.

## Requested capability

A supported way to keep owned, chunk-persistent, transactional and client-synced
state at a vanilla cauldron position, including a vanilla block without a block
entity factory. It could be an opt-in block-entity attachment/factory extension
or another supported position-state service. Replacing a block with a differently
named custom cauldron would change its identity, commands, tags and mod interactions.

`BlockEntityPartState` currently keeps server-side state on an existing native
holder; it does not create a holder on a vanilla non-block-entity block. Adding
valid blocks to a type does not itself add the missing vanilla block factory.
The cauldron interaction registration API is useful, but does not supply the
missing durable/client-visible position state; its vanilla entries also cannot
be replaced through `register`.

## Acceptance example

A mod can pour a poison potion into a vanilla cauldron, leave and reload the
world, then fill a bottle or tip arrows with the same complete potion contents.
Transactions roll back the block, its saved contents and inventory together.
Clients can read the synced contents to draw the correct liquid color.

Searched existing issues for cauldron API requests, water_cauldron, and saved
state on vanilla block entities before filing; no matching request was found.
