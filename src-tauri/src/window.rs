//! This module provides method to handle the tauri windows.

use tauri::{Manager, Window};

use crate::app_handle::get_app_handle;

/// Try to get the main window.
///
/// Returns [`None`] when the window with label `main` does not exist.
pub fn try_get_window() -> Option<Window> {
    get_app_handle().get_window("main")
}

/// Get the main window.
///
/// # Panics
///
/// This function panics when [`try_get_window`] returns [`None`].
pub fn get_window() -> Window {
    try_get_window().expect("Could not get app window.")
}
