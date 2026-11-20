# mod API: deliver controller navigation and activation to mod-drawn screens

## Port follow-up — 2026-11-20

A native fallback is now implemented and tested against `0b14f8a`:
`Game::pad_menu_button` and `Game::pad_step` queue controller activations to
the port's own title input handler. Horizon MCP verified D-pad navigation,
South activation, East back through nested menus, and scrolling to/toggling
the saved Smooth Movement option. This covers the replacement title pages;
it does not implement the complete Legacy in-world controller interaction
model. The public hook request below remains relevant.

> 🤖 Filed by Jacob0225's AI agent (Codex, GPT-5), not by Jacob0225 personally.

## Summary

A client mod can draw and handle a screen through keyboard and mouse hooks, but it cannot receive or consume Horizon's controller navigation, activation, back or scroll actions.

This blocks functional controller support for mod-owned screens and screen replacements. It is especially visible in a Legacy4J port: the Legacy Console GUI can match the title menu visually and work with keyboard/mouse, but its primary console input path cannot move focus or activate its buttons. Horizon's hidden title screen may still react to the same controller underneath the replacement.

## Tested environment

- Horizon commit: `0b14f8acfd97fa2c750e42795249dcc981178ca4`
- Minecraft data: 26.3
- Mod SDK / shader API: 0.2 nightly from 2026-11-18
- Native Rust `.hmod`, built by `horizon-launcher native-build`

## Current API

- Screen key, character, mouse button, drag and wheel hooks explicitly cover window/keyboard/mouse input, not gamepad input or controller-driven focus clicks.
- `DrawContext::widget_button` can register narration/controller focus metadata, but a mod drawing over an existing title screen cannot observe the controller action that changes or activates its own logical focus.
- The `mod_screen_open` documentation states that a gamepad can close a mod screen but cannot click it.
- Registered key mappings are gameplay mappings; they do not expose screen navigation actions while a GUI is open.

## Reproduction

1. Draw a mod-owned menu, or replace the title screen in `screen_render_pre`/`screen_render_post`.
2. Handle `screen_key_pressed_pre` and `screen_mouse_button_pressed_pre` so the custom buttons work with keyboard and mouse.
3. Connect a controller and use the D-pad/stick plus the confirm button.

The mod receives no equivalent screen event for moving focus or activating the focused custom button. On an overdrawn built-in screen, Horizon may instead navigate or activate the hidden built-in widgets.

## Expected

Expose Horizon's semantic screen actions rather than raw controller hardware. One possible shape is a cancellable hook such as:

```rust
fn screen_action_pre(
    &self,
    host: &dyn ClientHost,
    event: &mut ScreenActionPreEvent<'_>,
) -> Flow {
    // Navigate(Direction), Activate, Back, Scroll(f32), Page(Direction)
}
```

The action should identify its input source when useful, but mods should not need to know Xbox/PlayStation/Switch button numbers. The same semantic surface can serve keyboard focus and future input sources consistently.

For a mod's own screen/widget set, an engine-managed focus/activation bridge would also work if registered mod widgets participate in it and the mod receives their activation.

## Actual

The port has working mouse hitboxes and keyboard focus, but controller navigation cannot operate the custom menu. Mapping controller buttons to synthetic keyboard events inside the mod is not possible through the public API and would bypass Horizon's controller bindings, repeat timing, focus, glyph family and accessibility behavior.

This was also reproduced with Horizon's virtual controller: after opening a mod-drawn How To Play article on the title screen, connecting an Xbox-family pad and sending D-pad Down followed by South left the mod's page and logical selection unchanged. The screen hooks received neither action.

## Existing issue check

- Horizon's controller documentation describes the engine's internal focus navigation and explicitly calls out the native mod screen exception; it does not expose that path to mods.
- #14205 proposes an internal `InputHost`/`InputSource` layer with registered focus targets while moving controller code out of the engine. It is architectural engine work, not a public mod-screen input contract.
- #14383 concerns controller behavior on Horizon's Pack Options page.
- #4953 tracks native-mod API coverage but does not provide controller screen actions.
- No exact issue for controller navigation/activation on mod-drawn screens was found through GitHub issue search.

## Acceptance criteria

- A mod-owned screen can participate in Horizon's normal controller focus order.
- A mod replacing or overlaying an existing screen can receive and consume controller Navigate, Activate, Back and Scroll/Page actions before hidden game widgets act.
- Activation reaches the mod's registered focused widget or a cancellable semantic screen-action hook.
- Keyboard focus navigation can use the same target order without requiring duplicate coordinates.
- The behavior follows the player's controller bindings, repeat timing and accessibility settings.
- The API reports semantic actions, not controller-brand-specific raw button numbers.
- With no listening mod or registered targets, Horizon's existing controller behavior and cost are unchanged.
