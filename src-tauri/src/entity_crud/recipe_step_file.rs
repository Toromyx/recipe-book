//! This module implements [`EntityCrudTrait`] for [`crate::entity::recipe_step_file`].

use async_trait::async_trait;
use sea_orm::{
    sea_query::IntoCondition,
    ActiveValue::{NotSet, Set, Unchanged},
    ColumnTrait, Condition, DeriveIntoActiveModel, IntoActiveModel, QueryOrder, Select,
};
use serde::Deserialize;

use crate::{
    entity::recipe_step_file::{ActiveModel, Column, Entity, Model, PrimaryKey, Relation},
    entity_crud::{EntityCrudTrait, Filter, Order, OrderBy},
    event::channel::{
        ENTITY_ACTION_CREATED_RECIPE_STEP_FILE, ENTITY_ACTION_DELETED_RECIPE_STEP_FILE,
        ENTITY_ACTION_UPDATED_RECIPE_STEP_FILE,
    },
};

#[derive(Debug, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct RecipeStepFileCreate {
    pub order: i64,
    pub recipe_step_id: i64,
    pub file_id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeStepFileUpdate {
    pub id: i64,
    pub order: Option<i64>,
}

impl IntoActiveModel<ActiveModel> for RecipeStepFileUpdate {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: Unchanged(self.id),
            order: match self.order {
                Some(order) => Set(order),
                _ => NotSet,
            },
            recipe_step_id: NotSet,
            file_id: NotSet,
        }
    }
}

pub type RecipeStepFileFilter = Filter<RecipeStepFileCondition, RecipeStepFileOrderBy>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeStepFileCondition {
    pub recipe_step_id: Option<i64>,
}

impl IntoCondition for RecipeStepFileCondition {
    fn into_condition(self) -> Condition {
        Condition::all().add_option(
            self.recipe_step_id
                .map(|recipe_step_id| Column::RecipeStepId.eq(recipe_step_id)),
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecipeStepFileOrderBy {
    Order(Order),
}

impl OrderBy for RecipeStepFileOrderBy {
    type Entity = Entity;

    fn add(self, select: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            RecipeStepFileOrderBy::Order(order) => select.order_by(Column::Order, order.into()),
        }
    }
}

pub struct RecipeStepFileCrud {}

#[async_trait]
impl EntityCrudTrait for RecipeStepFileCrud {
    type Entity = Entity;
    type Model = Model;
    type ActiveModel = ActiveModel;
    type Column = Column;
    type Relation = Relation;
    type PrimaryKey = PrimaryKey;
    type EntityCreate = RecipeStepFileCreate;
    type EntityUpdate = RecipeStepFileUpdate;
    type EntityCondition = RecipeStepFileCondition;
    type EntityOrderBy = RecipeStepFileOrderBy;

    fn primary_key_value(model: &Model) -> i64 {
        model.id
    }

    fn primary_key_colum() -> Column {
        Column::Id
    }

    fn entity_action_created_channel() -> &'static str {
        ENTITY_ACTION_CREATED_RECIPE_STEP_FILE
    }

    fn entity_action_updated_channel() -> &'static str {
        ENTITY_ACTION_UPDATED_RECIPE_STEP_FILE
    }

    fn entity_action_deleted_channel() -> &'static str {
        ENTITY_ACTION_DELETED_RECIPE_STEP_FILE
    }
}
