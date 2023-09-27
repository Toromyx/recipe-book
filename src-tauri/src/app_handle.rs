use std::sync::{Condvar, Mutex};

use tauri::{App, AppHandle};

static APP_HANDLE_MUTEX: Mutex<Option<AppHandle>> = Mutex::new(None);

static APP_HANDLE_CONDVAR: Condvar = Condvar::new();

/// Setup the static app handle and notify waiting threads.
pub fn setup(app: &mut App) {
    let mut app_handle = APP_HANDLE_MUTEX.lock().unwrap();
    *app_handle = Some(app.handle());
    APP_HANDLE_CONDVAR.notify_all();
}

/// Get the app handle but block the current thread until it is available.
///
/// The app handle is made available via [`setup`].
pub fn get_app_handle() -> AppHandle {
    let mut app_handle = APP_HANDLE_MUTEX.lock().unwrap();
    while app_handle.is_none() {
        app_handle = APP_HANDLE_CONDVAR.wait(app_handle).unwrap();
    }
    app_handle.as_ref().unwrap().clone()
}
