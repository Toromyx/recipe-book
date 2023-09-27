//! This module is a wrapper for [`tauri::PathResolver`], resolving different paths dependent on whether the binary was compiled with `debug_assertions`, `test`, or not.

use std::{fs::create_dir_all, path::PathBuf};

use crate::app_handle::get_app_handle;

macro_rules! mutable_or_immutable {
    ($var_name:ident, $var_value:expr) => {
        #[cfg(any(debug_assertions, test))]
        let mut $var_name = $var_value;

        #[cfg(not(any(debug_assertions, test)))]
        let $var_name = $var_value;
    };
}

/// Get the application's data directory.
///
/// # Panics
///
/// This function panics...
/// - ...when [`tauri::PathResolver::app_data_dir`] does return [`None`].
/// - ...when the directory cannot be created with [`create_dir_all`].
pub fn app_data_dir() -> PathBuf {
    mutable_or_immutable!(
        dir,
        match get_app_handle().path_resolver().app_data_dir() {
            Some(some) => some,
            None => panic!("Could not get data directory."),
        }
    );
    #[cfg(any(debug_assertions, test))]
    {
        #[cfg(test)]
        let env = ".TEST";
        #[cfg(not(test))]
        let env = ".DEVELOPMENT";
        dir.push(env);
    }
    if let Err(err) = create_dir_all(&dir) {
        panic!("Could not create app data directory: {err}");
    }
    dir
}
