# Gameplay port status — 2026-11-20

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

Not yet checked: a launcher native build and in-game run on 5ef7734. The
validation builds below are all from `0b14f8a`.

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
