use tauri::Manager;

use crate::{
    command::error::{CommandError, CommandError::NotFound},
    entity::recipe_file::Model,
    entity_crud::{
        recipe_file::{
            RecipeFileCondition, RecipeFileCreate, RecipeFileCrud, RecipeFileFilter,
            RecipeFileUpdate,
        },
        EntityCrudTrait,
    },
};

#[tauri::command]
pub async fn entity_create_recipe_file(create: RecipeFileCreate) -> Result<i64, CommandError> {
    let id = RecipeFileCrud::create(create).await?;
    Ok(id)
}

#[tauri::command]
pub async fn entity_read_recipe_file(
    id: i64,
    window: tauri::Window,
) -> Result<Model, CommandError> {
    let model_option = RecipeFileCrud::read(id).await?;
    let model = model_option.ok_or(NotFound)?;
    window.asset_protocol_scope().allow_file(&model.path)?;
    Ok(model)
}

#[tauri::command]
pub async fn entity_update_recipe_file(update: RecipeFileUpdate) -> Result<(), CommandError> {
    RecipeFileCrud::update(update).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_delete_recipe_file(id: i64) -> Result<(), CommandError> {
    RecipeFileCrud::delete(id).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_list_recipe_file(filter: RecipeFileFilter) -> Result<Vec<i64>, CommandError> {
    let list = RecipeFileCrud::list(filter).await?;
    Ok(list)
}

#[tauri::command]
pub async fn entity_count_recipe_file(
    condition: Option<RecipeFileCondition>,
) -> Result<i64, CommandError> {
    let count = RecipeFileCrud::count(condition).await?;
    Ok(count)
}
