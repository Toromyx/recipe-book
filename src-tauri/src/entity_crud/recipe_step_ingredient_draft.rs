//! This module implements [`EntityCrudTrait`] for [`crate::entity::recipe_step_ingredient_draft`].

use sea_orm::{
    sea_query::IntoCondition, ActiveValue, ColumnTrait, Condition, DeriveIntoActiveModel,
    IntoActiveModel, QueryOrder, Select,
};
use serde::Deserialize;

use crate::{
    entity::recipe_step_ingredient_draft::{
        ActiveModel, Column, Entity, Model, PrimaryKey, Relation,
    },
    entity_crud::{EntityCrudTrait, Filter, Order, OrderBy},
    event::channel::{
        ENTITY_ACTION_CREATED_RECIPE_STEP_INGREDIENT_DRAFT,
        ENTITY_ACTION_DELETED_RECIPE_STEP_INGREDIENT_DRAFT,
        ENTITY_ACTION_UPDATED_RECIPE_STEP_INGREDIENT_DRAFT,
    },
};

#[derive(Debug, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct RecipeStepIngredientDraftCreate {
    pub order: i64,
    pub text: String,
    pub recipe_step_id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeStepIngredientDraftUpdate {
    pub id: i64,
    pub order: Option<i64>,
    pub text: Option<String>,
}

impl IntoActiveModel<ActiveModel> for RecipeStepIngredientDraftUpdate {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: ActiveValue::Unchanged(self.id),
            order: match self.order {
                Some(order) => ActiveValue::Set(order),
                _ => ActiveValue::NotSet,
            },
            text: match self.text {
                Some(text) => ActiveValue::Set(text),
                _ => ActiveValue::NotSet,
            },
            recipe_step_id: ActiveValue::NotSet,
        }
    }
}

pub type RecipeStepIngredientDraftFilter =
    Filter<RecipeStepIngredientDraftCondition, RecipeStepIngredientDraftOrderBy>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeStepIngredientDraftCondition {
    pub recipe_step_id: Option<i64>,
}

impl IntoCondition for RecipeStepIngredientDraftCondition {
    fn into_condition(self) -> Condition {
        Condition::all().add_option(
            self.recipe_step_id
                .map(|recipe_step_id| Column::RecipeStepId.eq(recipe_step_id)),
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecipeStepIngredientDraftOrderBy {
    Order(Order),
}

impl OrderBy for RecipeStepIngredientDraftOrderBy {
    type Entity = Entity;

    fn add(self, select: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            RecipeStepIngredientDraftOrderBy::Order(order) => {
                select.order_by(Column::Order, order.into())
            }
        }
    }
}

pub struct RecipeStepIngredientDraftCrud {}

impl EntityCrudTrait for RecipeStepIngredientDraftCrud {
    type Entity = Entity;
    type Model = Model;
    type ActiveModel = ActiveModel;
    type Column = Column;
    type Relation = Relation;
    type PrimaryKey = PrimaryKey;
    type EntityCreate = RecipeStepIngredientDraftCreate;
    type EntityUpdate = RecipeStepIngredientDraftUpdate;
    type EntityCondition = RecipeStepIngredientDraftCondition;
    type EntityOrderBy = RecipeStepIngredientDraftOrderBy;

    fn primary_key_value(model: &Model) -> i64 {
        model.id
    }

    fn primary_key_colum() -> Column {
        Column::Id
    }

    fn entity_action_created_channel() -> &'static str {
        ENTITY_ACTION_CREATED_RECIPE_STEP_INGREDIENT_DRAFT
    }

    fn entity_action_updated_channel() -> &'static str {
        ENTITY_ACTION_UPDATED_RECIPE_STEP_INGREDIENT_DRAFT
    }

    fn entity_action_deleted_channel() -> &'static str {
        ENTITY_ACTION_DELETED_RECIPE_STEP_INGREDIENT_DRAFT
    }
}
