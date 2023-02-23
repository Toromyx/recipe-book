use sea_orm_migration::prelude::*;

use crate::migrator::index_name;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name(&index_name(&Ingredient::Table, &Ingredient::Name))
                    .table(Ingredient::Table)
                    .col(Ingredient::Name)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name(&index_name(&Recipe::Table, &Recipe::Name))
                    .table(Recipe::Table)
                    .col(Recipe::Name)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name(&index_name(
                        &RecipeIngredient::Table,
                        &RecipeIngredient::Order,
                    ))
                    .table(RecipeIngredient::Table)
                    .col(RecipeIngredient::Order)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name(&index_name(
                        &RecipeIngredient::Table,
                        &RecipeIngredient::RecipeStepId,
                    ))
                    .table(RecipeIngredient::Table)
                    .col(RecipeIngredient::RecipeStepId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name(&index_name(
                        &RecipeIngredient::Table,
                        &RecipeIngredient::IngredientId,
                    ))
                    .table(RecipeIngredient::Table)
                    .col(RecipeIngredient::IngredientId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name(&index_name(&RecipeStep::Table, &RecipeStep::Order))
                    .table(RecipeStep::Table)
                    .col(RecipeStep::Order)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name(&index_name(&RecipeStep::Table, &RecipeStep::RecipeId))
                    .table(RecipeStep::Table)
                    .col(RecipeStep::RecipeId)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Ingredient {
    Table,
    Name,
}

#[derive(Iden)]
enum Recipe {
    Table,
    Name,
}

#[derive(Iden)]
enum RecipeIngredient {
    Table,
    Order,
    RecipeStepId,
    IngredientId,
}

#[derive(Iden)]
enum RecipeStep {
    Table,
    Order,
    RecipeId,
}
