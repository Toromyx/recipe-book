use crate::{
    api,
    api::entity::recipe_file::{RecipeFileCreate, RecipeFileFilter, RecipeFileUpdate},
    command::error::{CommandError, CommandError::NotFound},
    entity::recipe_file::Model,
};

#[tauri::command]
pub async fn entity_create_recipe_file(create: RecipeFileCreate) -> Result<i64, CommandError> {
    let id = api::entity::recipe_file::create(create).await?;
    Ok(id)
}

#[tauri::command]
pub async fn entity_read_recipe_file(id: i64) -> Result<Model, CommandError> {
    let model_option = api::entity::recipe_file::read(id).await?;
    let model = model_option.ok_or(NotFound)?;
    Ok(model)
}

#[tauri::command]
pub async fn entity_update_recipe_file(update: RecipeFileUpdate) -> Result<(), CommandError> {
    api::entity::recipe_file::update(update).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_delete_recipe_file(id: i64) -> Result<(), CommandError> {
    api::entity::recipe_file::delete(id).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_list_recipe_file(filter: RecipeFileFilter) -> Result<Vec<i64>, CommandError> {
    let list = api::entity::recipe_file::list(filter).await?;
    Ok(list)
}

#[tauri::command]
pub async fn entity_count_recipe_file(filter: RecipeFileFilter) -> Result<i64, CommandError> {
    let count = api::entity::recipe_file::count(filter).await?;
    Ok(count)
}
