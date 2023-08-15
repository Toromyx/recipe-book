//! This module implements the creation of [`crate::entity::recipe_file`].

use sea_orm_migration::prelude::*;

use crate::migrator::{index_name, m20230306_214922_1_0_0::recipe_step::RecipeStep};

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(RecipeFile::Table)
                .col(
                    ColumnDef::new(RecipeFile::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(RecipeFile::Name).string().not_null())
                .col(ColumnDef::new(RecipeFile::Order).integer().not_null())
                .col(ColumnDef::new(RecipeFile::Mime).string().not_null())
                .col(ColumnDef::new(RecipeFile::Path).string().not_null())
                .col(
                    ColumnDef::new(RecipeFile::RecipeStepId)
                        .integer()
                        .not_null(),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(RecipeFile::Table, RecipeFile::RecipeStepId)
                        .to(RecipeStep::Table, RecipeStep::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .index(
                    Index::create()
                        .col(RecipeFile::Order)
                        .col(RecipeFile::RecipeStepId)
                        .unique(),
                )
                .to_owned(),
        )
        .await?;
    manager
        .create_index(
            Index::create()
                .name(&index_name(&RecipeFile::Table, &RecipeFile::Order))
                .table(RecipeFile::Table)
                .col(RecipeFile::Order)
                .to_owned(),
        )
        .await?;
    manager
        .create_index(
            Index::create()
                .name(&index_name(&RecipeFile::Table, &RecipeFile::RecipeStepId))
                .table(RecipeFile::Table)
                .col(RecipeFile::RecipeStepId)
                .to_owned(),
        )
        .await?;
    Ok(())
}

#[derive(Iden)]
enum RecipeFile {
    Table,
    Id,
    Name,
    Order,
    Mime,
    Path,
    RecipeStepId,
}

#[cfg(test)]
pub mod tests {
    use pretty_assertions::{assert_eq, assert_str_eq};
    use sea_orm::DatabaseConnection;

    use crate::database::tests::{get_table_indices, get_table_schema};

    pub async fn assert_recipe_file_schema(db: &DatabaseConnection) {
        let table_schema = get_table_schema("recipe_file", db).await;
        assert_str_eq!(
            table_schema,
            "CREATE TABLE \"recipe_file\" ( \
        \"id\" integer NOT NULL PRIMARY KEY AUTOINCREMENT, \
        \"name\" text NOT NULL, \
        \"order\" integer NOT NULL, \
        \"mime\" text NOT NULL, \
        \"path\" text NOT NULL, \
        \"recipe_step_id\" integer NOT NULL, \
        UNIQUE (\"order\", \"recipe_step_id\"), \
        FOREIGN KEY (\"recipe_step_id\") REFERENCES \"recipe_step\" (\"id\") ON DELETE CASCADE \
        )"
        );
    }

    pub async fn assert_recipe_file_indices(db: &DatabaseConnection) {
        let indices = get_table_indices("recipe_file", db).await;
        assert_eq!(
            indices,
            vec![
                String::from(
                    "CREATE INDEX \"idx-recipe_file-order\" ON \"recipe_file\" (\"order\")"
                ),
                String::from(
                    "CREATE INDEX \"idx-recipe_file-recipe_step_id\" ON \"recipe_file\" (\"recipe_step_id\")"
                ),
            ]
        )
    }
}
