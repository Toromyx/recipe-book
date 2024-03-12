//! This module implements the recipe step file entity.
//!
//! See [`Model`] for more information.

use async_trait::async_trait;
use sea_orm::entity::prelude::*;
use serde::Serialize;

/// This struct represents a recipe step file.
///
/// A recipe step file is a supplementary binary file to a recipe step.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "recipe_step_file")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub order: i64,
    pub recipe_step_id: i64,
    pub file_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::recipe_step::Entity",
        from = "Column::RecipeStepId",
        to = "super::recipe_step::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    RecipeStep,
    #[sea_orm(
        belongs_to = "super::file::Entity",
        from = "Column::FileId",
        to = "super::file::Column::Id",
        on_update = "NoAction",
        on_delete = "Restrict"
    )]
    File,
}

impl Related<super::recipe_step::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecipeStep.def()
    }
}

impl Related<super::file::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::File.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn after_delete<C>(self, db: &C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        super::file::remove_orphans(db).await?;
        Ok(self)
    }
}
