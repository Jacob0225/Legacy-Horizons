fn legacy_hud_lift(batch: &mut HudBatch) {
    let Some((vertices, _)) = legacy_horizon::legacy_hud_end() else { return };
    let lift = legacy_horizon::legacy_hud_lift([batch.width, batch.height]);
    for v in batch.vertices.iter_mut().skip(vertices) {
        v.pos[1] += lift;
    }
}
