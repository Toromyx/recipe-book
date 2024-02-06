use crate::{
    command::error::{CommandError, CommandError::NotFound},
    entity::recipe_step_ingredient_draft::Model,
    entity_crud::{
        recipe_step_ingredient_draft::{
            RecipeStepIngredientDraftCondition, RecipeStepIngredientDraftCreate,
            RecipeStepIngredientDraftCrud, RecipeStepIngredientDraftFilter,
            RecipeStepIngredientDraftUpdate,
        },
        EntityCrudTrait,
    },
};

#[tauri::command]
pub async fn entity_create_recipe_step_ingredient_draft(
    create: RecipeStepIngredientDraftCreate,
) -> Result<i64, CommandError> {
    let id = RecipeStepIngredientDraftCrud::create(create).await?;
    Ok(id)
}

#[tauri::command]
pub async fn entity_read_recipe_step_ingredient_draft(id: i64) -> Result<Model, CommandError> {
    let model_option = RecipeStepIngredientDraftCrud::read(id).await?;
    let model = model_option.ok_or(NotFound)?;
    Ok(model)
}

#[tauri::command]
pub async fn entity_update_recipe_step_ingredient_draft(
    update: RecipeStepIngredientDraftUpdate,
) -> Result<(), CommandError> {
    RecipeStepIngredientDraftCrud::update(update).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_delete_recipe_step_ingredient_draft(id: i64) -> Result<(), CommandError> {
    RecipeStepIngredientDraftCrud::delete(id).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_list_recipe_step_ingredient_draft(
    filter: RecipeStepIngredientDraftFilter,
) -> Result<Vec<i64>, CommandError> {
    let list = RecipeStepIngredientDraftCrud::list(filter).await?;
    Ok(list)
}

#[tauri::command]
pub async fn entity_count_recipe_step_ingredient_draft(
    condition: Option<RecipeStepIngredientDraftCondition>,
) -> Result<i64, CommandError> {
    let count = RecipeStepIngredientDraftCrud::count(condition).await?;
    Ok(count)
}
