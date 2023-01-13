use tauri::{Manager, Window};

use crate::try_get_app_handle;

pub fn try_get_window() -> Option<Window> {
    match try_get_app_handle() {
        Some(app_handle) => app_handle.get_window("main"),
        _ => None,
    }
}

pub fn get_window() -> Window {
    try_get_window().expect("Could not get app window.")
}
