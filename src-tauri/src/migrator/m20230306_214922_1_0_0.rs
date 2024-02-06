//! This module implements the initial database migration for version 1.0.
//!
//! It creates all entities.

use sea_orm_migration::prelude::*;

mod ingredient;
mod recipe;
mod recipe_file;
mod recipe_step;
mod recipe_step_ingredient;
mod recipe_step_ingredient_draft;
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
        recipe_step_ingredient::up(manager).await?;
        recipe_step_ingredient_draft::up(manager).await?;
        unit_name::up(manager).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use ingredient::tests::{assert_ingredient_indices, assert_ingredient_schema};
    use recipe::tests::{assert_recipe_indices, assert_recipe_schema};
    use recipe_file::tests::{assert_recipe_file_indices, assert_recipe_file_schema};
    use recipe_step::tests::{assert_recipe_step_indices, assert_recipe_step_schema};
    use recipe_step_ingredient::tests::{
        assert_recipe_step_ingredient_indices, assert_recipe_step_ingredient_schema,
    };
    use recipe_step_ingredient_draft::tests::{
        assert_recipe_step_ingredient_draft_indices, assert_recipe_step_ingredient_draft_schema,
    };
    use sea_orm_migration::SchemaManager;
    use unit_name::tests::{
        assert_unit_name_content, assert_unit_name_indices, assert_unit_name_schema,
    };

    use super::*;
    use crate::database::tests::get_memory_database;

    #[tokio::test]
    pub async fn test_up() {
        let db = get_memory_database().await;
        let schema_manager = SchemaManager::new(&db);
        let migration = Migration {};
        migration.up(&schema_manager).await.unwrap();
        assert_ingredient_schema(&db).await;
        assert_ingredient_indices(&db).await;
        assert_recipe_schema(&db).await;
        assert_recipe_indices(&db).await;
        assert_recipe_step_schema(&db).await;
        assert_recipe_step_indices(&db).await;
        assert_recipe_file_schema(&db).await;
        assert_recipe_file_indices(&db).await;
        assert_recipe_step_ingredient_schema(&db).await;
        assert_recipe_step_ingredient_indices(&db).await;
        assert_recipe_step_ingredient_draft_schema(&db).await;
        assert_recipe_step_ingredient_draft_indices(&db).await;
        assert_unit_name_schema(&db).await;
        assert_unit_name_indices(&db).await;
        assert_unit_name_content(&db).await;
    }
}
