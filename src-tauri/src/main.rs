#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Wry;

use crate::command::{
    entity::{
        file::{
            entity_count_file, entity_create_file, entity_delete_file, entity_list_file,
            entity_read_file, entity_update_file,
        },
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
        recipe_ingredient_draft::{
            entity_count_recipe_ingredient_draft, entity_create_recipe_ingredient_draft,
            entity_delete_recipe_ingredient_draft, entity_list_recipe_ingredient_draft,
            entity_read_recipe_ingredient_draft, entity_update_recipe_ingredient_draft,
        },
        recipe_step::{
            entity_count_recipe_step, entity_create_recipe_step, entity_delete_recipe_step,
            entity_list_recipe_step, entity_read_recipe_step, entity_update_recipe_step,
        },
        recipe_step_file::{
            entity_count_recipe_step_file, entity_create_recipe_step_file,
            entity_delete_recipe_step_file, entity_list_recipe_step_file,
            entity_read_recipe_step_file, entity_update_recipe_step_file,
        },
        recipe_step_ingredient::{
            entity_count_recipe_step_ingredient, entity_create_recipe_step_ingredient,
            entity_delete_recipe_step_ingredient, entity_list_recipe_step_ingredient,
            entity_read_recipe_step_ingredient, entity_update_recipe_step_ingredient,
        },
        recipe_step_ingredient_draft::{
            entity_count_recipe_step_ingredient_draft, entity_create_recipe_step_ingredient_draft,
            entity_delete_recipe_step_ingredient_draft, entity_list_recipe_step_ingredient_draft,
            entity_read_recipe_step_ingredient_draft, entity_update_recipe_step_ingredient_draft,
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

mod app_handle;
mod command;
mod database;
mod dom_content_loaded;
mod entity;
mod entity_crud;
mod event;
mod external_recipe;
mod file_storage;
mod fs;
mod log;
mod migrator;
mod path;
mod scraper;
mod unit_conversion;
mod window;

#[tokio::main]
async fn main() {
    setup()
        .run(tauri::generate_context!())
        .expect("There was an error while running the application");
}

fn setup() -> tauri::Builder<Wry> {
    tauri::Builder::default()
        .setup(|app| {
            app_handle::setup(app);
            log::init();
            dom_content_loaded::setup(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            entity_create_file,
            entity_read_file,
            entity_update_file,
            entity_delete_file,
            entity_list_file,
            entity_count_file,
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
            entity_create_recipe_ingredient_draft,
            entity_read_recipe_ingredient_draft,
            entity_update_recipe_ingredient_draft,
            entity_delete_recipe_ingredient_draft,
            entity_list_recipe_ingredient_draft,
            entity_count_recipe_ingredient_draft,
            entity_create_recipe_step_file,
            entity_read_recipe_step_file,
            entity_update_recipe_step_file,
            entity_delete_recipe_step_file,
            entity_list_recipe_step_file,
            entity_count_recipe_step_file,
            entity_create_recipe_step_ingredient,
            entity_read_recipe_step_ingredient,
            entity_update_recipe_step_ingredient,
            entity_delete_recipe_step_ingredient,
            entity_list_recipe_step_ingredient,
            entity_count_recipe_step_ingredient,
            entity_create_recipe_step_ingredient_draft,
            entity_read_recipe_step_ingredient_draft,
            entity_update_recipe_step_ingredient_draft,
            entity_delete_recipe_step_ingredient_draft,
            entity_list_recipe_step_ingredient_draft,
            entity_count_recipe_step_ingredient_draft,
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
    use std::{cell::RefCell, path::PathBuf, sync::Once, thread};

    use super::*;

    static RUN_ONCE: Once = Once::new();

    thread_local! {
        pub static TEST_NAME: RefCell<Option<String>> = RefCell::new(None);
    }

    /// Run the tauri app, but only once.
    pub fn run() {
        RUN_ONCE.call_once(|| {
            thread::spawn(move || {
                setup()
                    .any_thread()
                    .run(tauri::generate_context!())
                    .unwrap();
            });
        });
    }

    pub fn temp_file_path(name: &str) -> PathBuf {
        let mut temp_path = std::env::temp_dir();
        temp_path.push("recipe_book_tests");
        std::fs::create_dir_all(&temp_path).unwrap();
        temp_path.push(name);
        temp_path
    }

    pub fn create_temp_file<Contents>(name: &str, contents: Contents) -> PathBuf
    where
        Contents: AsRef<[u8]>,
    {
        let temp_path = temp_file_path(name);
        std::fs::write(&temp_path, contents).unwrap();

        temp_path
    }
}
