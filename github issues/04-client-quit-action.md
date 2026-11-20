# mod API: let a client mod request a graceful game shutdown

## Port follow-up — 2026-11-20

Upstream issue #18389 is closed. The port now has a tested native fallback:
the Exit Game confirmation sets a request consumed by a HEAD replacement at
`Game::idle`, which calls `EventLoopCtl::exit`. Horizon's normal shutdown ran
and the disposable test process exited successfully. The old request below
records the original public-API boundary.

> 🤖 Filed by Jacob0225's AI agent (Codex, GPT-5), not by Jacob0225 personally.

## Summary

A client mod can replace or extend the title menu, but `ClientHost` has no safe equivalent of the title screen's **Quit Game** action. A mod can draw and handle an exit confirmation, but it cannot ask Horizon to save its normal client state and shut down through the same path as Horizon's own button.

This blocks a Legacy4J title-screen port from making its `Exit Game` confirmation functional.

## Tested environment

- Horizon commit: `0b14f8acfd97fa2c750e42795249dcc981178ca4`
- Minecraft data: 26.3
- Mod SDK / shader API: 0.2 nightly from 2026-11-18
- Native Rust `.hmod`, built by `horizon-launcher native-build`

## Reproduction

1. Use the title-screen render and input hooks to replace or add an `Exit Game` button.
2. Draw a confirmation screen and consume its input.
3. Try to perform Horizon's normal graceful quit after confirmation.

`ClientHost` has screen close, inventory, mod-screen and world disconnect surfaces, but no client shutdown request.

Calling `std::process::exit` is not a valid workaround: it bypasses Horizon's normal shutdown path and may skip state/log flushing or integrated-server cleanup. Synthesizing a window close event is not exposed and would couple the mod to platform internals.

## Expected

A narrowly scoped client method requests the same graceful shutdown as Horizon's own title button, for example:

```rust
let accepted = host.request_client_quit();
```

It should return false outside the window's client or when shutdown is already irreversible, and it should not terminate the process synchronously inside the mod hook.

## Actual

The port can display the confirmation accurately, but its confirmed Quit button must remain inert.

## Existing issue check

- #9581/#9601 added client Host services and startup/disconnect/shutdown **notifications**. It does not expose an action that requests client shutdown.
- Agent control's `quit` command proves Horizon already has a graceful internal route, but that command is not a mod API.
- No issue specifically requesting a client-mod quit action was found.

## Acceptance criteria

- A client mod can request the same graceful quit used by Horizon's title menu.
- The request is deferred safely out of the current hook dispatch.
- Normal client settings, logs and any integrated-server state are handled exactly as for Horizon's own Quit button.
- Invalid contexts return false without panicking or abruptly terminating the process.

