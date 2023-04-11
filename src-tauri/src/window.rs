//! This module provides method to handle the tauri windows.

use tauri::{Manager, Window};

use crate::try_get_app_handle;

/// Try to get the main window.
///
/// Returns [`None`] when the [`try_get_app_handle`] returns [`None`].
pub fn try_get_window() -> Option<Window> {
    match try_get_app_handle() {
        Some(app_handle) => app_handle.get_window("main"),
        _ => None,
    }
}

/// Get the main window.
///
/// # Panics
///
/// This function panics when [`try_get_window`] returns [`None`].
pub fn get_window() -> Window {
    try_get_window().expect("Could not get app window.")
}
