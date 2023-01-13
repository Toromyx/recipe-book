use crate::{
    api,
    api::entity::recipe::{RecipeCreate, RecipeFilter, RecipeUpdate},
    command::error::{CommandError, CommandError::NotFound},
    entity::recipe::Model,
};

#[tauri::command]
pub async fn entity_create_recipe(create: RecipeCreate) -> Result<i64, CommandError> {
    let id = api::entity::recipe::create(create).await?;
    Ok(id)
}

#[tauri::command]
pub async fn entity_read_recipe(id: i64) -> Result<Model, CommandError> {
    let model_option = api::entity::recipe::read(id).await?;
    let model = model_option.ok_or(NotFound)?;
    Ok(model)
}

#[tauri::command]
pub async fn entity_update_recipe(update: RecipeUpdate) -> Result<(), CommandError> {
    api::entity::recipe::update(update).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_delete_recipe(id: i64) -> Result<(), CommandError> {
    api::entity::recipe::delete(id).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_list_recipe(filter: RecipeFilter) -> Result<Vec<i64>, CommandError> {
    let list = api::entity::recipe::list(filter).await?;
    Ok(list)
}

#[tauri::command]
pub async fn entity_count_recipe(filter: RecipeFilter) -> Result<i64, CommandError> {
    let count = api::entity::recipe::count(filter).await?;
    Ok(count)
}
