//! This module implements [`sea_orm`] migrations.
//!
//! Each migration lives in a sub-module.

use sea_orm_migration::prelude::*;

mod m20230306_214922_1_0_0;

/// This struct implements [`MigratorTrait`] to run the migrations via [`<Self as MigrationTrait>::up`].
pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20230306_214922_1_0_0::Migration)]
    }
}

/// Get an index name for a table and column.
pub fn index_name<'a>(table: &'a dyn Iden, col: &'a dyn Iden) -> String {
    format!(
        "idx-{table_string}-{col_string}",
        table_string = table.to_string(),
        col_string = col.to_string()
    )
}

#[cfg(test)]
pub mod tests {
    use sea_orm::DatabaseConnection;

    use super::*;
    use crate::database::tests::get_memory_database;

    pub async fn get_memory_database_migrated() -> DatabaseConnection {
        let db = get_memory_database().await;
        Migrator::up(&db, None).await.unwrap();
        db
    }
}
