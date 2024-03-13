use crate::{
    command::error::CommandError,
    entity_crud::{
        file::{FileCreate, FileCreateUri, FileCrud},
        recipe::{RecipeCreate, RecipeCrud},
        recipe_file::{RecipeFileCreate, RecipeFileCrud},
        recipe_ingredient_draft::{RecipeIngredientDraftCreate, RecipeIngredientDraftCrud},
        recipe_step::{RecipeStepCreate, RecipeStepCrud},
        recipe_step_file::{RecipeStepFileCreate, RecipeStepFileCrud},
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
    for (i, ingredient) in external_recipe.ingredients.into_iter().enumerate() {
        tokio::spawn(async move {
            RecipeIngredientDraftCrud::create(RecipeIngredientDraftCreate {
                order: (i + 1) as i64,
                text: ingredient,
                recipe_id,
            })
            .await
            .unwrap();
        });
    }
    for (i, file) in external_recipe.files.into_iter().enumerate() {
        tokio::spawn(async move {
            let file_id = FileCrud::create(FileCreate {
                name: file.clone(),
                uri: FileCreateUri::Url(file),
            })
            .await
            .unwrap();
            RecipeFileCrud::create(RecipeFileCreate {
                order: (i + 1) as i64,
                recipe_id,
                file_id,
            })
            .await
            .unwrap();
        });
    }
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
                    let file_id = FileCrud::create(FileCreate {
                        name: file.clone(),
                        uri: FileCreateUri::Url(file),
                    })
                    .await
                    .unwrap();
                    RecipeStepFileCrud::create(RecipeStepFileCreate {
                        order: (i + 1) as i64,
                        recipe_step_id,
                        file_id,
                    })
                    .await
                    .unwrap();
                });
            }
        });
    }
    Ok(recipe_id)
}
