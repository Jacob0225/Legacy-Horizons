# Horizon blocker: title-screen GUI textures from native mods

Legacy Horizon's native package contains the original Legacy4J GUI resources,
including:

- `assets/legacy/textures/gui/title/panorama_day.png`
- `assets/legacy/textures/gui/title/panorama_night.png`
- the other `assets/legacy/textures/gui/**` resources

The package is accepted at runtime and its `data/legacy/**` files are present in
the generated content overlay. The native hooks also run. However, calls such as

```rust
draw.sprite("legacy:gui/title/panorama_day", source, destination, None)
```

return no drawable sprite on the title screen. Horizon consequently leaves its
own panorama visible. Its built-in `gui/title/edition` sprite is also deliberately
replaced with "Rust Edition", so it cannot be used as Legacy4J's Java Edition
banner.

## Capability needed from Horizon

Please make GUI textures bundled in an installed native mod available in the
initial title-screen `HudAtlas`, under the documented key
`<namespace>:gui/<path>`, before `screen_render_post` first fires. The same must
remain true after a resource reload.

An alternative API that supplies an immutable RGBA image/sprite during
`client_setup` would also unblock this, but the current `mod_image` access is
world-only and therefore cannot populate title-screen artwork.

## Reproduction

1. Native-build this project for Horizon commit
   `0b14f8acfd97fa2c750e42795249dcc981178ca4`.
2. Open the title screen.
3. The Legacy buttons and vanilla logo render, proving the hook is active.
4. The `legacy:gui/title/panorama_day` sprite does not render; Horizon's own
   panorama remains visible.

Once Horizon provides this capability, the already-translated scrolling
panorama code in `src/lib.rs` can render the original Legacy4J asset directly.
