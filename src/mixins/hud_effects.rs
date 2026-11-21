fn legacy_hud_effects(game: &Game<'_>, batch: &mut HudBatch) {
    let Some((vertices, _)) = legacy_horizon::legacy_hud_end() else { return };
    let width = batch.width;
    let alpha = legacy_horizon::legacy_hud_alpha(game.selected as usize);
    for v in batch.vertices.iter_mut().skip(vertices) {
        v.pos = legacy_horizon::legacy_hud_effect_point(width, v.pos);
        v.color = legacy_horizon::legacy_hud_fade(v.color, alpha);
    }
}
