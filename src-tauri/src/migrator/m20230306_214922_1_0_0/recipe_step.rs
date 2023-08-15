//! This module implements the creation of [`crate::entity::recipe_step`].

use sea_orm_migration::prelude::*;

use crate::migrator::{index_name, m20230306_214922_1_0_0::recipe::Recipe};

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
        .await?;
    manager
        .create_index(
            Index::create()
                .name(&index_name(&RecipeStep::Table, &RecipeStep::Order))
                .table(RecipeStep::Table)
                .col(RecipeStep::Order)
                .to_owned(),
        )
        .await?;
    manager
        .create_index(
            Index::create()
                .name(&index_name(&RecipeStep::Table, &RecipeStep::RecipeId))
                .table(RecipeStep::Table)
                .col(RecipeStep::RecipeId)
                .to_owned(),
        )
        .await?;
    Ok(())
}

#[derive(Iden)]
pub enum RecipeStep {
    Table,
    Id,
    Order,
    Description,
    RecipeId,
}

#[cfg(test)]
pub mod tests {
    use pretty_assertions::{assert_eq, assert_str_eq};
    use sea_orm::DatabaseConnection;

    use crate::database::tests::{get_table_indices, get_table_schema};

    pub async fn assert_recipe_step_schema(db: &DatabaseConnection) {
        let table_schema = get_table_schema("recipe_step", db).await;
        assert_str_eq!(
            table_schema,
            "CREATE TABLE \"recipe_step\" ( \
            \"id\" integer NOT NULL PRIMARY KEY AUTOINCREMENT, \
            \"order\" integer NOT NULL, \
            \"description\" text NOT NULL, \
            \"recipe_id\" integer NOT NULL, \
            UNIQUE (\"order\", \"recipe_id\"), \
            FOREIGN KEY (\"recipe_id\") REFERENCES \"recipe\" (\"id\") ON DELETE CASCADE \
            )"
        );
    }

    pub async fn assert_recipe_step_indices(db: &DatabaseConnection) {
        let indices = get_table_indices("recipe_step", db).await;
        assert_eq!(
            indices,
            vec![
                String::from(
                    "CREATE INDEX \"idx-recipe_step-order\" ON \"recipe_step\" (\"order\")"
                ),
                String::from(
                    "CREATE INDEX \"idx-recipe_step-recipe_id\" ON \"recipe_step\" (\"recipe_id\")"
                ),
            ]
        )
    }
}
