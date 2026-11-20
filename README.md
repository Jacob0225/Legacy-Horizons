# Legacy Horizon

A Horizon-native port of Legacy4J, with GUI parity as the first milestone.

## Current GUI slice

- Uses the current flat Horizon native-mod project format.
- Carries Legacy4J's original assets and data at the package root.
- Replaces the Horizon title drawing with Legacy4J's exact 225x20 button
  geometry, five-pixel spacing, labels, hover state and keyboard focus. The
  title and plain Help/Controls/Settings lists stay directly over the panorama;
  only Legacy4J screens that define a panel receive one.
- Implements the title, Play Game, Leaderboards, Help & Options, Controls,
  Settings, Store, Change Skin, Credits and Exit-confirmation menu hierarchy.
- Implements the title-side Mouse Settings, Key Binds, Controller, Game
  Options, Audio, Graphics, User Interface, Reset Settings and store-category
  pages, including their scrolling and parent navigation.
- Implements the valid 43-entry How To Play index. Its articles are read from
  Legacy4J's original UI-definition JSON, translated through Horizon's active
  language resources, formatting-cleaned, wrapped and scrollable.
- Uses Legacy4J's translation keys for the title hierarchy and every How To
  Play title, with explicit Legacy wording where the original relies on the
  separately enabled Legacy Resources pack.
- Supports mouse and keyboard navigation, including wheel navigation in the
  How To index, and plays Horizon's normal UI button sound for activations and
  help-page changes.
- Replaces Nether/End travel and same-dimension respawn transitions with
  Legacy4J's translated entering/leaving/respawning header geometry and
  animated eight-block loading indicator.
- Adds a live crafting recipe browser beside the survival inventory and
  crafting-table screens. It reads Horizon's current crafting recipes, draws
  their real result and ingredient items, supports pointer, wheel and keyboard
  selection. Recipe placement is wired to the public recipe API, but the tested
  crafting-table call was refused; placement remains incomplete pending diagnosis.
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

Issues 1-12 were posted by the project owner. Issue 13 was checked for an
existing report and posted with GitHub CLI as
[`#18655`](https://github.com/horizon-rust/horizon/issues/18655). Every report
also has a matching local Markdown copy.

This is not full Legacy4J GUI parity yet. The title/help slice is usable with
mouse and keyboard, while the remaining title actions, controller path, custom
art, player preview, functional settings and changed container layouts depend
on the API surfaces above. In particular, the crafting browser renders and selects recipes,
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

