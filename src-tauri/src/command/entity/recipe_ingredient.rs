use crate::{
    api,
    api::entity::recipe_ingredient::{
        RecipeIngredientCreate, RecipeIngredientFilter, RecipeIngredientUpdate,
    },
    command::error::{CommandError, CommandError::NotFound},
    entity::recipe_ingredient::Model,
};

#[tauri::command]
pub async fn entity_create_recipe_ingredient(
    create: RecipeIngredientCreate,
) -> Result<i64, CommandError> {
    let id = api::entity::recipe_ingredient::create(create).await?;
    Ok(id)
}

#[tauri::command]
pub async fn entity_read_recipe_ingredient(id: i64) -> Result<Model, CommandError> {
    let model_option = api::entity::recipe_ingredient::read(id).await?;
    let model = model_option.ok_or(NotFound)?;
    Ok(model)
}

#[tauri::command]
pub async fn entity_update_recipe_ingredient(
    update: RecipeIngredientUpdate,
) -> Result<(), CommandError> {
    api::entity::recipe_ingredient::update(update).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_delete_recipe_ingredient(id: i64) -> Result<(), CommandError> {
    api::entity::recipe_ingredient::delete(id).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_list_recipe_ingredient(
    filter: RecipeIngredientFilter,
) -> Result<Vec<i64>, CommandError> {
    let list = api::entity::recipe_ingredient::list(filter).await?;
    Ok(list)
}

#[tauri::command]
pub async fn entity_count_recipe_ingredient(
    filter: RecipeIngredientFilter,
) -> Result<i64, CommandError> {
    let count = api::entity::recipe_ingredient::count(filter).await?;
    Ok(count)
}
