//! This module implements the recipe step entity.
//!
//! See [`Model`] for more information.

use sea_orm::entity::prelude::*;
use serde::Serialize;

/// This struct represents a recipe step.
///
/// A recipe step is a part of a [`super::recipe::Model`].
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "recipe_step")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub order: i64,
    pub description: String,
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
    #[sea_orm(has_many = "super::recipe_ingredient::Entity")]
    RecipeIngredient,
    #[sea_orm(has_many = "super::recipe_file::Entity")]
    RecipeFile,
}

impl Related<super::recipe::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Recipe.def()
    }
}

impl Related<super::recipe_ingredient::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecipeIngredient.def()
    }
}

impl Related<super::recipe_file::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecipeFile.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
