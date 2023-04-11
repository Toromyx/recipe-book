//! This module implements the creation of [`crate::entity::ingredient`].

use sea_orm_migration::prelude::*;

use crate::migrator::index_name;

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(Ingredient::Table)
                .col(
                    ColumnDef::new(Ingredient::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Ingredient::Name).string().not_null())
                .to_owned(),
        )
        .await?;
    manager
        .create_index(
            Index::create()
                .name(&index_name(&Ingredient::Table, &Ingredient::Name))
                .table(Ingredient::Table)
                .col(Ingredient::Name)
                .to_owned(),
        )
        .await?;
    Ok(())
}

#[derive(Iden)]
pub enum Ingredient {
    Table,
    Id,
    Name,
}
