//! This module implements the recipe entity.
//!
//! See [`Model`] for more information.

use async_trait::async_trait;
use sea_orm::{entity::prelude::*, IntoActiveModel, TryIntoModel};
use serde::Serialize;

/// This struct represents a recipe.
///
/// A recipe is a series of instructions.
/// A recipe is identifiable by its name.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "recipe")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::recipe_step::Entity")]
    RecipeStep,
}

impl Related<super::recipe_step::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecipeStep.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_delete<C>(self, db: &C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let model = self.clone().try_into_model()?;
        let recipe_steps = model
            .find_related(super::recipe_step::Entity)
            .all(db)
            .await?;
        for recipe_step in recipe_steps {
            recipe_step.into_active_model().before_delete(db).await?;
        }
        Ok(self)
    }
}
