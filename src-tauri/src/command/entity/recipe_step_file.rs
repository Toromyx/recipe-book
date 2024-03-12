use crate::{
    command::error::{CommandError, CommandError::NotFound},
    entity::recipe_step_file::Model,
    entity_crud::{
        recipe_step_file::{
            RecipeStepFileCondition, RecipeStepFileCreate, RecipeStepFileCrud,
            RecipeStepFileFilter, RecipeStepFileUpdate,
        },
        EntityCrudTrait,
    },
};

#[tauri::command]
pub async fn entity_create_recipe_step_file(
    create: RecipeStepFileCreate,
) -> Result<i64, CommandError> {
    let id = RecipeStepFileCrud::create(create).await?;
    Ok(id)
}

#[tauri::command]
pub async fn entity_read_recipe_step_file(id: i64) -> Result<Model, CommandError> {
    let model_option = RecipeStepFileCrud::read(id).await?;
    let model = model_option.ok_or(NotFound)?;
    Ok(model)
}

#[tauri::command]
pub async fn entity_update_recipe_step_file(
    update: RecipeStepFileUpdate,
) -> Result<(), CommandError> {
    RecipeStepFileCrud::update(update).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_delete_recipe_step_file(id: i64) -> Result<(), CommandError> {
    RecipeStepFileCrud::delete(id).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_list_recipe_step_file(
    filter: RecipeStepFileFilter,
) -> Result<Vec<i64>, CommandError> {
    let list = RecipeStepFileCrud::list(filter).await?;
    Ok(list)
}

#[tauri::command]
pub async fn entity_count_recipe_step_file(
    condition: Option<RecipeStepFileCondition>,
) -> Result<i64, CommandError> {
    let count = RecipeStepFileCrud::count(condition).await?;
    Ok(count)
}
