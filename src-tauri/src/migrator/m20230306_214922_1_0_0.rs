//! This module implements the initial database migration for version 1.0.
//!
//! It creates all entities.

use sea_orm_migration::prelude::*;

mod ingredient;
mod recipe;
mod recipe_file;
mod recipe_ingredient;
mod recipe_ingredient_draft;
mod recipe_step;
mod unit_name;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        ingredient::up(manager).await?;
        recipe::up(manager).await?;
        recipe_step::up(manager).await?;
        recipe_file::up(manager).await?;
        recipe_ingredient::up(manager).await?;
        recipe_ingredient_draft::up(manager).await?;
        unit_name::up(manager).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sea_orm_migration::SchemaManager;

    use super::*;
    use crate::database::tests::get_memory_database;

    #[tokio::test]
    pub async fn test_up() {
        let db = get_memory_database().await;
        let schema_manager = SchemaManager::new(&db);
        let migration = Migration {};
        migration.up(&schema_manager).await.unwrap();
        ingredient::tests::test_ingredient_schema(&db).await;
        ingredient::tests::test_ingredient_indices(&db).await;
        recipe::tests::test_recipe_schema(&db).await;
        recipe::tests::test_recipe_indices(&db).await;
        recipe_step::tests::test_recipe_step_schema(&db).await;
        recipe_step::tests::test_recipe_step_indices(&db).await;
        recipe_file::tests::test_recipe_file_schema(&db).await;
        recipe_file::tests::test_recipe_file_indices(&db).await;
        recipe_ingredient::tests::test_recipe_ingredient_schema(&db).await;
        recipe_ingredient::tests::test_recipe_ingredient_indices(&db).await;
        recipe_ingredient_draft::tests::test_recipe_ingredient_draft_schema(&db).await;
        recipe_ingredient_draft::tests::test_recipe_ingredient_draft_indices(&db).await;
        unit_name::tests::test_unit_name_schema(&db).await;
        unit_name::tests::test_unit_name_indices(&db).await;
        unit_name::tests::test_unit_name_content(&db).await;
    }
}
