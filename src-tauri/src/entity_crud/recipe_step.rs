//! This module implements [`EntityCrudTrait`] for [`crate::entity::recipe_step`].

use sea_orm::{
    sea_query::IntoCondition,
    ActiveValue::{NotSet, Set, Unchanged},
    ColumnTrait, Condition, DeriveIntoActiveModel, IntoActiveModel, QueryOrder, Select,
};
use serde::Deserialize;

use crate::{
    entity::recipe_step::{ActiveModel, Column, Entity, Model, PrimaryKey, Relation},
    entity_crud::{EntityCrudTrait, Filter, Order, OrderBy},
    event::channel::{
        ENTITY_ACTION_CREATED_RECIPE_STEP, ENTITY_ACTION_DELETED_RECIPE_STEP,
        ENTITY_ACTION_UPDATED_RECIPE_STEP,
    },
};

#[derive(Debug, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct RecipeStepCreate {
    pub order: i64,
    pub description: String,
    pub recipe_id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeStepUpdate {
    pub id: i64,
    pub order: Option<i64>,
    pub description: Option<String>,
}

impl IntoActiveModel<ActiveModel> for RecipeStepUpdate {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: Unchanged(self.id),
            order: match self.order {
                Some(order) => Set(order),
                _ => NotSet,
            },
            description: match self.description {
                Some(description) => Set(description),
                _ => NotSet,
            },
            recipe_id: NotSet,
        }
    }
}

pub type RecipeStepFilter = Filter<RecipeStepCondition, RecipeStepOrderBy>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeStepCondition {
    pub recipe_id: Option<i64>,
}

impl IntoCondition for RecipeStepCondition {
    fn into_condition(self) -> Condition {
        Condition::all().add_option(
            self.recipe_id
                .map(|recipe_id| Column::RecipeId.eq(recipe_id)),
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecipeStepOrderBy {
    Order(Order),
}

impl OrderBy for RecipeStepOrderBy {
    type Entity = Entity;

    fn add(self, select: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            RecipeStepOrderBy::Order(order) => select.order_by(Column::Order, order.into()),
        }
    }
}

pub struct RecipeStepCrud {}

impl EntityCrudTrait for RecipeStepCrud {
    type Entity = Entity;
    type Model = Model;
    type ActiveModel = ActiveModel;
    type Column = Column;
    type Relation = Relation;
    type PrimaryKey = PrimaryKey;
    type EntityCreate = RecipeStepCreate;
    type EntityUpdate = RecipeStepUpdate;
    type EntityCondition = RecipeStepCondition;
    type EntityOrderBy = RecipeStepOrderBy;

    fn primary_key_value(model: &Model) -> i64 {
        model.id
    }

    fn primary_key_colum() -> Column {
        Column::Id
    }

    fn entity_action_created_channel() -> &'static str {
        ENTITY_ACTION_CREATED_RECIPE_STEP
    }

    fn entity_action_updated_channel() -> &'static str {
        ENTITY_ACTION_UPDATED_RECIPE_STEP
    }

    fn entity_action_deleted_channel() -> &'static str {
        ENTITY_ACTION_DELETED_RECIPE_STEP
    }
}
