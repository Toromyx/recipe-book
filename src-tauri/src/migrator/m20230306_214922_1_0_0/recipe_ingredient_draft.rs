//! This module implements the creation of [`crate::entity::recipe_step_ingredient_draft`].

use sea_orm_migration::prelude::*;

use crate::migrator::{index_name, m20230306_214922_1_0_0::recipe::Recipe};

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
                    ColumnDef::new(RecipeIngredientDraft::RecipeId)
                        .integer()
                        .not_null(),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(
                            RecipeIngredientDraft::Table,
                            RecipeIngredientDraft::RecipeId,
                        )
                        .to(Recipe::Table, Recipe::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .index(
                    Index::create()
                        .col(RecipeIngredientDraft::Order)
                        .col(RecipeIngredientDraft::RecipeId)
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
                    &RecipeIngredientDraft::RecipeId,
                ))
                .table(RecipeIngredientDraft::Table)
                .col(RecipeIngredientDraft::RecipeId)
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
    RecipeId,
}

#[cfg(test)]
pub mod tests {
    use pretty_assertions::{assert_eq, assert_str_eq};
    use sea_orm::DatabaseConnection;

    use crate::database::tests::{get_table_indices, get_table_schema};

    pub async fn assert_recipe_ingredient_draft_schema(db: &DatabaseConnection) {
        let table_schema = get_table_schema("recipe_ingredient_draft", db).await;
        assert_str_eq!(
            table_schema,
            "CREATE TABLE \"recipe_ingredient_draft\" ( \
            \"id\" integer NOT NULL PRIMARY KEY AUTOINCREMENT, \
            \"order\" integer NOT NULL, \
            \"text\" text NOT NULL, \
            \"recipe_id\" integer NOT NULL, \
            UNIQUE (\"order\", \"recipe_id\"), \
            FOREIGN KEY (\"recipe_id\") REFERENCES \"recipe\" (\"id\") ON DELETE CASCADE \
            )"
        );
    }

    pub async fn assert_recipe_ingredient_draft_indices(db: &DatabaseConnection) {
        let indices = get_table_indices("recipe_ingredient_draft", db).await;
        assert_eq!(
            indices,
            vec![
                String::from(
                    "CREATE INDEX \"idx-recipe_ingredient_draft-order\" ON \"recipe_ingredient_draft\" (\"order\")"
                ),
                String::from(
                    "CREATE INDEX \"idx-recipe_ingredient_draft-recipe_id\" ON \"recipe_ingredient_draft\" (\"recipe_id\")"
                ),
            ]
        )
    }
}
