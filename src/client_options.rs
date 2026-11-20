//! Requests applied on the native client loop through Horizon's validated
//! options setter, including its save and settings-change side effects.
use std::{collections::VecDeque, sync::Mutex};

static CHANGES: Mutex<VecDeque<(&'static str, bool)>> = Mutex::new(VecDeque::new());

pub fn toggle(key: &'static str, current: bool) {
    let mut changes = CHANGES.lock().unwrap();
    if let Some((_, pending)) = changes.iter_mut().find(|(name, _)| *name == key) {
        *pending = !*pending;
    } else if changes.len() < 16 {
        changes.push_back((key, !current));
    }
}

pub fn take() -> Option<(&'static str, bool)> {
    CHANGES.lock().unwrap().pop_front()
}
