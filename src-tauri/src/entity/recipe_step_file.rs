//! This module implements the recipe step file entity.
//!
//! See [`Model`] for more information.

use async_trait::async_trait;
use log;
use sea_orm::{entity::prelude::*, TryIntoModel};
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
    pub name: String,
    pub order: i64,
    pub mime: String,
    pub path: String,
    pub recipe_step_id: i64,
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
}

impl Related<super::recipe_step::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecipeStep.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_delete<C>(self, _db: &C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let model = self.clone().try_into_model()?;
        if let Err(err) = crate::recipe_step_file_storage::delete(&model).await {
            log::warn!(
                "Could not delete recipe step file from storage while deleting entity: {}",
                err
            );
        };
        Ok(self)
    }
}
