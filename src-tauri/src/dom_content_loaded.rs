//! This module implements a [`Condvar`] to wait for the frontend to be loaded.
//!
//! This is done via the `DOMContentLoaded` event of the frontend.

use std::sync::{Condvar, Mutex};

use tauri::{App, Manager};

static LOADED_MUTEX: Mutex<bool> = Mutex::new(false);

static LOADED_CONDVAR: Condvar = Condvar::new();

/// Setup this module to react to the `DOMContentLoaded` event of the frontend.
///
/// This event must be explicitly sent, it is not automatically sent!
pub fn setup(app: &mut App) {
    app.get_window("main")
        .unwrap()
        .listen("DOMContentLoaded", |_| {
            log::debug!("DOMContentLoaded");
            let mut loaded = LOADED_MUTEX.lock().unwrap();
            *loaded = true;
            LOADED_CONDVAR.notify_all();
        });
}

/// Block the current thread until the frontend is loaded.
///
/// See [`setup`] for the wake-up logic.
pub fn await_dom_content_loaded() {
    let mut loaded = LOADED_MUTEX.lock().unwrap();
    while !*loaded {
        loaded = LOADED_CONDVAR.wait(loaded).unwrap();
    }
}
