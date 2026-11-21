//! Legacy4J's in-game HUD placement (`LegacyGuiElements.setup`,
//! `LegacyRenderUtil.prepareHUDRender`): the hotbar, the health rows and the
//! experience bar are scaled about the bottom centre by the HUD size and
//! raised by the HUD distance.
//!
//! Horizon bakes each HUD layer into vertices, and a pose a mod pushes in
//! `gui_layer_pre` doesn't reach them. The patch tool can't reach into the
//! layers' closures either, so the `legacy_hud_*` mixins in `horizon-mod.toml`
//! sit around those `gui_layer` calls in `Game::hud_layers`: [`begin`] notes
//! where the batch stood, and afterwards what the layer drew is moved through
//! [`point`]. The selected item name and the action bar keep their size, as
//! Legacy4J leaves them unscaled, and only rise by [`lift`] so they stay above
//! the larger rows.

use std::sync::Mutex;

/// `getHUDScale` for the default HUD size (2) outside SD and FHD modes.
pub const HUD_SCALE: f32 = 1.5;

/// `getHUDDistance` for the default distance (1.0): `-(22.4 + 17.6)` with
/// in-game control tooltips. Those button prompts can't be drawn yet (API 06),
/// so the gap they sit in is left out.
pub const HUD_DISTANCE: f32 = -22.4;

/// The HUD batch's vertex and blend-run counts before a moved layer.
static STARTED: Mutex<Option<(usize, usize)>> = Mutex::new(None);

/// Before a moved layer (`legacy_hud_*_begin`).
pub fn begin(vertices: usize, blends: usize) {
    *STARTED
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some((vertices, blends));
}

/// After it (`legacy_hud_*_end`): where its vertices and blend runs start.
pub fn end() -> Option<(usize, usize)> {
    STARTED
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .take()
}

/// Where a point a moved layer draws at (GUI pixels, `size` the GUI size)
/// ends up: Legacy4J's `hud.translate*` and `hud.scale*` elements, to
/// `(width / 2, height + distance)`, scaled, then back by `(-width / 2, -height)`.
pub fn point(size: [f32; 2], at: [f32; 2]) -> [f32; 2] {
    [
        size[0] / 2.0 + (at[0] - size[0] / 2.0) * HUD_SCALE,
        size[1] + HUD_DISTANCE + (at[1] - size[1]) * HUD_SCALE,
    ]
}

/// How far the selected item name and the action bar rise: as far as the
/// top of the stock health rows, 39 above the bottom, moves under [`point`],
/// so they keep their stock gap above the larger rows.
pub fn lift(size: [f32; 2]) -> f32 {
    point(size, [0.0, size[1] - 39.0])[1] - (size[1] - 39.0)
}

/// Legacy4J's chat offset (`ChatComponentMixin`): in by the chat safe zone
/// (29 at the default distance) and up by the HUD distance and 42, so the
/// lines clear the raised HUD.
pub fn chat_offset() -> [i32; 2] {
    [29, (HUD_DISTANCE - 42.0).round() as i32]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hotbar_grows_about_the_bottom_centre_and_rises() {
        let size = [640.0, 360.0];
        // The stock hotbar: 182 wide, from y = height - 22.
        assert_eq!(point(size, [229.0, 338.0]), [183.5, 304.6]);
        assert_eq!(point(size, [411.0, 360.0]), [456.5, 337.6]);
        // The centre column stays put horizontally.
        assert_eq!(point(size, [320.0, 0.0])[0], 320.0);
    }

    #[test]
    fn the_item_name_clears_the_larger_rows() {
        // The hearts' top moves from 39 to 80.9 above the bottom.
        assert!((lift([640.0, 360.0]) + 41.9).abs() < 1e-4);
        assert!((lift([426.0, 240.0]) + 41.9).abs() < 1e-4);
    }

    #[test]
    fn chat_moves_in_and_up() {
        assert_eq!(chat_offset(), [29, -64]);
    }

    #[test]
    fn the_hearts_row_stays_on_screen_at_small_gui_sizes() {
        // The hearts' top edge, 39 above the bottom at a 320x240 GUI.
        let top = point([320.0, 240.0], [160.0, 201.0])[1];
        assert!(top > 0.0 && top < 240.0, "{top}");
    }
}
