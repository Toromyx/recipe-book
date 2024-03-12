//! This module implements the creation of [`crate::entity::recipe_step_file`].

use sea_orm_migration::prelude::*;

use crate::migrator::{
    index_name,
    m20230306_214922_1_0_0::{file::File, recipe_step::RecipeStep},
};

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(RecipeStepFile::Table)
                .col(
                    ColumnDef::new(RecipeStepFile::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(RecipeStepFile::Order).integer().not_null())
                .col(
                    ColumnDef::new(RecipeStepFile::RecipeStepId)
                        .integer()
                        .not_null(),
                )
                .col(ColumnDef::new(RecipeStepFile::FileId).integer().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .from(RecipeStepFile::Table, RecipeStepFile::RecipeStepId)
                        .to(RecipeStep::Table, RecipeStep::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(RecipeStepFile::Table, RecipeStepFile::FileId)
                        .to(File::Table, File::Id)
                        .on_delete(ForeignKeyAction::Restrict),
                )
                .index(
                    Index::create()
                        .col(RecipeStepFile::Order)
                        .col(RecipeStepFile::RecipeStepId)
                        .unique(),
                )
                .to_owned(),
        )
        .await?;
    manager
        .create_index(
            Index::create()
                .name(&index_name(&RecipeStepFile::Table, &RecipeStepFile::Order))
                .table(RecipeStepFile::Table)
                .col(RecipeStepFile::Order)
                .to_owned(),
        )
        .await?;
    manager
        .create_index(
            Index::create()
                .name(&index_name(
                    &RecipeStepFile::Table,
                    &RecipeStepFile::RecipeStepId,
                ))
                .table(RecipeStepFile::Table)
                .col(RecipeStepFile::RecipeStepId)
                .to_owned(),
        )
        .await?;
    manager
        .create_index(
            Index::create()
                .name(&index_name(&RecipeStepFile::Table, &RecipeStepFile::FileId))
                .table(RecipeStepFile::Table)
                .col(RecipeStepFile::FileId)
                .to_owned(),
        )
        .await?;
    Ok(())
}

#[derive(Iden)]
enum RecipeStepFile {
    Table,
    Id,
    Order,
    RecipeStepId,
    FileId,
}

#[cfg(test)]
pub mod tests {
    use pretty_assertions::{assert_eq, assert_str_eq};
    use sea_orm::DatabaseConnection;

    use crate::database::tests::{get_table_indices, get_table_schema};

    pub async fn assert_recipe_step_file_schema(db: &DatabaseConnection) {
        let table_schema = get_table_schema("recipe_step_file", db).await;
        assert_str_eq!(
            table_schema,
            "CREATE TABLE \"recipe_step_file\" ( \
            \"id\" integer NOT NULL PRIMARY KEY AUTOINCREMENT, \
            \"order\" integer NOT NULL, \
            \"recipe_step_id\" integer NOT NULL, \
            \"file_id\" integer NOT NULL, \
            UNIQUE (\"order\", \"recipe_step_id\"), \
            FOREIGN KEY (\"recipe_step_id\") REFERENCES \"recipe_step\" (\"id\") ON DELETE CASCADE, \
            FOREIGN KEY (\"file_id\") REFERENCES \"file\" (\"id\") ON DELETE RESTRICT \
            )"
        );
    }

    pub async fn assert_recipe_step_file_indices(db: &DatabaseConnection) {
        let indices = get_table_indices("recipe_step_file", db).await;
        assert_eq!(
            indices,
            vec![
                String::from(
                    "CREATE INDEX \"idx-recipe_step_file-order\" ON \"recipe_step_file\" (\"order\")"
                ),
                String::from(
                    "CREATE INDEX \"idx-recipe_step_file-recipe_step_id\" ON \"recipe_step_file\" (\"recipe_step_id\")"
                ),
                String::from(
                    "CREATE INDEX \"idx-recipe_step_file-file_id\" ON \"recipe_step_file\" (\"file_id\")"
                ),
            ]
        )
    }
}
