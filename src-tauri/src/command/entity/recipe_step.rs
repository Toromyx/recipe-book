use crate::{
    api,
    api::entity::recipe_step::{RecipeStepCreate, RecipeStepFilter, RecipeStepUpdate},
    command::error::{CommandError, CommandError::NotFound},
    entity::recipe_step::Model,
};

#[tauri::command]
pub async fn entity_create_recipe_step(create: RecipeStepCreate) -> Result<i64, CommandError> {
    let id = api::entity::recipe_step::create(create).await?;
    Ok(id)
}

#[tauri::command]
pub async fn entity_read_recipe_step(id: i64) -> Result<Model, CommandError> {
    let model_option = api::entity::recipe_step::read(id).await?;
    let model = model_option.ok_or(NotFound)?;
    Ok(model)
}

#[tauri::command]
pub async fn entity_update_recipe_step(update: RecipeStepUpdate) -> Result<(), CommandError> {
    api::entity::recipe_step::update(update).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_delete_recipe_step(id: i64) -> Result<(), CommandError> {
    api::entity::recipe_step::delete(id).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_list_recipe_step(filter: RecipeStepFilter) -> Result<Vec<i64>, CommandError> {
    let list = api::entity::recipe_step::list(filter).await?;
    Ok(list)
}

#[tauri::command]
pub async fn entity_count_recipe_step(filter: RecipeStepFilter) -> Result<i64, CommandError> {
    let count = api::entity::recipe_step::count(filter).await?;
    Ok(count)
}
