use sea_orm_migration::prelude::*;

use crate::migrator::m20230113_000001_init::recipe::Recipe;

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(RecipeStep::Table)
                .col(
                    ColumnDef::new(RecipeStep::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(RecipeStep::Order).integer().not_null())
                .col(ColumnDef::new(RecipeStep::Description).text().not_null())
                .col(
                    ColumnDef::new(RecipeStep::Image)
                        .blob(BlobSize::Long)
                        .null(),
                )
                .col(ColumnDef::new(RecipeStep::RecipeId).integer().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .from(RecipeStep::Table, RecipeStep::RecipeId)
                        .to(Recipe::Table, Recipe::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .index(
                    Index::create()
                        .col(RecipeStep::Order)
                        .col(RecipeStep::RecipeId)
                        .unique(),
                )
                .to_owned(),
        )
        .await
}

pub async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(Table::drop().table(RecipeStep::Table).to_owned())
        .await
}

#[derive(Iden)]
pub enum RecipeStep {
    Table,
    Id,
    Order,
    Description,
    Image,
    RecipeId,
}
