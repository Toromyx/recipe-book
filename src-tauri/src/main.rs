#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::AppHandle;

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
        recipe_step::{
            entity_count_recipe_step, entity_create_recipe_step, entity_delete_recipe_step,
            entity_list_recipe_step, entity_read_recipe_step, entity_update_recipe_step,
        },
    },
    unit_list::unit_list_get,
};

mod command;
mod database;
mod entity;
mod entity_crud;
mod event;
mod fs;
mod log;
mod migrator;
mod path;
mod protocol;
mod recipe_file_storage;
mod window;

static mut APP_HANDLE: Option<AppHandle> = None;

pub fn get_app_handle() -> &'static AppHandle {
    try_get_app_handle().expect("Could not get the app handle.")
}
pub fn try_get_app_handle() -> Option<&'static AppHandle> {
    unsafe { APP_HANDLE.as_ref() }
}

#[tokio::main]
async fn main() {
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
            entity_create_recipe_step,
            entity_read_recipe_step,
            entity_update_recipe_step,
            entity_delete_recipe_step,
            entity_list_recipe_step,
            entity_count_recipe_step,
            unit_list_get,
        ])
        .run(tauri::generate_context!())
        .expect("There was an error while running the application");
}
