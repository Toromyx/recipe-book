//! This module handles storage of binary files like images.

use std::path::PathBuf;

use sea_orm::{DbErr, EntityTrait};
use tokio::fs;

use crate::{
    database,
    entity::{recipe_file::Model, recipe_step},
    path::app_data_dir,
    recipe_file_storage::error::RecipeFileStorageError,
};

pub mod error;

/// Creates a new file by copying from the path specified in the existing recipe files entity.
///
/// It returns the new path.
///
/// # Errors
///
/// - [`RecipeFileStorageError::Io`] when there is an I/O error while writing the file
/// - [`RecipeFileStorageError::Db`] when [`file`] errors
pub async fn create(recipe_file: &Model) -> Result<PathBuf, RecipeFileStorageError> {
    let target = file(recipe_file).await?;
    let source = PathBuf::from(&recipe_file.path);
    fs::create_dir_all(&target.parent().unwrap()).await?;
    fs::copy(&source, &target).await?;
    Ok(target)
}

/// Deletes the stored recipe file associated with an recipe file entity.
///
/// # Errors
///
/// - [`RecipeFileStorageError::Io`] when there is an I/O error while deleting the file
/// - [`RecipeFileStorageError::Db`] when [`file`] errors
pub async fn delete(recipe_file: &Model) -> Result<(), RecipeFileStorageError> {
    let file = file(recipe_file).await?;
    fs::remove_file(file).await?;
    Ok(())
}

/// Get the recipe file storage root directory.
pub fn dir() -> PathBuf {
    let mut dir = app_data_dir();
    dir.push("recipe_files");
    dir
}

/// Get the path segments of the canonical file path relative to the recipe file storage root.
///
/// See [`dir`] for getting the recipe file storage root directory.
///
/// # Errors
///
/// - when the recipe step corresponding to the recipe file can not be found in the database
pub async fn path_segments(recipe_file: &Model) -> Result<Vec<String>, DbErr> {
    let db = database::connect().await;
    let recipe_step = recipe_step::Entity::find_by_id(recipe_file.recipe_step_id)
        .one(db)
        .await?
        .unwrap();
    let file_name = match mime2ext::mime2ext(&recipe_file.mime) {
        Some(extension) => format!("{}.{extension}", recipe_file.id),
        None => recipe_file.id.to_string(),
    };
    Ok(vec![
        recipe_step.recipe_id.to_string(),
        recipe_step.id.to_string(),
        file_name,
    ])
}

/// Get the canonical file path of a recipe file entity.
///
/// # Errors
///
/// - when [`path_segments`] returns an error variant
pub async fn file(recipe_file: &Model) -> Result<PathBuf, DbErr> {
    let mut file = dir();
    for path_segment in path_segments(recipe_file).await? {
        file.push(path_segment);
    }
    Ok(file)
}
