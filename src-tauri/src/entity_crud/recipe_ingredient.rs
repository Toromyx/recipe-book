//! This module implements [`EntityCrudTrait`] for [`crate::entity::recipe_ingredient`].

use sea_orm::{
    sea_query::IntoCondition,
    ActiveValue::{NotSet, Set, Unchanged},
    ColumnTrait, Condition, DeriveIntoActiveModel, IntoActiveModel,
};
use serde::Deserialize;

use crate::{
    entity::recipe_ingredient::{ActiveModel, Column, Entity, Model, PrimaryKey, Relation},
    entity_crud::{EntityCrudTrait, Filter},
    event::channel::{
        ENTITY_ACTION_CREATED_RECIPE_INGREDIENT, ENTITY_ACTION_DELETED_RECIPE_INGREDIENT,
        ENTITY_ACTION_UPDATED_RECIPE_INGREDIENT,
    },
};

#[derive(Debug, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct RecipeIngredientCreate {
    pub order: i64,
    pub quantity: Option<f64>,
    pub unit: Option<String>,
    pub quality: Option<String>,
    pub recipe_step_id: i64,
    pub ingredient_id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeIngredientUpdate {
    pub id: i64,
    pub order: Option<i64>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub quantity: Option<Option<f64>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub unit: Option<Option<String>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub quality: Option<Option<String>>,
    pub ingredient_id: Option<i64>,
}

impl IntoActiveModel<ActiveModel> for RecipeIngredientUpdate {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: Unchanged(self.id),
            order: match self.order {
                Some(order) => Set(order),
                _ => NotSet,
            },
            quantity: match self.quantity {
                Some(quantity) => Set(quantity),
                _ => NotSet,
            },
            unit: match self.unit {
                Some(unit) => Set(unit),
                _ => NotSet,
            },
            quality: match self.quality {
                Some(quality) => Set(quality),
                _ => NotSet,
            },
            recipe_step_id: NotSet,
            ingredient_id: match self.ingredient_id {
                Some(ingredient_id) => Set(ingredient_id),
                _ => NotSet,
            },
        }
    }
}

pub type RecipeIngredientFilter = Filter<RecipeIngredientCondition, RecipeIngredientOrderBy>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeIngredientCondition {
    pub recipe_step_id: Option<i64>,
    pub ingredient_id: Option<i64>,
}

impl IntoCondition for RecipeIngredientCondition {
    fn into_condition(self) -> Condition {
        Condition::all()
            .add_option(
                self.recipe_step_id
                    .map(|recipe_step_id| Column::RecipeStepId.eq(recipe_step_id)),
            )
            .add_option(
                self.ingredient_id
                    .map(|ingredient_id| Column::IngredientId.eq(ingredient_id)),
            )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecipeIngredientOrderBy {
    Order,
}

impl From<RecipeIngredientOrderBy> for Column {
    fn from(value: RecipeIngredientOrderBy) -> Self {
        match value {
            RecipeIngredientOrderBy::Order => Column::Order,
        }
    }
}

pub struct RecipeIngredientCrud {}

impl EntityCrudTrait for RecipeIngredientCrud {
    type Entity = Entity;
    type Model = Model;
    type ActiveModel = ActiveModel;
    type Column = Column;
    type Relation = Relation;
    type PrimaryKey = PrimaryKey;
    type EntityCreate = RecipeIngredientCreate;
    type EntityUpdate = RecipeIngredientUpdate;
    type EntityCondition = RecipeIngredientCondition;
    type EntityOrderBy = RecipeIngredientOrderBy;

    fn primary_key_value(model: &Model) -> i64 {
        model.id
    }

    fn primary_key_colum() -> Column {
        Column::Id
    }

    fn entity_action_created_channel() -> &'static str {
        ENTITY_ACTION_CREATED_RECIPE_INGREDIENT
    }

    fn entity_action_updated_channel() -> &'static str {
        ENTITY_ACTION_UPDATED_RECIPE_INGREDIENT
    }

    fn entity_action_deleted_channel() -> &'static str {
        ENTITY_ACTION_DELETED_RECIPE_INGREDIENT
    }
}
