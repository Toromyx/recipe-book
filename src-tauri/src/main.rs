#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{AppHandle, Wry};

use crate::command::{
    entity::{
        ingredient::{
            entity_count_ingredient, entity_create_ingredient, entity_delete_ingredient,
            entity_list_ingredient, entity_read_ingredient, entity_update_ingredient,
        },
        recipe::{
            entity_count_recipe, entity_create_recipe, entity_delete_recipe, entity_list_recipe,
            entity_read_recipe, entity_update_recipe,
        },
        recipe_file::{
            entity_count_recipe_file, entity_create_recipe_file, entity_delete_recipe_file,
            entity_list_recipe_file, entity_read_recipe_file, entity_update_recipe_file,
        },
        recipe_ingredient::{
            entity_count_recipe_ingredient, entity_create_recipe_ingredient,
            entity_delete_recipe_ingredient, entity_list_recipe_ingredient,
            entity_read_recipe_ingredient, entity_update_recipe_ingredient,
        },
        recipe_ingredient_draft::{
            entity_count_recipe_ingredient_draft, entity_create_recipe_ingredient_draft,
            entity_delete_recipe_ingredient_draft, entity_list_recipe_ingredient_draft,
            entity_read_recipe_ingredient_draft, entity_update_recipe_ingredient_draft,
        },
        recipe_step::{
            entity_count_recipe_step, entity_create_recipe_step, entity_delete_recipe_step,
            entity_list_recipe_step, entity_read_recipe_step, entity_update_recipe_step,
        },
        unit_name::{
            entity_count_unit_name, entity_create_unit_name, entity_delete_unit_name,
            entity_list_unit_name, entity_read_unit_name, entity_update_unit_name,
        },
    },
    external_recipe::external_recipe,
    ocr::ocr,
    unit_conversion::unit_convert,
    unit_list::unit_list_get,
};

mod command;
mod database;
mod entity;
mod entity_crud;
mod event;
mod external_recipe;
mod fs;
mod log;
mod migrator;
mod path;
mod protocol;
mod recipe_file_storage;
mod scraper;
mod unit_conversion;
mod window;

/// This static variable holds the app handle once the tauri app has started.
static mut APP_HANDLE: Option<AppHandle> = None;

/// Get and unwrap the static app handle [`APP_HANDLE`].
///
/// See [`try_get_app_handle`] for a non-panicing version.
///
/// # Panics
///
/// This function panics when [`APP_HANDLE`] is [`None`]. this is the case when the tauri app is un-initialized.
pub fn get_app_handle() -> &'static AppHandle {
    try_get_app_handle().expect("Could not get the app handle.")
}

/// Get the static app handle [`APP_HANDLE`].
pub fn try_get_app_handle() -> Option<&'static AppHandle> {
    unsafe { APP_HANDLE.as_ref() }
}

#[tokio::main]
async fn main() {
    setup()
        .run(tauri::generate_context!())
        .expect("There was an error while running the application");
}

fn setup() -> tauri::Builder<Wry> {
    tauri::Builder::default()
        .setup(|app| {
            unsafe {
                APP_HANDLE = Some(app.handle());
            }
            log::init();
            Ok(())
        })
        .register_uri_scheme_protocol(protocol::recipe_file::URI_SCHEME, |app_handle, request| {
            let response = protocol::recipe_file::protocol(app_handle, request)?;
            Ok(response)
        })
        .invoke_handler(tauri::generate_handler![
            entity_create_ingredient,
            entity_read_ingredient,
            entity_update_ingredient,
            entity_delete_ingredient,
            entity_list_ingredient,
            entity_count_ingredient,
            entity_create_recipe,
            entity_read_recipe,
            entity_update_recipe,
            entity_delete_recipe,
            entity_list_recipe,
            entity_count_recipe,
            entity_create_recipe_file,
            entity_read_recipe_file,
            entity_update_recipe_file,
            entity_delete_recipe_file,
            entity_list_recipe_file,
            entity_count_recipe_file,
            entity_create_recipe_ingredient,
            entity_read_recipe_ingredient,
            entity_update_recipe_ingredient,
            entity_delete_recipe_ingredient,
            entity_list_recipe_ingredient,
            entity_count_recipe_ingredient,
            entity_create_recipe_ingredient_draft,
            entity_read_recipe_ingredient_draft,
            entity_update_recipe_ingredient_draft,
            entity_delete_recipe_ingredient_draft,
            entity_list_recipe_ingredient_draft,
            entity_count_recipe_ingredient_draft,
            entity_create_recipe_step,
            entity_read_recipe_step,
            entity_update_recipe_step,
            entity_delete_recipe_step,
            entity_list_recipe_step,
            entity_count_recipe_step,
            entity_create_unit_name,
            entity_read_unit_name,
            entity_update_unit_name,
            entity_delete_unit_name,
            entity_list_unit_name,
            entity_count_unit_name,
            external_recipe,
            ocr,
            unit_convert,
            unit_list_get,
        ])
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{mpsc::channel, Once},
        thread,
    };

    use super::*;

    static RUN_ONCE: Once = Once::new();

    /// Run the tauri app, but only once.
    pub fn run() {
        RUN_ONCE.call_once(|| {
            let (sender, receiver) = channel();
            thread::spawn(move || {
                setup()
                    .any_thread()
                    .build(tauri::generate_context!())
                    .unwrap()
                    .run(move |_, _| {
                        sender.send(()).ok();
                    });
            });
            receiver.recv().ok();
        });
    }
}
