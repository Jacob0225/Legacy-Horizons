# Continuing Legacy Horizons with Claude or another agent

This is the actual port repository, published to
[Jacob0225/Legacy-Horizons](https://github.com/Jacob0225/Legacy-Horizons).
In the owner's larger workspace it lives in `Legacy Horizon/`. Sibling
folders contain reference source and older attempts; do not develop the port
in the failed translation or a Horizon engine checkout.

## Owner's requirements

- Port Legacy4J behavior and appearance, with GUI parity first. Research the
  original screen/behavior before implementing it. Source and the owner's
  supplied screenshots take precedence over guessed console conventions.
- Button-list screens must stay clear over the panorama. Do not add a
  white/gray card, outline or translucent dark rectangle around the group.
  This includes settings, controls, skin-option lists and How To Play.
  Buttons retain their own background and focus/hover appearance. World-list,
  inventory and crafting surfaces shown in the reference screenshots are
  distinct screen content; do not erase their source-defined layouts.
- How To Play's index and articles have no backing background at all.
- The supplied Legacy crafting layout applies to crafting-table screens.
  Do not replace personal inventory crafting with that table menu.
- Leave the current Rust Edition subtitle unless the owner changes that
  preference. Do not silently substitute the Java Edition logo.
- Continue useful, independent work after completing/pushing a section.
  When a real engine/API boundary is reached, explain it, document evidence
  and follow the issue workflow; do not fake a working feature.
- The owner requested GitHub CLI for issues and regular pushes under their
  configured GitHub/Git identity. Existing reports must not be reposted.

## Read before coding

1. Inspect `git status` and applicable local instructions. Preserve unrelated
   edits/deletions. Stage only the work you performed; do not stage everything.
2. Read `CONTRIBUTING.md`, `GAMEPLAY-STATUS.md` and the relevant local issue.
3. Locate the corresponding code/assets in the sibling `Legacy4j/` source
   and the supplied `references/` images. The workspace `doc/` folder and
   the exact pinned SDK docs are useful API references. Paths vary on another
   machine, so locate them instead of hardcoding this workstation's paths.
4. `neolegacy/` may be consulted for visual/behavior ideas. Do not copy its
   console source/assets into the public port. Retain Legacy4J attribution
   and upstream licensing; do not relicense inherited material as MIT.
5. Prefer focused searches (`rg`, `rg --files`) over reading whole SDK trees.

## API and native implementation

The exact Minecraft, Horizon and mixins revisions are in `horizon-mod.toml`.
The SDK provisioned for that target is `.horizon/sdk/api`. Do not assume the
latest online docs describe the installed build, or upgrade the target just
to bypass an error without documenting and testing the migration.

Follow the SDK's hook/Part/injection/native ladder. Prefer existing hooks or
typed Part operations, use `ServerHost` for authoritative writes, and retain
engine validation, transactions, inventory synchronization and persistence.
Read its modding `agent-rules.md`, native-route documentation and current
upstream contribution/agent rules before submitting upstream work.

Native function replacement works here. Helpers in `src/mixins/` are declared
in `horizon-mod.toml` and compiled in their target module's scope; they are
not ordinary `mod` declarations in the root crate. Look at existing helpers
for concrete examples. Game subdirectories marked `.game-area` are flattened
in module names. Re-exports can hide the actual defining crate: player motion
is in `horizon_client_lib::motion`, despite its physical source path.

Use the selected prebuilt SDK's `functions.json` to confirm replacement
targets. Game-crate methods and lower-crate published replacement points
have different availability. A native build accepting a helper does not
prove the feature works. A missing small inner replacement point also does
not prove that every larger native implementation is impossible.

Existing native adapters provide controller input/title navigation, graceful
quit and six saved settings. Vanilla option toggles queue changes to the
client loop and call Horizon's validated `options_control` setter; do not
replace this with direct `options.txt` edits. Controller inversion uses the
engine's normal settings-changed handler.

## Known boundaries and honest status

- Crafting is a side browser beside stock slots, not the full replacement
  slot layout. Recipe placement was refused in the tested build.
- Natural mob caps are an opt-in subset, not the full source gamerule.
  Breeding, manual/placed entities, despawn and scheduling parity remain work.
- Controller curves/title navigation work; the complete source gameplay
  preset, contextual interactions, auto-aim and gyro do not.
- Creative/elytra physics are not enabled. `research/native-flight/` records
  rejected inner targets and preparation; arithmetic tests are not runtime
  flight validation.
- Potion-cauldron preparation is excluded because its vanilla-block saved
  block-entity registration caused startup rejection. Do not re-enable it
  until the boundary is resolved and the complete path is tested.

Archived experiments use `.rs.txt` in `research/`: the native packager checks
Rust source outside `src/` too. Test-only preparation belongs under `src/`
with `#[cfg(test)]`. Never call an archived experiment implemented gameplay.

## Build, live verification and screenshots

Provision the exact SDK with Horizon's native tooling first. From the repo:

```sh
cargo test --locked
horizon-launcher native-build --instance YOUR_DISPOSABLE_TEST_INSTANCE .
```

Use the installed launcher's actual path if it is not on PATH. Do not build
the whole Horizon engine when the native-mod build is sufficient. Use
meaningful tests for new logic and the existing tests for regressions; a
background-only paint change mainly needs build and visual verification.

Use the Horizon MCP to launch/control the test client and inspect `game_ui`,
real activations, `game_options_get`, world state and screenshots. Read the
actual tool schemas. Wait for a ready client before issuing dependent input;
if a call fails, diagnose it before chaining more mutations.

Use an isolated display and disposable instance/world, never the owner's
real save. Follow applicable test-environment rules. If a graphical display
is unavailable, the earlier run used a headless Weston Wayland display at
1280×720; this is a local test setup, not a portable project dependency.

Check mouse, keyboard and controller activation where relevant. Compare
appearance against the source/reference at a known GUI scale. For settings,
check actual engine values, persistence after restart and restore temporary
test preferences. Release held sticks/buttons immediately: MCP inputs stay
held between calls, so tool latency invalidates exact motion-duration claims.

Save actual game captures as PNG evidence in the repository and inspect them.
Do not use generated/mock images or image editing as verification. Screenshots
prove appearance, not hidden physics or persistence. Label temporary settings
and historical screenshots accurately. Record target/build, observations,
limitations and remaining work in `GAMEPLAY-STATUS.md` and relevant docs.

## GitHub and handoff

Use [github issues/README.md](github%20issues/README.md) for the exact reporting
conventions. New issue title goes in GitHub's title field without `#`; retain
the `#` heading in the local Markdown file. Keep the matching Markdown and
upstream URL in the same commit as the research/change.

Before committing, inspect the diff and `git diff --check`, then stage explicit
paths. Preserve unrelated work. Use the configured user identity and the
authenticated account (`gh api user --jq .login`); do not create another
account or change global Git identity. Add an accurate agent co-author line,
using your own tool/model rather than copying Codex's attribution.

Push verified changes to the established branch/remote when authorized, as
the owner has requested for this project. Confirm the push succeeded; do not
say it is published while it is still running or rejected. Do not force-push,
overwrite another contributor's work or include `.horizon/`, `target/`, tokens,
game resources, private account data or local saves. Handle authorization or
tool failures openly rather than bypassing them.

A useful handoff says what changed, why, what passed, which screenshots show
it, which issues already exist and what remains unimplemented. Never report
full GUI/gameplay parity based only on a successful build.
