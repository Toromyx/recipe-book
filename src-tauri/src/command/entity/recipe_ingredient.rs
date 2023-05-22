use crate::{
    command::error::{CommandError, CommandError::NotFound},
    entity::recipe_ingredient::Model,
    entity_crud::{
        recipe_ingredient::{
            RecipeIngredientCondition, RecipeIngredientCreate, RecipeIngredientCrud,
            RecipeIngredientFilter, RecipeIngredientUpdate,
        },
        EntityCrudTrait,
    },
};

#[tauri::command]
pub async fn entity_create_recipe_ingredient(
    create: RecipeIngredientCreate,
) -> Result<i64, CommandError> {
    let id = RecipeIngredientCrud::create(create).await?;
    Ok(id)
}

#[tauri::command]
pub async fn entity_read_recipe_ingredient(id: i64) -> Result<Model, CommandError> {
    let model_option = RecipeIngredientCrud::read(id).await?;
    let model = model_option.ok_or(NotFound)?;
    Ok(model)
}

#[tauri::command]
pub async fn entity_update_recipe_ingredient(
    update: RecipeIngredientUpdate,
) -> Result<(), CommandError> {
    RecipeIngredientCrud::update(update).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_delete_recipe_ingredient(id: i64) -> Result<(), CommandError> {
    RecipeIngredientCrud::delete(id).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_list_recipe_ingredient(
    filter: RecipeIngredientFilter,
) -> Result<Vec<i64>, CommandError> {
    let list = RecipeIngredientCrud::list(filter).await?;
    Ok(list)
}

#[tauri::command]
pub async fn entity_count_recipe_ingredient(
    condition: Option<RecipeIngredientCondition>,
) -> Result<i64, CommandError> {
    let count = RecipeIngredientCrud::count(condition).await?;
    Ok(count)
}
