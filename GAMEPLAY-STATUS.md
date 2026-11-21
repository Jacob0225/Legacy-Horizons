# Gameplay port status — 2026-11-20

## PR integration and local launcher instance (2026-11-21)

PRs 1–4 were reviewed as a stack, built/tested on Linux, and merged in order
with exact-head guards. Merge `331a14d` matches the reviewed `7157778` tree.
35 standalone tests passed in the isolated SDK checkout (temporary development
lock expansion; the native package lock was restored). Native package builds
passed with the committed registry lock. The permanent main checkout installed
`native-f037744-7009c0f7c971` into **Legacy Horizons - PR Test**.

MCP verified the HUD's hotbar/items/counts, armor, health, food and XP; virtual
gamepad Resume, Exit confirmation, Cancel/back and final leave-to-title; and
world persistence after reopening through the launcher (position, XP and mode).
Physical-controller and multiplayer verification remain unfinished. The
disabled pause destinations remain disabled. Screenshot evidence is in
`gameplay-merged-pr-hud.png` and `gui-merged-pr-pause.png`.

The selected local instance includes a separate **Legacy Horizons Demo** world
with a safe platform, workstations and equipment. See [TESTING.md](TESTING.md)
for the launcher steps; it does not include or alter existing player worlds.

## Target moved to f037744 (2026-11-21)

`horizon-mod.toml` now pins Horizon `f037744a79f9982c47876f49c5a3067279f88706`
(Nightly 2026-11-21 · f037744), published hours after 8f83e5c; its SDK still
names horizon-mixins `e59da90`. `Cargo.lock` needed no change, and `cargo test
--locked` passes (35 tests) against the f037744 SDK.

Launcher check on Windows: `native-build` installed `native-f037744-d0a3d5855c45`
with all mixins accepted. In game, through the launcher's MCP: the Legacy HUD
and the Legacy pause menu work as on 8f83e5c.

## Target moved to 8f83e5c (2026-11-21)

`horizon-mod.toml` now pins Horizon `8f83e5c85ae3b9a117f742e8d91170c5c9080a06`
(Nightly 2026-11-21 · 8f83e5c), the newest Nightly; horizon-mixins stays at
`e59da90`. `Cargo.lock` needed no change, and `cargo test --locked` passes (35
tests) against the 8f83e5c SDK.

Launcher check on Windows: `native-build` installed `native-8f83e5c-58748b755c45`
with all mixins accepted. In game, through the launcher's MCP: the Legacy HUD
(hotbar, health, food, armor, XP, mount hearts) and the Legacy pause menu
(open, Resume) work as on 5ef7734. The other 5ef7734 checks below were not
repeated.

## Target moved to 5ef7734 (2026-11-21)

`horizon-mod.toml` now pins Horizon `5ef7734fdd854add5efd6817178be3c988d8d050`
(Nightly 2026-11-19 · 5ef7734); horizon-mixins stays at `e59da90`. The old pin,
`0b14f8a`, was a Nov 18 build replaced the same day by `dd67b76` (the published
`nightly-2026-11-18`), and a Nightly superseded the same day may no longer be
installable on new launchers (horizon-rust/horizon#18598).

Checked so far: the package builds with `cargo horizon-mod build` against the
5ef7734 SDK, `cargo test --locked` passes (23 tests), and the six native
replacement targets (`idle`, `pad_move`, `pad_look`, `modify_input`,
`pad_menu_button`, `pad_step`) have bodies identical between `dd67b76` and
`5ef7734`. `Cargo.lock` follows the 5ef7734 SDK (its internal tool crates are
gone) and the game's own `libc`, `log` and `zerocopy` versions.

Launcher check on Windows (launcher `native-build`, prebuilt route): build
`native-5ef7734-7ef6e0a4ce09` passed with `left_out: []`, together with five
other native mods. Its `legacy` digest `5af8ba5c…` is the launcher's content
digest of the tested package, recomputed from the file. The tester reports
the Legacy title screen and menus in game; capture:
`gui-title-5ef7734.png` (Windows, 1919×1150 window, Rust Edition subtitle kept). Not yet re-verified on 5ef7734:
controller curves and navigation, the six saved settings, mob caps, audio
and credits; the validation builds below are all from `0b14f8a`.

## Legacy HUD placement (2026-11-21)

`src/hud.rs` and the `legacy_hud_*` mixins give the in-game HUD Legacy4J's
placement (`LegacyGuiElements.setup`, `prepareHUDRender`) at its defaults:
the hotbar, the health/armor/food/air rows and the experience bar are scaled
1.5× about the bottom centre (`getHUDScale`, HUD size 2) and raised 22.4 GUI
pixels (`getHUDDistance` at distance 1.0; the 17.6 added for in-game control
tooltips is left out because those prompts can't be drawn, API 06). The
selected item name and the action bar keep their size and rise by as much as
the health rows' top (41.9), keeping their stock gap. The unfocused chat moves
in by 29 and up by 64 (`ChatComponentMixin`) through `customize_chat_overlay`;
the open chat stays put because its input box can't be moved.

Route: a pose pushed in `gui_layer_pre` does not reach the stock layer, since
Horizon bakes each layer straight into HUD vertices, and the patch tool refuses
sites inside the layers' closures (`E_UNSUPPORTED_SITE`). So `BEFORE_CALL` and
`AFTER_CALL` mixins on `Game::hud_layers` sit around its `self.gui_layer` calls
(ordinals 1, 4, 5, 6 and 8 of 15) and transform the vertices, and the clip
rectangles of 3D item runs, that each of those layers added. Mods' own layers
drawn in those calls move with them.

Tested on Windows, Nightly 5ef7734, by the agent through the launcher's MCP
(`claude-test` instance, survival and creative): hotbar, selection, item
counts, a 3D grass block, torches and a sword; hearts, food, armor and the XP
level; the selected item name above the rows; chat lines clear of the HUD.
Capture: `gameplay-legacy-hud.png`.

Follow-up, same day: the three User Interface settings Legacy4J has for the HUD
now work and save to `config/legacy/hud.json`: HUD Size (1–3, `getHUDScale`
1×/1.5×/2×), HUD Opacity (0–100%, default 80%) and a new Interface Distance row
(0–100%, default 100%), each stepped by click or Enter. The moved layers are drawn
at `getHUDOpacity`: at 80% or less the HUD holds 0.8 for 3.6 s after the hotbar
selection changes, then fades to the setting. The mount's hearts
(`draw_mount_hud`) move with the rows; the jump bar is part of the contextual
layer already. The effect icons take `renderGuiEffects`' places (24 apart from
`width - 55`, rows at 18 and 42) through the same mixin route, and boss bars
start at `12 + 16 × distance` through `customize_boss_event_progress`. The item
name moves with its own stock line (59 up), so it stays above the armor row at
every size.

Tested through the MCP: sizes 3 → 1 wrap, 30% and 40% opacity (loaded from and
saved to the config file), the 0.8 hold after a slot change, 50% distance, a
tamed horse's hearts, Speed/Night Vision/Slowness icons, a `/bossbar`, and the
item name over the armor row at size 3. Not done: Legacy4J's SD/FHD UI modes, its
own boss-bar and effect-panel art (API 01), and its in-game control tooltips,
which need Horizon's input glyphs (API 06, horizon-rust/horizon#18396, open).
A newly given effect's fly-in animation passes through the icon remap and can
show away from the icons for a moment. Not tested: a controller, or
hunger/poison/absorption rows.

Container screens (chest, furnace family, brewing, anvil, trading…) stay stock:
Legacy4J moves their slots, and the pinned API still can't move live slots or
their hitboxes (API 07, horizon-rust/horizon#18397, open). The checklist marks
them `[!]`.

## Legacy pause menu (2026-11-21)

`src/pause_menu.rs` replaces the stock pause screen's drawing and buttons with
Legacy4J's column (`PauseScreenMixin`): 225×20 buttons, 5 apart, from
`width / 2 - 112`, `height / 3 + 5`. Resume Game closes the screen as Escape
does (queued to `client_tick_pre`, which runs while paused). Exit Game opens a
confirmation (Cancel / Exit Game, `legacy.menu.server_exit_message`) and then
calls `disconnect(None)`, which leaves as the stock Disconnect does and saves a
single-player world first. Help & Options, Leaderboards, Advancements and Save
Game are drawn disabled: `mod_screen_open` refuses while another screen is open
and the pinned API opens no stats, advancements or save screen. Every click on
the pause screen is consumed, and Tab/Left/Right are swallowed, so an undrawn
stock button can't run. The controller menu mixins also route the gamepad to
the Legacy menu on `Screen::Paused`.

Tested on Windows, Nightly 5ef7734, a single-player world started from the
launcher's Worlds → Play (the Legacy title has no world list yet, APIs 02–03):
the column draws as in `gui-pause-menu.png`; Resume Game returns to the game;
arrow keys move focus between Resume Game and Exit Game only and Enter
activates; Exit Game → Cancel returns to the list; Exit Game → Exit Game
lands on the title screen, and reopening the world from the launcher found it
saved where it was left. Not tested: a gamepad and a multiplayer server. 28
local tests pass.

## Previous target

Target: Minecraft 26.3, Horizon `0b14f8a`, Mod API 0.2.
Gameplay/census validation build: `native-0b14f8a-b13f98955780`.
Native settings adapter validation build: `native-0b14f8a-21f10fddf2f7`.
Final formatting-only package build: `native-0b14f8a-70df1564f13c` (passed).

## Clear button-list backgrounds

The owner's follow-up explicitly requires transparent surroundings for
button-list menus. Removed the light backing card and its borders from the
shared option-page drawing and the titled button-list drawing. Settings and
skin-option buttons now sit directly over the panorama, with the same button
rectangles, focus treatment and input handling. How To Play remains clear.

Native build `native-0b14f8a-ff73d7b68191` passed. Horizon MCP inspected Game
Options widgets and captured the Game Options and Change Skin pages at
1280×720 (GUI 640×360). Evidence: `gui-game-options-clear.png` and
`gui-skin-options-clear.png`. These supersede the older panel-backed captures
as the intended background treatment. The skin screen's existing static
scaffolding is not made functional by this paint change.

## Requested features

| Feature | Runnable status | Remaining work |
|---|---|---|
| Console creative flight | Not enabled | Published inner travel/speed/collision/replay extension points, then compile and live flight validation; #18811. |
| Legacy elytra momentum | Not enabled | Inner movement integration and flight validation; roll is separately tracked by #18694. Existing Legacy elytra sound is independent of physics. |
| Console mob caps | Natural-spawn subset, opt-in | Breeding/manual/placement caps, animal despawn/protection, category scheduling parity and broader live spawn-boundary tests. |
| Potion gameplay | Preparation only | Vanilla-cauldron durable/synced state rejected at startup (#19095), plus rendering, weather/drip integration and end-to-end transactions. |
| Controller | Movement/look curves and replacement title navigation | Source gameplay button preset, auto-aim, contextual inventory interactions, cursor behavior, gyro and configurable source controls. |

This is a partial implementation. Remaining work is not all proven to require
upstream changes: native replacements can implement more than the ordinary
hooks. In particular, no claim is made that manual mob caps or the remaining
controller interactions are impossible with native code.

## Implemented behavior

- Left stick: source axial 0.25 dead zone, linear per-axis remapping, full
  square diagonals, source smooth-movement option, retained 0.98 input,
  use/crouch slowdowns. Only the local-server path bypasses square reshaping;
  remote play keeps Horizon's ordinary behavior by default.
- Camera: source axial 0.34 dead zone, squared per-axis response, sensitivity
  polynomial and 0.125 scope factor. Uses Horizon's configured controller
  sensitivity and inversions, integrates the native frame delta and retains
  its hitch limit. Source polling timing, linear-camera setting and gyro are
  not ported. Focus/menu/radial guards remain intact.
- Replacement title menus accept D-pad/stick directions, South/confirm,
  East/Start/back and bumper tab navigation. Horizon's gameplay bindings and
  stock controller guide remain unchanged.
- Source natural bucket limits and generic per-type half-limits, including
  ghast 4, zombified piglin 40 and End enderman difficulty limits. Counts use
  loaded entity handles. Unknown candidate categories fail open. Custom
  spawners that do not pass the observed natural category/position hooks are
  not claimed covered. The vanilla 400-tick creature scheduling remains.
- Existing entities are counted, never deleted to enforce a quota. The rule
  defaults off. While enabled, its Overworld creature gate is temporarily
  1,000,000 so ordinary creature caps do not preempt the world-wide checks.
  Opt-out restores the saved value when it is still this mod's override.
  Other spawn-category limits remain intact.
- Display HUD/outline, hand, system/death/advancement messages save in
  `config/legacy/interface.json`. Normal player chat is retained.
- Exit confirmation uses the native event loop's normal shutdown path.
- Mouse X/Y inversion, discrete scrolling, auto jump and view bobbing use
  Horizon's validated `options-set` engine path, including application/save.
  Controller Invert Look changes the typed controller setting and uses the
  normal settings-changed handler. These six rows accept mouse and
  keyboard/controller activation; ordinary settings sliders remain unfinished.

## Verification

23 local tests passed. Source-derived flight arithmetic and potion transitions
are **preparation tests only**, excluded from production; they do not prove
those features work in game. Native package build and launch succeeded.

Horizon MCP, disposable instance `legacy-horizon-gui-test`, world
`legacy-horizon-parity-test`, isolated 1280×720 display:

- Left stick 0.20: stationary at `[0.5,121,0.5]`; 0.625: moved with measured
  horizontal velocity `-0.0589295325`. Tool delays make held-input durations
  unsuitable for exact displacement/rate assertions.
- Right stick 0.30: yaw remained 0, pitch 0; 0.67: yaw changed while position
  and pitch stayed fixed. The squared curve/scope ratio are checked in tests;
  exact live turn-rate and spyglass behavior remain to be measured.
- D-pad Down twice + South opened Help & Options; East returned to root.
  Back also worked through multiple nested option pages.
- Eight D-pad steps scrolled the Controller page to Smooth Movement. South
  changed it ON→OFF→ON; the labels and saved state followed the activations.
- Live census with rule ON: Animals 83/50, Chickens 19/8, Wolves 11/8 and
  creature gate 1,000,000. These existing excess mobs were not removed.
  Toggling OFF restored gate 10. Population reads/rule state/restoration were
  observed live; individual natural-spawn denial boundaries are unit-tested,
  not yet exhaustively exercised live for every species/dimension.
- HUD OFF removed the HUD while the hand/held block remained; it was restored
  ON using the menu. Quit confirmation exited the client successfully.
- Menu clicks changed mouse X/Y inversion, discrete scrolling and auto jump
  false→true, bobbing true→false and controller inversion false→true. MCP
  `options-get` reported all five vanilla changes, and both normal settings
  files contained the saved values. `gui-mouse-options-progress.png` and
  `gui-game-options-progress.png` capture these actual temporary test values.
- After a full client restart, MCP reported all five vanilla values unchanged
  and the controller menu read Invert Look ON. D-pad/South toggled controller
  inversion back OFF. Controller activation also toggled bobbing true→false→true
  through the native setter. All six test values were restored afterwards.

Screenshots: `gui-controller-progress.png`, `gameplay-hud-off.png` and
`gameplay-controller-look.png`.
They are actual game captures, not visual proof of flight, potion or full
controller GUI parity. The controller guide visible in the latter is still
Horizon's stock guide.

## Rejected experiment

Potion-cauldron code compiled but registering a saved block entity on
`minecraft:water_cauldron` caused startup rejection:
`mod legacy: block entity type legacy:water_cauldron cannot own vanilla block minecraft:water_cauldron`.
It was removed from registration and archived as `.rs.txt`, so the running
mod no longer triggers that failure. The archive has not received complete
server, persistence or multiplayer testing.
