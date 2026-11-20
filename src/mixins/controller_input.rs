fn legacy_controller_input(input: [f32; 2], item_use: Option<f32>, sneaking: Option<f32>) -> [f32; 2] {
    if input[0] * input[0] + input[1] * input[1] == 0.0 { return input; }
    let mut v = [input[0] * 0.98, input[1] * 0.98];
    if let Some(multiplier) = item_use { v = [v[0] * multiplier, v[1] * multiplier]; }
    if let Some(multiplier) = sneaking { v = [v[0] * multiplier, v[1] * multiplier]; }
    if legacy_horizon::legacy_controller_input_active() { v } else { square_movement(v) }
}
