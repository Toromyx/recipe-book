use sea_orm_migration::prelude::*;

mod m20230113_000001_init;
mod m20230218_135537_index;
mod m20230218_161605_image_type;
mod m20230221_221123_recipe_ingredient_optionals;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230113_000001_init::Migration),
            Box::new(m20230218_135537_index::Migration),
            Box::new(m20230218_161605_image_type::Migration),
            Box::new(m20230221_221123_recipe_ingredient_optionals::Migration),
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
