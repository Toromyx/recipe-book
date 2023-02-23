use sea_orm_migration::prelude::*;

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
        .await
}

#[derive(Iden)]
pub enum Recipe {
    Table,
    Id,
    Name,
}
