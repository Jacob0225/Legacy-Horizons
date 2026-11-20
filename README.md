# Legacy Horizon

A Horizon-native port of Legacy4J, with GUI parity as the first milestone.

## Gameplay work

The native build now ports Legacy4J's axial controller movement (25% dead
zone), optional smooth input, and right-stick camera curve (34% dead zone,
squared response, source sensitivity scaling and spyglass slowdown). Movement
defaults to Horizon's normal shaping on remote servers. Existing Horizon
gameplay button mappings remain in use: this is not the full Legacy controller
preset, auto-aim, gyro or contextual inventory interaction port.

Natural-spawn quotas are available separately through
`/gamerule legacy:natural_mobcap_limits true` (off by default).
`/legacy_mobcaps` reports the loaded population and active creature gate.
The switch applies source bucket/type limits to supported natural spawn
checks and temporarily lifts the vanilla Overworld creature cap; turning it
off restores the previous numeric cap. It does not remove existing mobs.
Breeding, manual spawning, placed entities and animal despawn/protection are
unfinished, and Horizon's category scheduling remains in use. This is not
the full Legacy mob-cap gamerule.

Creative-flight and elytra physics remain unchanged. Native inner movement
replacement attempts were refused by the pinned function index; preparation
and evidence are in `research/native-flight/` and issue 15. Potion-cauldron
preparation compiled but was rejected at startup because a mod block entity
cannot own vanilla water cauldrons. That experiment is archived and excluded
from the runnable build; see `research/potion-cauldron/` and issue 17.

See [GAMEPLAY-STATUS.md](GAMEPLAY-STATUS.md) for tested behavior and remaining
work. This project has not reached full GUI or gameplay parity.

## Current GUI slice

- Uses the current flat Horizon native-mod project format.
- Carries Legacy4J's original assets and data at the package root.
- Replaces the Horizon title drawing with Legacy4J's exact 225x20 button
  geometry, five-pixel spacing, labels, hover state and keyboard focus. The
  title and plain Help/Controls/Settings lists stay directly over the panorama;
  only Legacy4J screens that define a panel receive one.
- Implements the title, Play Game, Leaderboards, Help & Options, Controls,
  Settings, Store, Change Skin, Credits and Exit-confirmation menu hierarchy.
- Play Game now uses Legacy4J's 300-pixel-wide panel, recessed list area and
  joined Load/Create/Join tabs, with pointer and keyboard tab navigation.
  Its world list and row actions still require upstream title-state APIs;
  Create/Join rows are disabled scaffolding.
- Implements the title-side Mouse Settings, Key Binds, Controller, Game
  Options, Audio, Graphics, User Interface, Reset Settings and store-category
  pages, including their scrolling and parent navigation.
- Reads actual supported mouse/input options, key bindings, view bobbing and
  master volume. These vanilla values are read-only; unsupported option rows
  are explicitly disabled instead of displaying invented defaults.
- Cave and minecart sound switches work and save across restarts. Legacy
  elytra sound uses the original volume cap of 0.8 and fixed pitch of 1.0 on
  both starts and live updates. Preferences live in `config/legacy/audio.json`.
  This is a partial audio port; music fades and the full Legacy audio system
  remain unported.
- User Interface → Display Hand now hides both first-person hands and held
  items, matching Legacy4J's `GameRendererMixin`. It saves separately in
  `config/legacy/interface.json`; world/HUD rendering remains independent.
- Display HUD, Display Game Messages, Death Messages and Announce Advancements
  now work and save in the same file. Message filters use translation-component
  keys and preserve player chat. Display HUD also controls the block outline.
- Controller D-pad/stick navigation, A/confirm, B/back and bumper tab changes
  now reach the replacement title pages through native adapters. Exit Game's
  confirmation now performs Horizon's normal graceful shutdown.
- Implements the valid 43-entry How To Play index. Its articles are read from
  Legacy4J's original UI-definition JSON, translated through Horizon's active
  language resources, formatting-cleaned, wrapped and scrollable.
- How To Play's list and article reader have no backing panel: their spaces
  stay clear over the panorama, as requested by the project owner.
- Uses Legacy4J's translation keys for the title hierarchy and every How To
  Play title, with explicit Legacy wording where the original relies on the
  separately enabled Legacy Resources pack.
- Supports mouse and keyboard navigation, including wheel navigation in the
  How To index, and plays Horizon's normal UI button sound for activations and
  help-page changes.
- Replaces Nether/End travel and same-dimension respawn transitions with
  Legacy4J's translated entering/leaving/respawning header geometry and
  animated eight-block loading indicator.
- Adds a live crafting recipe browser only beside crafting-table screens.
  Personal inventory crafting remains unchanged. It reads Horizon's current crafting recipes, draws
  their real result and ingredient items, supports pointer, wheel and keyboard
  selection. Recipe placement is wired to the public recipe API, but the tested
  crafting-table call was refused; placement remains incomplete pending diagnosis.
- Crafting category tabs now filter Building, Redstone, Equipment and
  Miscellaneous recipes. Title pages have a clickable Back hint. Light beveled
  panels replace the translucent dark fallback boxes, and exit/reset dialogs
  show translated confirmation messages with unavailable actions disabled.
- The side browser requires 164 GUI pixels to the left of the stock inventory;
  at larger GUI scales it shows a visible scale hint instead of covering slots.
- Ports the Legacy credits roll from the original credits JSON: combined
  resource credits, 18-pixel line spacing, enlarged section headings, yellow
  names, accelerated scrolling and looping. The original background and fade
  sprites are attempted; issue 1 still prevents their display in the pinned
  nightly, so its black fallback is not artwork parity.
- Targets the installed Nov 18 Horizon nightly at `0b14f8a` and its matching
  horizon-mixins revision.

Menu destinations that require Horizon-owned behavior remain safely inert.
Clicks are consumed so an invisible stock button cannot run the wrong action.
The exact public-API boundaries are documented as ready-to-post reports in
[`github issues`](github%20issues/):

1. bundled title GUI textures;
2. opening a mod-owned screen at the title;
3. opening Horizon's built-in title/options screens;
4. graceful client quit;
5. GUI player/skin preview;
6. active input-glyph drawing;
7. replacement vanilla container slot layouts/hitboxes;
8. validated vanilla client-option writes;
9. controller navigation/activation for mod-drawn screens;
10. book and sign overlay render/input hooks;
11. closing world-save screen drawing;
12. distinguishing respawn from login/other dimension transitions.
13. reading Horizon's active, resource-pack-aware title splash from a
    replacement title renderer.
14. native client-tick recipe placement refusing an open crafting table:
    [#18799](https://github.com/horizon-rust/horizon/issues/18799).
15. collision-aware local creative-flight movement and takeoff/landing rules:
    [#18811](https://github.com/horizon-rust/horizon/issues/18811).
16. elytra camera banking requires camera-pose roll, already requested in
    [#18694](https://github.com/horizon-rust/horizon/issues/18694); no duplicate filed.
17. saved/synced potion state on vanilla cauldrons:
    [#19095](https://github.com/horizon-rust/horizon/issues/19095).
18. cancellable animal love-mode entry for breeding quotas:
    [#19105](https://github.com/horizon-rust/horizon/issues/19105).
19. controller-axis processing hooks (working native fallback included):
    [#19106](https://github.com/horizon-rust/horizon/issues/19106).

Issues 1-12 were posted by the project owner. Issue 13 was checked for an
existing report and posted with GitHub CLI as
[`#18655`](https://github.com/horizon-rust/horizon/issues/18655). Every report
also has a matching local Markdown copy.

Issue 4 is closed and the port now implements graceful quit natively. Issue 9
has a working native adapter for title-menu navigation; it is not a full
controller port. Other title actions, custom art, player preview, vanilla
settings writes and changed container layouts remain unfinished. In
particular, the crafting browser renders and selects recipes,
but it stays beside Horizon's stock inventory slots until issue 7's slot-layout
API exists; moving those live slots into Legacy4J's 348x215 crafting panel is
not safe with the current API.

## Development

Run `cargo test --locked` for the local layout/navigation tests. Build the exact
nightly package with `horizon-launcher native-build` and test it in the
`legacy-horizon-gui-test` instance. The pinned SDK is cached in `.horizon/sdk/`,
so ordinary tests do not fetch a second copy of Horizon.

For a fresh checkout, provision the exact SDK pinned in `horizon-mod.toml`
using Horizon's native-mod tooling before running Cargo. The SDK and build
outputs are intentionally excluded from Git. See [CONTRIBUTING.md](CONTRIBUTING.md)
for current status and testing boundaries.

## Credits and licensing

This is an unfinished Rust port derived from Legacy4J by Wily and its
contributors. Original Legacy4J material retains its upstream LGPL-3.0 terms
and copyright notices; see [LICENSE-LEGACY4J.md](LICENSE-LEGACY4J.md).
The repository's existing MIT license does not relicense upstream material.
Bundled resource notices and third-party credits remain applicable.

