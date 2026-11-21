fn legacy_hud_end(batch: &mut HudBatch) {
    let Some((vertices, blends)) = legacy_horizon::legacy_hud_end() else { return };
    let (size, scale) = ([batch.width, batch.height], batch.scale.max(f32::EPSILON));
    for v in batch.vertices.iter_mut().skip(vertices) {
        v.pos = legacy_horizon::legacy_hud_point(size, v.pos);
    }
    for (_, blend) in batch.blends.iter_mut().skip(blends) {
        if let horizon_client::hud::Blend::Depth(clip) | horizon_client::hud::Blend::DepthGlint(clip) = blend {
            let a = legacy_horizon::legacy_hud_point(size, [clip[0] as f32 / scale, clip[1] as f32 / scale]);
            let b = legacy_horizon::legacy_hud_point(size, [(clip[0] + clip[2]) as f32 / scale, (clip[1] + clip[3]) as f32 / scale]);
            let (x0, y0) = ((a[0] * scale).max(0.0) as u32, (a[1] * scale).max(0.0) as u32);
            *clip = [x0, y0, ((b[0] * scale).max(0.0) as u32).saturating_sub(x0), ((b[1] * scale).max(0.0) as u32).saturating_sub(y0)];
        }
    }
}
