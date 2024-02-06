use crate::{
    command::error::{CommandError, CommandError::NotFound},
    entity::recipe_step_ingredient::Model,
    entity_crud::{
        recipe_step_ingredient::{
            RecipeStepIngredientCondition, RecipeStepIngredientCreate, RecipeStepIngredientCrud,
            RecipeStepIngredientFilter, RecipeStepIngredientUpdate,
        },
        EntityCrudTrait,
    },
};

#[tauri::command]
pub async fn entity_create_recipe_step_ingredient(
    create: RecipeStepIngredientCreate,
) -> Result<i64, CommandError> {
    let id = RecipeStepIngredientCrud::create(create).await?;
    Ok(id)
}

#[tauri::command]
pub async fn entity_read_recipe_step_ingredient(id: i64) -> Result<Model, CommandError> {
    let model_option = RecipeStepIngredientCrud::read(id).await?;
    let model = model_option.ok_or(NotFound)?;
    Ok(model)
}

#[tauri::command]
pub async fn entity_update_recipe_step_ingredient(
    update: RecipeStepIngredientUpdate,
) -> Result<(), CommandError> {
    RecipeStepIngredientCrud::update(update).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_delete_recipe_step_ingredient(id: i64) -> Result<(), CommandError> {
    RecipeStepIngredientCrud::delete(id).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_list_recipe_step_ingredient(
    filter: RecipeStepIngredientFilter,
) -> Result<Vec<i64>, CommandError> {
    let list = RecipeStepIngredientCrud::list(filter).await?;
    Ok(list)
}

#[tauri::command]
pub async fn entity_count_recipe_step_ingredient(
    condition: Option<RecipeStepIngredientCondition>,
) -> Result<i64, CommandError> {
    let count = RecipeStepIngredientCrud::count(condition).await?;
    Ok(count)
}
