//! This module implements the creation of [`crate::entity::recipe_step_ingredient`].

use sea_orm_migration::prelude::*;

use crate::migrator::{
    index_name,
    m20230306_214922_1_0_0::{ingredient::Ingredient, recipe_step::RecipeStep},
};

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(RecipeStepIngredient::Table)
                .col(
                    ColumnDef::new(RecipeStepIngredient::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(
                    ColumnDef::new(RecipeStepIngredient::Order)
                        .integer()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(RecipeStepIngredient::Quantity)
                        .double()
                        .null(),
                )
                .col(ColumnDef::new(RecipeStepIngredient::Unit).string().null())
                .col(
                    ColumnDef::new(RecipeStepIngredient::Quality)
                        .string()
                        .null(),
                )
                .col(
                    ColumnDef::new(RecipeStepIngredient::RecipeStepId)
                        .integer()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(RecipeStepIngredient::IngredientId)
                        .integer()
                        .not_null(),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(
                            RecipeStepIngredient::Table,
                            RecipeStepIngredient::RecipeStepId,
                        )
                        .to(RecipeStep::Table, RecipeStep::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(
                            RecipeStepIngredient::Table,
                            RecipeStepIngredient::IngredientId,
                        )
                        .to(Ingredient::Table, Ingredient::Id)
                        .on_delete(ForeignKeyAction::Restrict),
                )
                .index(
                    Index::create()
                        .col(RecipeStepIngredient::Order)
                        .col(RecipeStepIngredient::RecipeStepId)
                        .unique(),
                )
                .index(
                    Index::create()
                        .col(RecipeStepIngredient::RecipeStepId)
                        .col(RecipeStepIngredient::IngredientId)
                        .unique(),
                )
                .to_owned(),
        )
        .await?;
    manager
        .create_index(
            Index::create()
                .name(&index_name(
                    &RecipeStepIngredient::Table,
                    &RecipeStepIngredient::Order,
                ))
                .table(RecipeStepIngredient::Table)
                .col(RecipeStepIngredient::Order)
                .to_owned(),
        )
        .await?;
    manager
        .create_index(
            Index::create()
                .name(&index_name(
                    &RecipeStepIngredient::Table,
                    &RecipeStepIngredient::RecipeStepId,
                ))
                .table(RecipeStepIngredient::Table)
                .col(RecipeStepIngredient::RecipeStepId)
                .to_owned(),
        )
        .await?;
    manager
        .create_index(
            Index::create()
                .name(&index_name(
                    &RecipeStepIngredient::Table,
                    &RecipeStepIngredient::IngredientId,
                ))
                .table(RecipeStepIngredient::Table)
                .col(RecipeStepIngredient::IngredientId)
                .to_owned(),
        )
        .await?;
    Ok(())
}

#[derive(Iden)]
pub enum RecipeStepIngredient {
    Table,
    Id,
    Order,
    Quantity,
    Unit,
    Quality,
    RecipeStepId,
    IngredientId,
}

#[cfg(test)]
pub mod tests {
    use pretty_assertions::{assert_eq, assert_str_eq};
    use sea_orm::DatabaseConnection;

    use crate::database::tests::{get_table_indices, get_table_schema};

    pub async fn assert_recipe_step_ingredient_schema(db: &DatabaseConnection) {
        let table_schema = get_table_schema("recipe_step_ingredient", db).await;
        assert_str_eq!(
            table_schema,
            "CREATE TABLE \"recipe_step_ingredient\" ( \
            \"id\" integer NOT NULL PRIMARY KEY AUTOINCREMENT, \
            \"order\" integer NOT NULL, \
            \"quantity\" real NULL, \
            \"unit\" text NULL, \
            \"quality\" text NULL, \
            \"recipe_step_id\" integer NOT NULL, \
            \"ingredient_id\" integer NOT NULL, \
            UNIQUE (\"order\", \"recipe_step_id\"), \
            UNIQUE (\"recipe_step_id\", \"ingredient_id\"), \
            FOREIGN KEY (\"recipe_step_id\") REFERENCES \"recipe_step\" (\"id\") ON DELETE CASCADE, \
            FOREIGN KEY (\"ingredient_id\") REFERENCES \"ingredient\" (\"id\") ON DELETE RESTRICT \
            )"
        );
    }

    pub async fn assert_recipe_step_ingredient_indices(db: &DatabaseConnection) {
        let indices = get_table_indices("recipe_step_ingredient", db).await;
        assert_eq!(
            indices,
            vec![
                String::from(
                    "CREATE INDEX \"idx-recipe_step_ingredient-order\" ON \"recipe_step_ingredient\" (\"order\")"
                ),
                String::from(
                    "CREATE INDEX \"idx-recipe_step_ingredient-recipe_step_id\" ON \"recipe_step_ingredient\" (\"recipe_step_id\")"
                ),
                String::from(
                    "CREATE INDEX \"idx-recipe_step_ingredient-ingredient_id\" ON \"recipe_step_ingredient\" (\"ingredient_id\")"
                ),
            ]
        )
    }
}
