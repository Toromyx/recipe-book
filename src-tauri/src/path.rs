use std::{fs::create_dir_all, path::PathBuf};

use crate::get_app_handle;

pub fn app_data_dir() -> PathBuf {
    let mut dir = match get_app_handle().path_resolver().app_data_dir() {
        Some(some) => some,
        None => panic!("Could not get data directory."),
    };
    if cfg!(debug_assertions) || cfg!(test) {
        let env = match cfg!(test) {
            true => ".TEST",
            false => ".DEVELOPMENT",
        };
        dir.push(env);
    }
    if let Err(err) = create_dir_all(&dir) {
        panic!("Could not create app data directory: {err}");
    }
    dir
}
