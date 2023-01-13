use sea_orm_migration::prelude::*;

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
        .await
}

pub async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(Table::drop().table(Ingredient::Table).to_owned())
        .await
}

#[derive(Iden)]
pub enum Ingredient {
    Table,
    Id,
    Name,
}
