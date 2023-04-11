//! This module implements the creation of [`crate::entity::recipe_ingredient`].

use sea_orm_migration::prelude::*;

use crate::migrator::{
    index_name,
    m20230306_214922_1_0_0::{ingredient::Ingredient, recipe_step::RecipeStep},
};

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
                .col(ColumnDef::new(RecipeIngredient::Quantity).double().null())
                .col(ColumnDef::new(RecipeIngredient::Unit).string().null())
                .col(ColumnDef::new(RecipeIngredient::Quality).string().null())
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
                .index(
                    Index::create()
                        .col(RecipeIngredient::RecipeStepId)
                        .col(RecipeIngredient::IngredientId)
                        .unique(),
                )
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
    Ok(())
}

#[derive(Iden)]
pub enum RecipeIngredient {
    Table,
    Id,
    Order,
    Quantity,
    Unit,
    Quality,
    RecipeStepId,
    IngredientId,
}
