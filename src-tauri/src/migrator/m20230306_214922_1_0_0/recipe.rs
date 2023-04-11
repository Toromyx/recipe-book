//! This module implements the creation of [`crate::entity::recipe`].

use sea_orm_migration::prelude::*;

use crate::migrator::index_name;

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(Recipe::Table)
                .col(
                    ColumnDef::new(Recipe::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Recipe::Name).string().not_null())
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
    Ok(())
}

#[derive(Iden)]
pub enum Recipe {
    Table,
    Id,
    Name,
}
