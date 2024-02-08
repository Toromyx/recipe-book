//! This module implements the recipe ingredient draft entity.
//!
//! See [`Model`] for more information.

use sea_orm::entity::prelude::*;
use serde::Serialize;

/// This struct represents a recipe ingredient draft.
///
/// Just like [`super::recipe_step_ingredient::Model`] it is not yet split up into quantity, unit, and does not include a reference to an [`super::ingredient::Model`].
/// But in contrast to [`super::recipe_step_ingredient::Model`] this entity is related to the recipe directly because a [`super::recipe_step::Model`] could not be determined.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "recipe_ingredient_draft")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub order: i64,
    pub text: String,
    pub recipe_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::recipe::Entity",
        from = "Column::RecipeId",
        to = "super::recipe::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Recipe,
}

impl Related<super::recipe::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Recipe.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
