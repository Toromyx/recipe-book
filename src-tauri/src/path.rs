use std::{fs::create_dir_all, path::PathBuf};

use tauri::api::path;

use crate::get_app_handle;

pub fn app_data_dir() -> PathBuf {
    let mut dir = match path::app_data_dir(&get_app_handle().config()) {
        Some(some) => some,
        None => panic!("Could not get data directory."),
    };
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
