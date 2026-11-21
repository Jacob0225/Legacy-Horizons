fn legacy_hud_begin(batch: &HudBatch) {
    legacy_horizon::legacy_hud_begin(batch.vertices.len(), batch.blends.len());
}
