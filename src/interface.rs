//! Local interface switches supported by Horizon's render hooks.
use mod_api::{ClientHost, ConfigPath};
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Default)]
pub struct Options {
    hide_hand: AtomicBool,
    loaded: AtomicBool,
    dirty: AtomicBool,
}

impl Options {
    pub fn display_hand(&self) -> bool {
        !self.hide_hand.load(Ordering::Relaxed)
    }

    pub fn toggle_hand(&self) {
        self.hide_hand.fetch_xor(true, Ordering::Relaxed);
        self.dirty.store(true, Ordering::Relaxed);
    }

    pub fn sync(&self, host: &dyn ClientHost) {
        if self.loaded.load(Ordering::Relaxed) && !self.dirty.load(Ordering::Relaxed) {
            return;
        }
        let path = ConfigPath::new("legacy/interface.json").expect("owned config path");
        let files = host.config_files();
        if !self.loaded.swap(true, Ordering::Relaxed) && !self.dirty.load(Ordering::Relaxed) {
            if let Ok(bytes) = files.read(&path) {
                if let Ok(value) = serde_json::from_slice::<serde_json::Value>(&bytes) {
                    self.hide_hand.store(
                        !value["display_hand"].as_bool().unwrap_or(true),
                        Ordering::Relaxed,
                    );
                }
            }
        }
        if self.dirty.swap(false, Ordering::Relaxed) {
            let bytes = serde_json::to_vec_pretty(
                &serde_json::json!({"display_hand": self.display_hand()}),
            )
            .expect("boolean config");
            if let Err(error) = files.write(&path, &bytes) {
                eprintln!("Legacy Horizon: could not save interface options: {error:?}");
            }
        }
    }
}
