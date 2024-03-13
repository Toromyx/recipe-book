//! This module implements [`EntityCrudTrait`] for [`crate::entity::recipe_step_ingredient`].

use sea_orm::{
    sea_query::IntoCondition, ActiveValue, ColumnTrait, Condition, DeriveIntoActiveModel,
    IntoActiveModel, QueryOrder, Select,
};
use serde::Deserialize;

use crate::{
    entity::recipe_step_ingredient::{ActiveModel, Column, Entity, Model, PrimaryKey, Relation},
    entity_crud::{EntityCrudTrait, Filter, Order, OrderBy},
    event::channel::{
        ENTITY_ACTION_CREATED_RECIPE_STEP_INGREDIENT, ENTITY_ACTION_DELETED_RECIPE_STEP_INGREDIENT,
        ENTITY_ACTION_UPDATED_RECIPE_STEP_INGREDIENT,
    },
};

#[derive(Debug, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct RecipeStepIngredientCreate {
    pub order: i64,
    pub quantity: Option<f64>,
    pub unit: Option<String>,
    pub quality: Option<String>,
    pub recipe_step_id: i64,
    pub ingredient_id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeStepIngredientUpdate {
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

impl IntoActiveModel<ActiveModel> for RecipeStepIngredientUpdate {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: ActiveValue::Unchanged(self.id),
            order: match self.order {
                Some(order) => ActiveValue::Set(order),
                _ => ActiveValue::NotSet,
            },
            quantity: match self.quantity {
                Some(quantity) => ActiveValue::Set(quantity),
                _ => ActiveValue::NotSet,
            },
            unit: match self.unit {
                Some(unit) => ActiveValue::Set(unit),
                _ => ActiveValue::NotSet,
            },
            quality: match self.quality {
                Some(quality) => ActiveValue::Set(quality),
                _ => ActiveValue::NotSet,
            },
            recipe_step_id: ActiveValue::NotSet,
            ingredient_id: match self.ingredient_id {
                Some(ingredient_id) => ActiveValue::Set(ingredient_id),
                _ => ActiveValue::NotSet,
            },
        }
    }
}

pub type RecipeStepIngredientFilter =
    Filter<RecipeStepIngredientCondition, RecipeStepIngredientOrderBy>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeStepIngredientCondition {
    pub recipe_step_id: Option<i64>,
    pub ingredient_id: Option<i64>,
}

impl IntoCondition for RecipeStepIngredientCondition {
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
pub enum RecipeStepIngredientOrderBy {
    Order(Order),
}

impl OrderBy for RecipeStepIngredientOrderBy {
    type Entity = Entity;

    fn add(self, select: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            RecipeStepIngredientOrderBy::Order(order) => {
                select.order_by(Column::Order, order.into())
            }
        }
    }
}

pub struct RecipeStepIngredientCrud {}

impl EntityCrudTrait for RecipeStepIngredientCrud {
    type Entity = Entity;
    type Model = Model;
    type ActiveModel = ActiveModel;
    type Column = Column;
    type Relation = Relation;
    type PrimaryKey = PrimaryKey;
    type EntityCreate = RecipeStepIngredientCreate;
    type EntityUpdate = RecipeStepIngredientUpdate;
    type EntityCondition = RecipeStepIngredientCondition;
    type EntityOrderBy = RecipeStepIngredientOrderBy;

    fn primary_key_value(model: &Model) -> i64 {
        model.id
    }

    fn primary_key_colum() -> Column {
        Column::Id
    }

    fn entity_action_created_channel() -> &'static str {
        ENTITY_ACTION_CREATED_RECIPE_STEP_INGREDIENT
    }

    fn entity_action_updated_channel() -> &'static str {
        ENTITY_ACTION_UPDATED_RECIPE_STEP_INGREDIENT
    }

    fn entity_action_deleted_channel() -> &'static str {
        ENTITY_ACTION_DELETED_RECIPE_STEP_INGREDIENT
    }
}
