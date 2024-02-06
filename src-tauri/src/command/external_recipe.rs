use crate::{
    command::error::CommandError,
    entity_crud::{
        recipe::{RecipeCreate, RecipeCrud},
        recipe_file::{RecipeFileCreate, RecipeFileCreateUri, RecipeFileCrud},
        recipe_step::{RecipeStepCreate, RecipeStepCrud},
        recipe_step_ingredient_draft::{
            RecipeStepIngredientDraftCreate, RecipeStepIngredientDraftCrud,
        },
        EntityCrudTrait,
    },
};

#[tauri::command]
pub async fn external_recipe(url: String) -> Result<i64, CommandError> {
    let external_recipe = crate::external_recipe::get(url).await?;
    let recipe_id = RecipeCrud::create(RecipeCreate {
        name: external_recipe.name,
    })
    .await?;
    for (i, step) in external_recipe.steps.into_iter().enumerate() {
        tokio::spawn(async move {
            let recipe_step_id = RecipeStepCrud::create(RecipeStepCreate {
                recipe_id,
                description: step.description,
                order: (i + 1) as i64,
            })
            .await
            .unwrap();
            for (i, ingredient) in step.ingredients.into_iter().enumerate() {
                tokio::spawn(async move {
                    RecipeStepIngredientDraftCrud::create(RecipeStepIngredientDraftCreate {
                        order: (i + 1) as i64,
                        text: ingredient,
                        recipe_step_id,
                    })
                    .await
                    .unwrap();
                });
            }
            for (i, file) in step.files.into_iter().enumerate() {
                tokio::spawn(async move {
                    RecipeFileCrud::create(RecipeFileCreate {
                        name: file.clone(),
                        order: (i + 1) as i64,
                        uri: RecipeFileCreateUri::Url(file),
                        recipe_step_id,
                    })
                    .await
                    .unwrap();
                });
            }
        });
    }
    Ok(recipe_id)
}
