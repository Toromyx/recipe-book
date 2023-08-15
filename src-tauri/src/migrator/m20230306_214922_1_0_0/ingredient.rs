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

#[cfg(test)]
pub mod tests {
    use pretty_assertions::{assert_eq, assert_str_eq};
    use sea_orm::DatabaseConnection;

    use crate::database::tests::{get_table_indices, get_table_schema};

    pub async fn assert_ingredient_schema(db: &DatabaseConnection) {
        let table_schema = get_table_schema("ingredient", db).await;
        assert_str_eq!(
            table_schema,
            "CREATE TABLE \"ingredient\" ( \
        \"id\" integer NOT NULL PRIMARY KEY AUTOINCREMENT, \
        \"name\" text NOT NULL \
        )"
        );
    }

    pub async fn assert_ingredient_indices(db: &DatabaseConnection) {
        let indices = get_table_indices("ingredient", db).await;
        assert_eq!(
            indices,
            vec![String::from(
                "CREATE INDEX \"idx-ingredient-name\" ON \"ingredient\" (\"name\")"
            ),]
        )
    }
}
