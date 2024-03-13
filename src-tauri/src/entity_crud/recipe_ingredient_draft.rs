//! This module implements [`EntityCrudTrait`] for [`crate::entity::recipe_ingredient_draft`].

use sea_orm::{
    sea_query::IntoCondition, ActiveValue, ColumnTrait, Condition, DeriveIntoActiveModel,
    IntoActiveModel, QueryOrder, Select,
};
use serde::Deserialize;

use crate::{
    entity::recipe_ingredient_draft::{ActiveModel, Column, Entity, Model, PrimaryKey, Relation},
    entity_crud::{EntityCrudTrait, Filter, Order, OrderBy},
    event::channel::{
        ENTITY_ACTION_CREATED_RECIPE_INGREDIENT_DRAFT,
        ENTITY_ACTION_DELETED_RECIPE_INGREDIENT_DRAFT,
        ENTITY_ACTION_UPDATED_RECIPE_INGREDIENT_DRAFT,
    },
};

#[derive(Debug, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct RecipeIngredientDraftCreate {
    pub order: i64,
    pub text: String,
    pub recipe_id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeIngredientDraftUpdate {
    pub id: i64,
    pub order: Option<i64>,
    pub text: Option<String>,
}

impl IntoActiveModel<ActiveModel> for RecipeIngredientDraftUpdate {
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
            recipe_id: ActiveValue::NotSet,
        }
    }
}

pub type RecipeIngredientDraftFilter =
    Filter<RecipeIngredientDraftCondition, RecipeIngredientDraftOrderBy>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeIngredientDraftCondition {
    pub recipe_id: Option<i64>,
}

impl IntoCondition for RecipeIngredientDraftCondition {
    fn into_condition(self) -> Condition {
        Condition::all().add_option(
            self.recipe_id
                .map(|recipe_id| Column::RecipeId.eq(recipe_id)),
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecipeIngredientDraftOrderBy {
    Order(Order),
}

impl OrderBy for RecipeIngredientDraftOrderBy {
    type Entity = Entity;

    fn add(self, select: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            RecipeIngredientDraftOrderBy::Order(order) => {
                select.order_by(Column::Order, order.into())
            }
        }
    }
}

pub struct RecipeIngredientDraftCrud {}

impl EntityCrudTrait for RecipeIngredientDraftCrud {
    type Entity = Entity;
    type Model = Model;
    type ActiveModel = ActiveModel;
    type Column = Column;
    type Relation = Relation;
    type PrimaryKey = PrimaryKey;
    type EntityCreate = RecipeIngredientDraftCreate;
    type EntityUpdate = RecipeIngredientDraftUpdate;
    type EntityCondition = RecipeIngredientDraftCondition;
    type EntityOrderBy = RecipeIngredientDraftOrderBy;

    fn primary_key_value(model: &Model) -> i64 {
        model.id
    }

    fn primary_key_colum() -> Column {
        Column::Id
    }

    fn entity_action_created_channel() -> &'static str {
        ENTITY_ACTION_CREATED_RECIPE_INGREDIENT_DRAFT
    }

    fn entity_action_updated_channel() -> &'static str {
        ENTITY_ACTION_UPDATED_RECIPE_INGREDIENT_DRAFT
    }

    fn entity_action_deleted_channel() -> &'static str {
        ENTITY_ACTION_DELETED_RECIPE_INGREDIENT_DRAFT
    }
}
