use crate::{
    command::error::{CommandError, CommandError::NotFound},
    entity::recipe_ingredient_draft::Model,
    entity_crud::{
        recipe_ingredient_draft::{
            RecipeIngredientDraftCondition, RecipeIngredientDraftCreate, RecipeIngredientDraftCrud,
            RecipeIngredientDraftFilter, RecipeIngredientDraftUpdate,
        },
        EntityCrudTrait,
    },
};

#[tauri::command]
pub async fn entity_create_recipe_ingredient_draft(
    create: RecipeIngredientDraftCreate,
) -> Result<i64, CommandError> {
    let id = RecipeIngredientDraftCrud::create(create).await?;
    Ok(id)
}

#[tauri::command]
pub async fn entity_read_recipe_ingredient_draft(id: i64) -> Result<Model, CommandError> {
    let model_option = RecipeIngredientDraftCrud::read(id).await?;
    let model = model_option.ok_or(NotFound)?;
    Ok(model)
}

#[tauri::command]
pub async fn entity_update_recipe_ingredient_draft(
    update: RecipeIngredientDraftUpdate,
) -> Result<(), CommandError> {
    RecipeIngredientDraftCrud::update(update).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_delete_recipe_ingredient_draft(id: i64) -> Result<(), CommandError> {
    RecipeIngredientDraftCrud::delete(id).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_list_recipe_ingredient_draft(
    filter: RecipeIngredientDraftFilter,
) -> Result<Vec<i64>, CommandError> {
    let list = RecipeIngredientDraftCrud::list(filter).await?;
    Ok(list)
}

#[tauri::command]
pub async fn entity_count_recipe_ingredient_draft(
    condition: Option<RecipeIngredientDraftCondition>,
) -> Result<i64, CommandError> {
    let count = RecipeIngredientDraftCrud::count(condition).await?;
    Ok(count)
}
