use std::{fs::create_dir_all, path::PathBuf};

use crate::get_app_handle;

macro_rules! mutable_or_immutable {
    ($var_name:ident, $var_value:expr) => {
        #[cfg(any(debug_assertions, test))]
        let mut $var_name = $var_value;

        #[cfg(not(any(debug_assertions, test)))]
        let $var_name = $var_value;
    };
}

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
