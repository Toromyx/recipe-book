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

#[cfg(test)]
pub mod tests {
    use pretty_assertions::{assert_eq, assert_str_eq};
    use sea_orm::DatabaseConnection;

    use crate::database::tests::{get_table_indices, get_table_schema};

    pub async fn assert_recipe_schema(db: &DatabaseConnection) {
        let table_schema = get_table_schema("recipe", db).await;
        assert_str_eq!(
            table_schema,
            "CREATE TABLE \"recipe\" ( \
        \"id\" integer NOT NULL PRIMARY KEY AUTOINCREMENT, \
        \"name\" text NOT NULL \
        )"
        );
    }

    pub async fn assert_recipe_indices(db: &DatabaseConnection) {
        let indices = get_table_indices("recipe", db).await;
        assert_eq!(
            indices,
            vec![String::from(
                "CREATE INDEX \"idx-recipe-name\" ON \"recipe\" (\"name\")"
            ),]
        )
    }
}
