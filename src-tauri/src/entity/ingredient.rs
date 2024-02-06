//! This module implements the ingredient entity.
//!
//! See [`Model`] for more information.

use sea_orm::entity::prelude::*;
use serde::Serialize;

/// This struct represents an ingredient.
///
/// _Flour_ is an ingredient. A _pot_ is not an ingredient.
///
/// An ingredient is used up in a recipe.
/// An ingredient is identifiable by its name.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "ingredient")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::recipe_step_ingredient::Entity")]
    RecipeStepIngredient,
}

impl Related<super::recipe_step_ingredient::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecipeStepIngredient.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
