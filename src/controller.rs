//! Legacy4J BindingState.Axis and KeyboardControllerInputMixin movement.
use std::{
    cell::Cell,
    sync::atomic::{AtomicBool, Ordering},
};
use std::{collections::VecDeque, sync::Mutex};
static TITLE_ACTIVE: AtomicBool = AtomicBool::new(false);
static MENU_KEYS: Mutex<VecDeque<&'static str>> = Mutex::new(VecDeque::new());
pub fn title_active() -> bool {
    TITLE_ACTIVE.load(Ordering::Relaxed)
}
pub fn set_title_active(active: bool) {
    TITLE_ACTIVE.store(active, Ordering::Relaxed);
    if !active {
        MENU_KEYS.lock().unwrap().clear();
    }
}
pub fn menu_key(key: &'static str) {
    let mut keys = MENU_KEYS.lock().unwrap();
    if keys.len() < 32 {
        keys.push_back(key);
    }
}
pub fn take_menu_key() -> Option<&'static str> {
    MENU_KEYS.lock().unwrap().pop_front()
}
static LOCAL_SERVER: AtomicBool = AtomicBool::new(false);
static SMOOTH: AtomicBool = AtomicBool::new(true);
static INVERT_LOOK: AtomicBool = AtomicBool::new(false);
static INVERT_REQUESTS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
pub fn inverted_look() -> bool {
    INVERT_LOOK.load(Ordering::Relaxed)
}
pub fn report_inverted_look(enabled: bool) {
    INVERT_LOOK.store(enabled, Ordering::Relaxed);
}
pub fn request_invert_look() {
    INVERT_REQUESTS.fetch_add(1, Ordering::Relaxed);
}
pub fn take_invert_request() -> bool {
    INVERT_REQUESTS.swap(0, Ordering::Relaxed) % 2 != 0
}
thread_local! { static LEGACY_INPUT: Cell<bool> = const { Cell::new(false) }; }
pub fn set_server(active: bool) {
    LOCAL_SERVER.store(active, Ordering::Relaxed);
}
pub fn set_smooth(enabled: bool) {
    SMOOTH.store(enabled, Ordering::Relaxed);
}
pub fn local_server() -> bool {
    LOCAL_SERVER.load(Ordering::Relaxed)
}
pub fn set_input(active: bool) {
    LEGACY_INPUT.with(|value| value.set(active));
}
pub fn input_active() -> bool {
    LEGACY_INPUT.with(Cell::get)
}
pub fn vector(stick: [f32; 2]) -> Option<[f32; 2]> {
    if stick.iter().any(|v| !v.is_finite()) {
        return None;
    }
    let deadzone = 0.25_f32;
    let axis = |value: f32| {
        let value = value.clamp(-1.0, 1.0);
        if value.abs() <= deadzone {
            return 0.0;
        }
        if SMOOTH.load(Ordering::Relaxed) {
            (value - value.signum() * deadzone) / (1.0 - deadzone)
        } else {
            value.signum()
        }
    };
    // Horizon's canonical pad Y is positive forwards; physical SDL Y is flipped upstream.
    let result = [-axis(stick[0]), axis(stick[1])];
    (result != [0.0, 0.0]).then_some(result)
}
/// ControllerManager.updatePlayerCamera, expressed as Entity.turn inputs.
/// Integrate elapsed milliseconds, independent of display refresh rate.
pub fn look(stick: [f32; 2], sensitivity: f32, seconds: f32, scoping: bool) -> [f64; 2] {
    if !seconds.is_finite()
        || !sensitivity.is_finite()
        || stick.iter().any(|value| !value.is_finite())
    {
        return [0.0; 2];
    }
    let curve = |value: f32| {
        let value = value.clamp(-1.0, 1.0);
        let smooth = ((value.abs() - 0.34).max(0.0) / 0.66).copysign(value);
        f64::from(smooth * smooth.abs())
    };
    let speed = (f64::from(sensitivity.clamp(0.0, 1.0)) * 0.6 + 0.2).powi(3)
        * 7.5
        * f64::from(seconds.clamp(0.0, 0.1))
        * 1000.0
        * if scoping { 0.125 } else { 1.0 };
    // Horizon's canonical Y is up-positive; Entity.turn pitch is down-positive.
    [curve(stick[0]) * speed, -curve(stick[1]) * speed]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn axial_deadzone_preserves_square_diagonals_and_small_analogue_motion() {
        assert_eq!(vector([0.25, -0.25]), None);
        assert_eq!(vector([1.0, 1.0]), Some([-1.0, 1.0]));
        assert_eq!(vector([0.625, 0.0]), Some([-0.5, 0.0]));
        assert_eq!(vector([f32::NAN, 1.0]), None);
    }
    #[test]
    fn camera_has_axial_deadzone_squared_curve_and_spyglass_slowdown() {
        assert_eq!(look([0.34, -0.34], 0.5, 0.016, false), [0.0, 0.0]);
        let full = look([1.0, 1.0], 0.5, 0.016, false);
        assert!((full[0] - 15.0).abs() < 1e-5);
        assert_eq!(full[1], -full[0]);
        let half = look([0.67, 0.0], 0.5, 0.016, false);
        assert!((half[0] / full[0] - 0.25).abs() < 1e-6);
        assert_eq!(look([1.0, 1.0], 0.5, 0.016, true)[0], full[0] * 0.125);
        assert_eq!(look([1.0, 0.0], 0.5, f32::NAN, false), [0.0; 2]);
    }
}
