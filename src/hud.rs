//! Legacy4J's in-game HUD (`LegacyGuiElements.setup`,
//! `LegacyRenderUtil.prepareHUDRender`): the hotbar, the health rows, the
//! experience bar and the mount's hearts are scaled about the bottom centre by
//! the HUD size, raised by the HUD distance and drawn at the HUD opacity; the
//! effect icons and boss bars take Legacy4J's places.
//!
//! Horizon bakes each HUD layer into vertices, and a pose a mod pushes in
//! `gui_layer_pre` doesn't reach them. The patch tool can't reach into the
//! layers' closures either, so the `legacy_hud_*` mixins in `horizon-mod.toml`
//! sit around those calls in `Game::hud_layers`: [`begin`] notes where the
//! batch stood, and afterwards what the call drew is moved through [`point`]
//! (or [`effect_point`]) and faded by [`alpha`]. The selected item name and the
//! action bar keep their size, as Legacy4J leaves them unscaled, and only rise
//! by [`lift`] so they stay above the larger rows.
//!
//! HUD Size, Interface Opacity and Interface Distance are the User Interface
//! page's settings, saved in `config/legacy/hud.json`.

use mod_api::{ClientHost, ConfigPath};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// The User Interface page's rows these settings are on.
pub const OPACITY_ROW: usize = 4;
pub const SIZE_ROW: usize = 10;
pub const DISTANCE_ROW: usize = 11;

/// `hudScale` (1 to 3, default 2), `hudOpacity` and `hudDistance` in tenths
/// (0 to 10, defaults 0.8 and 1.0).
static SIZE: AtomicU8 = AtomicU8::new(2);
static OPACITY: AtomicU8 = AtomicU8::new(8);
static DISTANCE: AtomicU8 = AtomicU8::new(10);
static LOADED: AtomicBool = AtomicBool::new(false);
static DIRTY: AtomicBool = AtomicBool::new(false);

/// `LegacyGuiElements.lastHotbarSelection` and the time it last changed.
static LAST_SLOT: AtomicUsize = AtomicUsize::new(usize::MAX);
static CHANGED_AT: AtomicU64 = AtomicU64::new(0);

/// `getHUDScale` outside SD and FHD modes: 1, 1.5 or 2.
pub fn scale_for(size: u8) -> f32 {
    match size {
        1 => 1.0,
        3 => 2.0,
        _ => 1.5,
    }
}

/// `getHUDDistance`: `-distance * (22.4 + 17.6)` with in-game control
/// tooltips. Those button prompts can't be drawn yet (API 06), so the gap
/// they sit in is left out.
pub fn distance_for(tenths: u8) -> f32 {
    -f32::from(tenths) / 10.0 * 22.4
}

fn scale() -> f32 {
    scale_for(SIZE.load(Ordering::Relaxed))
}

fn distance() -> f32 {
    distance_for(DISTANCE.load(Ordering::Relaxed))
}

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

/// Legacy4J's `hud.translate*` and `hud.scale*` elements: to
/// `(width / 2, height + distance)`, scaled, then back by `(-width / 2, -height)`.
pub fn point_with(size: [f32; 2], at: [f32; 2], scale: f32, distance: f32) -> [f32; 2] {
    [
        size[0] / 2.0 + (at[0] - size[0] / 2.0) * scale,
        size[1] + distance + (at[1] - size[1]) * scale,
    ]
}

/// Where a point a moved layer draws at (GUI pixels, `size` the GUI size)
/// ends up with the current settings.
pub fn point(size: [f32; 2], at: [f32; 2]) -> [f32; 2] {
    point_with(size, at, scale(), distance())
}

/// How far the selected item name and the action bar rise: as far as the
/// item name's stock line, 59 above the bottom (room for the armor row above
/// the hearts), moves under [`point`], so it stays above the larger rows.
pub fn lift(size: [f32; 2]) -> f32 {
    lift_with(size, scale(), distance())
}

pub fn lift_with(size: [f32; 2], scale: f32, distance: f32) -> f32 {
    point_with(size, [0.0, size[1] - 59.0], scale, distance)[1] - (size[1] - 59.0)
}

/// Legacy4J's effect icons (`renderGuiEffects`): 24 apart from
/// `width - 55 - 24n`, beneficial ones at y = 18 and harmful ones 24 lower,
/// where Horizon draws them 25 apart from `width - 25n`, at y = 1 and 26
/// lower. A point of icon `n` (counted from the right, from 1) moves by
/// `n - 55` across and by 17 or 15 down.
pub fn effect_point(width: f32, at: [f32; 2]) -> [f32; 2] {
    let n = ((width - at[0]) / 25.0).ceil().max(1.0);
    let down = if at[1] < 26.0 { 17.0 } else { 15.0 };
    [at[0] + n - 55.0, at[1] + down]
}

/// Legacy4J's boss bars start at `12 + 16 * distance` instead of 12.
pub fn boss_bar_drop() -> i32 {
    (16.0 * f32::from(DISTANCE.load(Ordering::Relaxed)) / 10.0).round() as i32
}

/// Legacy4J's chat offset (`ChatComponentMixin`): in by the chat safe zone
/// (`29 * distance`) and up by the HUD distance and 42, so the lines clear the
/// raised HUD.
pub fn chat_offset() -> [i32; 2] {
    let tenths = f32::from(DISTANCE.load(Ordering::Relaxed));
    [
        (29.0 * tenths / 10.0).round() as i32,
        (distance() - 42.0).round() as i32,
    ]
}

/// `getHUDOpacity`: at an opacity of 0.8 or less the HUD shows at 0.8 for
/// 3.6 s after the hotbar selection changes (three 1.2 s units), then fades
/// to the setting over the next 1.2 s; above 0.8 it stays at the setting.
pub fn opacity_at(setting: f32, since_change_ms: u64) -> f32 {
    if setting > 0.8 {
        return setting;
    }
    let f = since_change_ms as f32 / 1200.0;
    let rise = if f >= 3.0 { (4.0 - f).max(0.0) } else { 1.0 };
    (setting + (1.0 - setting) * rise).min(0.8)
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as u64)
}

/// The moved layers' alpha this frame, `selected` the hotbar slot held.
pub fn alpha(selected: usize) -> f32 {
    let now = now_ms();
    let last = LAST_SLOT.swap(selected, Ordering::Relaxed);
    if last != usize::MAX && last != selected {
        CHANGED_AT.store(now, Ordering::Relaxed);
    }
    let since = now.saturating_sub(CHANGED_AT.load(Ordering::Relaxed));
    opacity_at(f32::from(OPACITY.load(Ordering::Relaxed)) / 10.0, since)
}

/// A packed HUD colour (alpha in the top byte) at `alpha` of its own.
pub fn fade(rgba: u32, alpha: f32) -> u32 {
    let a = ((rgba >> 24) as f32 * alpha.clamp(0.0, 1.0)).round() as u32;
    (rgba & 0x00FF_FFFF) | (a << 24)
}

/// The row's label value, for the User Interface page.
pub fn label(row: usize) -> Option<String> {
    match row {
        OPACITY_ROW => Some(format!(
            "{}%",
            u32::from(OPACITY.load(Ordering::Relaxed)) * 10
        )),
        SIZE_ROW => Some(SIZE.load(Ordering::Relaxed).to_string()),
        DISTANCE_ROW => Some(format!(
            "{}%",
            u32::from(DISTANCE.load(Ordering::Relaxed)) * 10
        )),
        _ => None,
    }
}

/// The next value of a row, as a click or Enter steps it: opacity and
/// distance by 10% up to 100% and back to 0, the size 1, 2, 3 and back.
pub fn step(row: usize) -> bool {
    let (setting, max, min) = match row {
        OPACITY_ROW => (&OPACITY, 10, 0),
        SIZE_ROW => (&SIZE, 3, 1),
        DISTANCE_ROW => (&DISTANCE, 10, 0),
        _ => return false,
    };
    let next = match setting.load(Ordering::Relaxed) {
        value if value >= max => min,
        value => value + 1,
    };
    setting.store(next, Ordering::Relaxed);
    DIRTY.store(true, Ordering::Relaxed);
    true
}

/// Loads the settings once and saves them after a change.
pub fn sync(host: &dyn ClientHost) {
    if LOADED.load(Ordering::Relaxed) && !DIRTY.load(Ordering::Relaxed) {
        return;
    }
    let path = ConfigPath::new("legacy/hud.json").expect("owned config path");
    let files = host.config_files();
    if !LOADED.swap(true, Ordering::Relaxed) && !DIRTY.load(Ordering::Relaxed) {
        if let Ok(bytes) = files.read(&path)
            && let Ok(value) = serde_json::from_slice::<serde_json::Value>(&bytes)
        {
            let read = |key: &str, min: u64, max: u64, setting: &AtomicU8| {
                if let Some(v) = value[key].as_u64() {
                    setting.store(v.clamp(min, max) as u8, Ordering::Relaxed);
                }
            };
            read("hud_size", 1, 3, &SIZE);
            read("hud_opacity", 0, 10, &OPACITY);
            read("hud_distance", 0, 10, &DISTANCE);
        }
    }
    if DIRTY.swap(false, Ordering::Relaxed) {
        let value = serde_json::json!({
            "hud_size": SIZE.load(Ordering::Relaxed),
            "hud_opacity": OPACITY.load(Ordering::Relaxed),
            "hud_distance": DISTANCE.load(Ordering::Relaxed),
        });
        let bytes = serde_json::to_vec_pretty(&value).expect("integer config");
        if let Err(error) = files.write(&path, &bytes) {
            eprintln!("Legacy Horizon: could not save HUD options: {error:?}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hotbar_grows_about_the_bottom_centre_and_rises() {
        let size = [640.0, 360.0];
        let (s, d) = (scale_for(2), distance_for(10));
        // The stock hotbar: 182 wide, from y = height - 22.
        assert_eq!(point_with(size, [229.0, 338.0], s, d), [183.5, 304.6]);
        assert_eq!(point_with(size, [411.0, 360.0], s, d), [456.5, 337.6]);
        // The centre column stays put horizontally.
        assert_eq!(point_with(size, [320.0, 0.0], s, d)[0], 320.0);
    }

    #[test]
    fn sizes_and_distances_follow_legacy4j() {
        assert_eq!([scale_for(1), scale_for(2), scale_for(3)], [1.0, 1.5, 2.0]);
        assert_eq!(distance_for(0), 0.0);
        assert!((distance_for(5) + 11.2).abs() < 1e-5);
        // Size 1 at distance 0 is the stock HUD.
        assert_eq!(
            point_with([640.0, 360.0], [229.0, 338.0], 1.0, 0.0),
            [229.0, 338.0]
        );
    }

    #[test]
    fn the_item_name_stays_above_the_armor_row() {
        let size = [426.0, 240.0];
        for (s, d) in [
            (scale_for(2), distance_for(10)),
            (scale_for(3), distance_for(5)),
        ] {
            // The name's line ends up at least a row above the armor row's top
            // (49 above the bottom, moved).
            let name = size[1] - 59.0 + lift_with(size, s, d);
            let armor = point_with(size, [0.0, size[1] - 49.0], s, d)[1];
            assert!(armor - name >= 10.0, "{name} {armor}");
        }
        assert!((lift_with(size, scale_for(2), distance_for(10)) + 51.9).abs() < 1e-4);
    }

    #[test]
    fn effect_icons_take_legacy4j_places() {
        let w = 640.0;
        // Horizon's first beneficial icon spans 615..639 at y 1..25.
        assert_eq!(effect_point(w, [615.0, 1.0]), [561.0, 18.0]);
        assert_eq!(effect_point(w, [639.0, 25.0]), [585.0, 42.0]);
        // The second, 25 further left, lands 24 further left.
        assert_eq!(effect_point(w, [590.0, 1.0]), [537.0, 18.0]);
        // A harmful icon (y 27) lands at 18 + 24.
        assert_eq!(effect_point(w, [615.0, 27.0]), [561.0, 42.0]);
    }

    #[test]
    fn opacity_shows_at_0_8_after_a_slot_change_then_fades() {
        assert_eq!(opacity_at(0.8, u64::MAX), 0.8);
        assert_eq!(opacity_at(1.0, 0), 1.0);
        assert_eq!(opacity_at(0.4, 0), 0.8);
        assert_eq!(opacity_at(0.4, 3_000), 0.8);
        // Halfway through the fade: 0.4 + 0.6 * 0.5.
        assert!((opacity_at(0.4, 4_200) - 0.7).abs() < 1e-5);
        assert_eq!(opacity_at(0.4, 10_000), 0.4);
    }

    #[test]
    fn fade_scales_only_the_alpha_byte() {
        assert_eq!(fade(0xFF11_2233, 0.8), 0xCC11_2233);
        assert_eq!(fade(0x8011_2233, 1.0), 0x8011_2233);
        assert_eq!(fade(0xFF11_2233, 0.0), 0x0011_2233);
    }

    #[test]
    fn rows_step_and_wrap() {
        // Labels and steps of the three rows only; other rows aren't theirs.
        assert!(label(OPACITY_ROW).is_some() && label(SIZE_ROW).is_some());
        assert!(label(0).is_none());
        assert!(!step(0));
    }
}
