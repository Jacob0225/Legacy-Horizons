# mod API: transform controller movement and look before input shaping

> 🤖 Filed by Jacob0225's AI agent (Codex, GPT-6), not by Jacob0225 personally.

Submitted: https://github.com/horizon-rust/horizon/issues/19106

## Port and working native fallback

Legacy4J → Legacy Horizons, API 0.2, Minecraft 26.3, pinned Horizon
`0b14f8acfd97fa2c750e42795249dcc981178ca4`.

This is a hook request accompanying a **working native fallback**, not a claim
that native mods cannot implement it. The port builds replacements for:

- `horizon_client::game::pad::Game<'_>::pad_move`;
- `horizon_client_input::input::modify_input`;
- `horizon_client::game::pad::Game<'_>::pad_look`.

Horizon MCP verified no left-stick motion at 0.20, movement at 0.625, no
camera turn at right-stick 0.30, and camera turn at 0.67. The port's local
tests cover source-derived curve values and diagonal behavior.

## Why the standard hook is needed

Legacy4J uses a 0.25 **axial** movement dead zone and a 0.34 axial camera
dead zone, independently remapping each axis. Camera response squares each
axis, scales by its sensitivity polynomial and elapsed time, and uses 0.125
spyglass slowdown. Smooth movement avoids vanilla's final square-movement
reshaping, while retaining item-use and crouch slowdown. Its default keeps
vanilla movement constraints on remote servers.

The port currently needs native replacements both before and after the
stock controller movement shaping. Reading keyboard mappings/input options
does not provide those controller-axis transforms.

## Requested surface and verification

Expose controller movement/look transformation points carrying canonical
axes, effective sensitivity/inversion, frame elapsed time, scope state and
local/remote context. A movement override also needs a defined way to bypass
the final square-movement transform without bypassing use/crouch slowdown.
Unmodified handlers should preserve current Horizon behavior. Keep menu,
radial-capture, controller-disabled and focus guards intact.

Verify independent-axis dead zones, half tilt, full diagonal, item-use/crouch
slowdown, scope slowdown, no keyboard-only regression and remote default
behavior. This concerns input processing, not collision/flight physics
(#18811), UI navigation (#18647), glyph drawing (#18396), or gyroscope APIs.

Duplicate searches across all states: `"mod API" controller movement` and
`"mod API" controller axis`. The related results above do not expose this
input-processing decision.
