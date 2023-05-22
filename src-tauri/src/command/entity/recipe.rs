use crate::{
    command::error::{CommandError, CommandError::NotFound},
    entity::recipe::Model,
    entity_crud::{
        recipe::{RecipeCondition, RecipeCreate, RecipeCrud, RecipeFilter, RecipeUpdate},
        EntityCrudTrait,
    },
};

#[tauri::command]
pub async fn entity_create_recipe(create: RecipeCreate) -> Result<i64, CommandError> {
    let id = RecipeCrud::create(create).await?;
    Ok(id)
}

#[tauri::command]
pub async fn entity_read_recipe(id: i64) -> Result<Model, CommandError> {
    let model_option = RecipeCrud::read(id).await?;
    let model = model_option.ok_or(NotFound)?;
    Ok(model)
}

#[tauri::command]
pub async fn entity_update_recipe(update: RecipeUpdate) -> Result<(), CommandError> {
    RecipeCrud::update(update).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_delete_recipe(id: i64) -> Result<(), CommandError> {
    RecipeCrud::delete(id).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_list_recipe(filter: RecipeFilter) -> Result<Vec<i64>, CommandError> {
    let list = RecipeCrud::list(filter).await?;
    Ok(list)
}

#[tauri::command]
pub async fn entity_count_recipe(condition: Option<RecipeCondition>) -> Result<i64, CommandError> {
    let count = RecipeCrud::count(condition).await?;
    Ok(count)
}
