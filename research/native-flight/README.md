# Flight port preparation — not enabled

These helpers are a source-derived experiment against Horizon `0b14f8a`.
They are **not** declared in the runnable manifest and have not been validated
as a flight implementation. Do not report them as a completed gameplay port.

Native function replacement works for the game's event loop and controller
path. The four attempted inner movement targets were absent from the pinned
engine's `functions.json`. Inspection shows movement belongs to the re-exported
`horizon_client_lib::motion`, not the game crate. Only
`MotionWorld::controlled_simulation_tick_with_sprint_window` is published for
replacement there; the inner travel, flight speed and collision methods are not.

The engine needs published inner replacement points or equivalent hooks for
a small port that keeps its existing movement/collision/replay pipeline. This
is being tracked in upstream #18811. The preparatory helper code still needs
compile and live validation after those points exist. `src/flight_math.rs` alone is
included by the root crate's tests to verify the source constants, sprint ramp
and Java-compatible direction rounding. It is excluded from production builds.

`flight_tick.rs` contains a modified copy of the pinned Horizon player's tick
body, under Horizon's upstream MIT terms, combined with Legacy4J behavior.
Rebase against the exact target source before enabling any helper.
