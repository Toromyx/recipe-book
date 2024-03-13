//! This module implements the creation of [`crate::entity::recipe_file`].

use sea_orm_migration::prelude::*;

use crate::migrator::{
    index_name,
    m20230306_214922_1_0_0::{file::File, recipe::Recipe},
};

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
                .col(ColumnDef::new(RecipeFile::Order).integer().not_null())
                .col(ColumnDef::new(RecipeFile::RecipeId).integer().not_null())
                .col(ColumnDef::new(RecipeFile::FileId).integer().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .from(RecipeFile::Table, RecipeFile::RecipeId)
                        .to(Recipe::Table, Recipe::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(RecipeFile::Table, RecipeFile::FileId)
                        .to(File::Table, File::Id)
                        .on_delete(ForeignKeyAction::Restrict),
                )
                .index(
                    Index::create()
                        .col(RecipeFile::Order)
                        .col(RecipeFile::RecipeId)
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
                .name(&index_name(&RecipeFile::Table, &RecipeFile::RecipeId))
                .table(RecipeFile::Table)
                .col(RecipeFile::RecipeId)
                .to_owned(),
        )
        .await?;
    manager
        .create_index(
            Index::create()
                .name(&index_name(&RecipeFile::Table, &RecipeFile::FileId))
                .table(RecipeFile::Table)
                .col(RecipeFile::FileId)
                .to_owned(),
        )
        .await?;
    Ok(())
}

#[derive(Iden)]
enum RecipeFile {
    Table,
    Id,
    Order,
    RecipeId,
    FileId,
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
            \"order\" integer NOT NULL, \
            \"recipe_id\" integer NOT NULL, \
            \"file_id\" integer NOT NULL, \
            UNIQUE (\"order\", \"recipe_id\"), \
            FOREIGN KEY (\"recipe_id\") REFERENCES \"recipe\" (\"id\") ON DELETE CASCADE, \
            FOREIGN KEY (\"file_id\") REFERENCES \"file\" (\"id\") ON DELETE RESTRICT \
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
                    "CREATE INDEX \"idx-recipe_file-recipe_id\" ON \"recipe_file\" (\"recipe_id\")"
                ),
                String::from(
                    "CREATE INDEX \"idx-recipe_file-file_id\" ON \"recipe_file\" (\"file_id\")"
                ),
            ]
        )
    }
}
