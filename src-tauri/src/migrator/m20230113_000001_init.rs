use sea_orm_migration::prelude::*;

mod ingredient;
mod recipe;
mod recipe_ingredient;
mod recipe_step;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230113_000001_init"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        ingredient::up(manager).await?;
        recipe::up(manager).await?;
        recipe_step::up(manager).await?;
        recipe_ingredient::up(manager).await?;
        Ok(())
    }
}
