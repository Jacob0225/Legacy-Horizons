//! Test-only source-derived preparation; this enables no flight physics.
use std::{collections::HashMap, sync::Mutex};
static SPRINT: Mutex<Option<HashMap<i32, u8>>> = Mutex::new(None);
pub fn sprint_tick(id: i32, sprinting: bool) -> f32 {
    let mut guard = SPRINT.lock().unwrap();
    let ticks = guard
        .get_or_insert_with(HashMap::new)
        .entry(id)
        .or_default();
    *ticks = if sprinting { (*ticks + 1).min(10) } else { 0 };
    (*ticks as f32 / 10.0).powi(2)
}
pub fn vertical(
    jump_strength: f64,
    jump_boost: Option<i32>,
    ascending: bool,
    flying_speed: f32,
) -> f64 {
    let speed = f64::from(flying_speed / 0.05_f32);
    let amount = if ascending {
        jump_strength
            + f64::from(jump_boost.map_or(0.0, |amplifier| 0.1_f32 * (amplifier + 1) as f32))
    } else {
        -0.57
    };
    amount * speed
}
pub fn direction(input: [f32; 2], yaw: f32) -> [f32; 2] {
    let [x, z] = input.map(f64::from);
    let length = x.hypot(z);
    if length * length < 1.0e-7 {
        return [0.0, 0.0];
    }
    let step = std::f64::consts::FRAC_PI_4;
    // Java Math.round is floor(x + 0.5), including negative half values.
    let angle = (z.atan2(x) / step + 0.5).floor() * step;
    let desired_yaw = (f64::from(yaw) / 45.0 + 0.5).floor() * 45.0;
    let (sin, cos) = (desired_yaw - f64::from(yaw)).to_radians().sin_cos();
    let (x, z) = (length * angle.cos(), length * angle.sin());
    [(x * cos - z * sin) as f32, (z * cos + x * sin) as f32]
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sprint_ramp_is_quadratic_and_resets() {
        assert!((sprint_tick(-991, true) - 0.01).abs() < 1e-6);
        for _ in 1..10 {
            sprint_tick(-991, true);
        }
        assert_eq!(sprint_tick(-991, true), 1.0);
        assert_eq!(sprint_tick(-991, false), 0.0);
    }
    #[test]
    fn vertical_and_direction_match_source() {
        assert!((vertical(0.42, None, true, 0.05) - 0.42).abs() < 1e-6);
        assert!((vertical(0.42, Some(1), true, 0.1) - 1.24).abs() < 1e-6);
        assert!((vertical(0.42, None, false, 0.05) + 0.57).abs() < 1e-6);
        assert_eq!(direction([0.0, 0.0], 22.5), [0.0, 0.0]);
        let v = direction([0.0, 0.5], 22.5);
        assert!((v[0] + 0.1913417).abs() < 1e-6);
        assert!((v[1] - 0.4619398).abs() < 1e-6);
    }
}
