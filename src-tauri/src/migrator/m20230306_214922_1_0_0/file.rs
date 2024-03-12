//! This module implements the creation of [`crate::entity::file`].

use sea_orm_migration::prelude::*;

use crate::migrator::index_name;

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(File::Table)
                .col(
                    ColumnDef::new(File::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(File::Name).string().not_null())
                .col(ColumnDef::new(File::Mime).string().not_null())
                .col(ColumnDef::new(File::Path).string().not_null())
                .to_owned(),
        )
        .await?;
    manager
        .create_index(
            Index::create()
                .name(&index_name(&File::Table, &File::Name))
                .table(File::Table)
                .col(File::Name)
                .to_owned(),
        )
        .await?;
    Ok(())
}

#[derive(Iden)]
pub enum File {
    Table,
    Id,
    Name,
    Mime,
    Path,
}

#[cfg(test)]
pub mod tests {
    use pretty_assertions::{assert_eq, assert_str_eq};
    use sea_orm::DatabaseConnection;

    use crate::database::tests::{get_table_indices, get_table_schema};

    pub async fn assert_file_schema(db: &DatabaseConnection) {
        let table_schema = get_table_schema("file", db).await;
        assert_str_eq!(
            table_schema,
            "CREATE TABLE \"file\" ( \
            \"id\" integer NOT NULL PRIMARY KEY AUTOINCREMENT, \
            \"name\" text NOT NULL, \
            \"mime\" text NOT NULL, \
            \"path\" text NOT NULL \
            )"
        );
    }

    pub async fn assert_file_indices(db: &DatabaseConnection) {
        let indices = get_table_indices("file", db).await;
        assert_eq!(
            indices,
            vec![String::from(
                "CREATE INDEX \"idx-file-name\" ON \"file\" (\"name\")"
            ),]
        )
    }
}
