//! This module implements the initial database migration for version 1.0.
//!
//! It creates all entities.

use sea_orm_migration::prelude::*;

mod ingredient;
mod recipe;
mod recipe_file;
mod recipe_ingredient;
mod recipe_step;

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
        Ok(())
    }
}
