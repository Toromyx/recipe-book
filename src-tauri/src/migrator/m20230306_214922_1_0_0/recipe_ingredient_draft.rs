//! This module implements the creation of [`crate::entity::recipe_ingredient_draft`].

use sea_orm_migration::prelude::*;

use crate::migrator::{index_name, m20230306_214922_1_0_0::recipe_step::RecipeStep};

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(RecipeIngredientDraft::Table)
                .col(
                    ColumnDef::new(RecipeIngredientDraft::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(
                    ColumnDef::new(RecipeIngredientDraft::Order)
                        .integer()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(RecipeIngredientDraft::Text)
                        .text()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(RecipeIngredientDraft::RecipeStepId)
                        .integer()
                        .not_null(),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(
                            RecipeIngredientDraft::Table,
                            RecipeIngredientDraft::RecipeStepId,
                        )
                        .to(RecipeStep::Table, RecipeStep::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .index(
                    Index::create()
                        .col(RecipeIngredientDraft::Order)
                        .col(RecipeIngredientDraft::RecipeStepId)
                        .unique(),
                )
                .to_owned(),
        )
        .await?;
    manager
        .create_index(
            Index::create()
                .name(&index_name(
                    &RecipeIngredientDraft::Table,
                    &RecipeIngredientDraft::Order,
                ))
                .table(RecipeIngredientDraft::Table)
                .col(RecipeIngredientDraft::Order)
                .to_owned(),
        )
        .await?;
    manager
        .create_index(
            Index::create()
                .name(&index_name(
                    &RecipeIngredientDraft::Table,
                    &RecipeIngredientDraft::RecipeStepId,
                ))
                .table(RecipeIngredientDraft::Table)
                .col(RecipeIngredientDraft::RecipeStepId)
                .to_owned(),
        )
        .await?;
    Ok(())
}

#[derive(Iden)]
pub enum RecipeIngredientDraft {
    Table,
    Id,
    Order,
    Text,
    RecipeStepId,
}
