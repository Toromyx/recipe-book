use sea_orm_migration::prelude::*;

mod m20230306_214922_1_0_0;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20230306_214922_1_0_0::Migration)]
    }
}

pub fn index_name<'a>(table: &'a dyn Iden, col: &'a dyn Iden) -> String {
    format!(
        "idx-{table_string}-{col_string}",
        table_string = table.to_string(),
        col_string = col.to_string()
    )
}
