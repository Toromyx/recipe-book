use sea_orm_migration::prelude::*;

mod m20230113_000001_init;
mod m20230218_135537_index;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230113_000001_init::Migration),
            Box::new(m20230218_135537_index::Migration),
        ]
    }
}

pub fn index_name<'a>(table: &'a dyn Iden, col: &'a dyn Iden) -> String {
    format!(
        "idx-{table_string}-{col_string}",
        table_string = table.to_string(),
        col_string = col.to_string()
    )
}
