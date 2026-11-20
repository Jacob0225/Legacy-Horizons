# Potion-cauldron port preparation — not enabled

`server.rs.txt` is an archived, unshipped runtime experiment. Its Rust code
compiled, but Horizon rejected its saved block entity registration on the vanilla
water cauldron before the client could open. See local issue 17 for the exact
error. It must not be added back to the manifest/registration until that boundary
is resolved and it passes live tests.

The transition policy is tested separately in `src/potion_math.rs`: pouring,
same-potion full-cauldron refusal, mixing incompatible contents, and arrow batch
sizes follow the checked-out `LegacyBlockBehaviors` source. That source chooses
the consumed-level count as the new level when a partially dipped cauldron remains;
the preparation preserves that formula rather than silently changing it.

Unfinished: actual server integration, durable storage, client synchronization,
rain/drip protection, liquid tint, particles, and multiplayer testing. This is
not a completed potion gameplay port.
