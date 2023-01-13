use crate::{
    api,
    api::entity::ingredient::{IngredientCreate, IngredientFilter, IngredientUpdate},
    command::error::{CommandError, CommandError::NotFound},
    entity::ingredient::Model,
};

#[tauri::command]
pub async fn entity_create_ingredient(create: IngredientCreate) -> Result<i64, CommandError> {
    let id = api::entity::ingredient::create(create).await?;
    Ok(id)
}

#[tauri::command]
pub async fn entity_read_ingredient(id: i64) -> Result<Model, CommandError> {
    let model_option = api::entity::ingredient::read(id).await?;
    let model = model_option.ok_or(NotFound)?;
    Ok(model)
}

#[tauri::command]
pub async fn entity_update_ingredient(update: IngredientUpdate) -> Result<(), CommandError> {
    api::entity::ingredient::update(update).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_delete_ingredient(id: i64) -> Result<(), CommandError> {
    api::entity::ingredient::delete(id).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_list_ingredient(filter: IngredientFilter) -> Result<Vec<i64>, CommandError> {
    let list = api::entity::ingredient::list(filter).await?;
    Ok(list)
}

#[tauri::command]
pub async fn entity_count_ingredient(filter: IngredientFilter) -> Result<i64, CommandError> {
    let count = api::entity::ingredient::count(filter).await?;
    Ok(count)
}
