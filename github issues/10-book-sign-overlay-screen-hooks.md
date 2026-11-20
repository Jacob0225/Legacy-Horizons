# mod API: expose book and sign editor overlays to screen render/input hooks

> 🤖 Filed by Jacob0225's AI agent (Codex, GPT-5), not by Jacob0225 personally.

## Summary

Horizon exposes render and input hooks for ordinary game screens and title modes, but explicitly excludes the book view/edit/sign overlays and sign editors. A client mod therefore cannot restyle or replace those interfaces.

This blocks GUI parity for Legacy4J. Its Legacy Console UI replaces book viewing, book-and-quill editing/signing, standing-sign editing and hanging-sign editing. The port can detect a sign editor through `ClientHost::sign_editor`, but it has no render stage or input event in which to draw and operate the replacement.

## Tested environment

- Horizon commit: `0b14f8acfd97fa2c750e42795249dcc981178ca4`
- Minecraft data: 26.3
- Mod SDK / shader API: 0.2 nightly from 2026-11-18
- Native Rust `.hmod`, built by `horizon-launcher native-build`

## Current API

- `screen_render_pre`, background, foreground and post cover Horizon's ordinary `Screen` list and title modes.
- Screen mouse, wheel, key, character and preedit hooks follow those covered screens.
- The API notes state that books and sign editors are overlays drawn over gameplay and do not receive these hooks.
- `ClientHost::sign_editor()` provides a read-only description of the edited sign, position and side, but no draw/input lifecycle.

There is no equivalent public `ScreenView` or overlay view for a book, nor a cancellable rendering/input surface for either family.

## Reproduction

1. Open a written book, a book and quill, or a sign editor.
2. Implement `screen_render_pre`/`screen_render_post` and the screen input hooks in a client mod.
3. Try to hide the built-in overlay, draw a replacement frame, and handle its buttons/text navigation.

None of those screen hooks fires for the book/sign overlay. The mod cannot safely replace its visuals or consume the corresponding input.

## Expected

Expose these editor/view overlays through the same screen lifecycle where safe, with distinct stable ids such as:

- `book_view`
- `book_edit`
- `book_sign`
- `sign_edit`
- `hanging_sign_edit`

The render-pre hook should be cancellable so a total UI replacement can suppress Horizon's normal drawing, while post allows additive overlays. Mouse, scroll, key, character and preedit events should identify the same overlay and preserve the editor's normal save/close semantics unless consumed.

A dedicated overlay hook family is also acceptable if these cannot share `ScreenView`, provided it has equivalent render/input coverage and a stable view of the editor state.

## Actual

The Legacy4J port must leave Horizon's book and sign interfaces unchanged. Drawing from a frame or HUD hook would not provide the overlay's bounds, text focus or cancellable input, and could leave two interactive interfaces on top of each other.

## Existing issue check

- #5543 and its completed screen-hook issues added drawing and input on ordinary open screens, especially containers; the documented overlay exclusions remain.
- #6315/#7195 added the title's own modes to screen hooks, not gameplay book/sign overlays.
- #4953 and #6452 track broad native-mod coverage but do not expose these overlays.
- #6270 is the authoritative event after a player finishes a sign; it is not a client editor render/input hook.
- No exact issue for book/sign editor overlay screen hooks was found through GitHub issue search.

## Acceptance criteria

- A client mod can identify book view, book edit, book signing, standing-sign edit and hanging-sign edit independently.
- Render pre/background/foreground/post or equivalent overlay stages fire with GUI bounds and a draw context.
- A total replacement can cancel the built-in overlay drawing without hiding unrelated HUD/toasts.
- Mouse, drag, wheel, key, character and IME preedit hooks reach these overlays and can consume input consistently.
- Unconsumed input retains Horizon's normal page navigation, text editing, save and close behavior.
- The view exposes only client-visible editor state and does not bypass server validation or allow a mod to submit an otherwise invalid edit.
- With no listener, the overlay paths do no allocation and behave exactly as before.
