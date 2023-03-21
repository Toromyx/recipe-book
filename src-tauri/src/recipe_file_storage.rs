use std::path::PathBuf;

use sea_orm::{DbErr, EntityTrait, ModelTrait};
use tokio::fs;

use crate::{
    database,
    entity::{recipe_file::Model, recipe_step},
    path::app_data_dir,
    recipe_file_storage::error::RecipeFileStorageError,
};

pub mod error;

pub async fn create(recipe_file: &Model) -> Result<PathBuf, RecipeFileStorageError> {
    let target = file(recipe_file).await?;
    let source = PathBuf::from(&recipe_file.path);
    fs::create_dir_all(&target.parent().unwrap()).await?;
    fs::copy(&source, &target).await?;
    Ok(target)
}

pub async fn delete(recipe_file: &Model) -> Result<(), RecipeFileStorageError> {
    let file = file(recipe_file).await?;
    fs::remove_file(file).await?;
    Ok(())
}

pub fn dir() -> PathBuf {
    let mut dir = app_data_dir();
    dir.push("recipe_files");
    dir
}

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

async fn file(recipe_file: &Model) -> Result<PathBuf, DbErr> {
    let mut file = dir();
    for path_segment in path_segments(recipe_file).await? {
        file.push(path_segment);
    }
    Ok(file)
}
