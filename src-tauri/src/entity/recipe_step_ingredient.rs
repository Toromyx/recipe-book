//! This module implements the recipe step ingredient entity.
//!
//! See [`Model`] for more information.

use sea_orm::entity::prelude::*;
use serde::Serialize;

/// This struct represents a recipe step ingredient.
///
/// A recipe step ingredient describes the quantity and quality of an [`super::ingredient::Model`] to be used in a [`super::recipe_step::Model`].
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "recipe_step_ingredient")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub order: i64,
    pub quantity: Option<f64>,
    pub unit: Option<String>,
    pub quality: Option<String>,
    pub recipe_step_id: i64,
    pub ingredient_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ingredient::Entity",
        from = "Column::IngredientId",
        to = "super::ingredient::Column::Id",
        on_update = "NoAction",
        on_delete = "Restrict"
    )]
    Ingredient,
    #[sea_orm(
        belongs_to = "super::recipe_step::Entity",
        from = "Column::RecipeStepId",
        to = "super::recipe_step::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    RecipeStep,
}

impl Related<super::ingredient::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ingredient.def()
    }
}

impl Related<super::recipe_step::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecipeStep.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
