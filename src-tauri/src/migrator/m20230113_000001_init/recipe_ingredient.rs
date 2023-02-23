use sea_orm_migration::prelude::*;

use crate::migrator::m20230113_000001_init::{ingredient::Ingredient, recipe_step::RecipeStep};

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(RecipeIngredient::Table)
                .col(
                    ColumnDef::new(RecipeIngredient::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(RecipeIngredient::Order).integer().not_null())
                .col(
                    ColumnDef::new(RecipeIngredient::Quantity)
                        .double()
                        .not_null(),
                )
                .col(ColumnDef::new(RecipeIngredient::Unit).string().not_null())
                .col(
                    ColumnDef::new(RecipeIngredient::RecipeStepId)
                        .integer()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(RecipeIngredient::IngredientId)
                        .integer()
                        .not_null(),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(RecipeIngredient::Table, RecipeIngredient::RecipeStepId)
                        .to(RecipeStep::Table, RecipeStep::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(RecipeIngredient::Table, RecipeIngredient::IngredientId)
                        .to(Ingredient::Table, Ingredient::Id)
                        .on_delete(ForeignKeyAction::Restrict),
                )
                .index(
                    Index::create()
                        .col(RecipeIngredient::Order)
                        .col(RecipeIngredient::RecipeStepId)
                        .unique(),
                )
                .to_owned(),
        )
        .await
}

#[derive(Iden)]
pub enum RecipeIngredient {
    Table,
    Id,
    Order,
    Quantity,
    Unit,
    RecipeStepId,
    IngredientId,
}
