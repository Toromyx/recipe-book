//! This module implements [`EntityCrudTrait`] for [`crate::entity::recipe_step`].

use std::{collections::HashSet, sync::OnceLock};

use milli::Index;
use sea_orm::{
    sea_query::IntoCondition, ActiveValue, ColumnTrait, Condition, DeriveIntoActiveModel,
    IntoActiveModel, QueryOrder, Select,
};
use sea_query::IntoIden;
use serde::{Deserialize, Serialize};

use crate::{
    entity::recipe_step::{ActiveModel, Column, Entity, Model, PrimaryKey, Relation},
    entity_crud::{EntityCrudTrait, EntityDocumentTrait, Filter, Order, OrderBy},
    event::channel::{
        ENTITY_ACTION_CREATED_RECIPE_STEP, ENTITY_ACTION_DELETED_RECIPE_STEP,
        ENTITY_ACTION_UPDATED_RECIPE_STEP,
    },
};

static SEARCH_INDEX_ONCE: OnceLock<Index> = OnceLock::new();

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
            id: ActiveValue::Unchanged(self.id),
            order: match self.order {
                Some(order) => ActiveValue::Set(order),
                _ => ActiveValue::NotSet,
            },
            description: match self.description {
                Some(description) => ActiveValue::Set(description),
                _ => ActiveValue::NotSet,
            },
            recipe_id: ActiveValue::NotSet,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RecipeStepDocument {}

impl EntityDocumentTrait for RecipeStepDocument {
    type Model = Model;

    fn from_model(_model: Self::Model) -> Self {
        todo!()
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
    type PrimaryKeyValue = i64;
    type EntityCreate = RecipeStepCreate;
    type EntityUpdate = RecipeStepUpdate;
    type EntityDocument = RecipeStepDocument;
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

    fn searchable_fields() -> Vec<String> {
        vec![]
    }

    fn filterable_fields() -> HashSet<String> {
        HashSet::from([Column::RecipeId.into_iden().to_string()])
    }

    fn sortable_fields() -> HashSet<String> {
        HashSet::from([Column::Order.into_iden().to_string()])
    }

    fn search_index_once() -> &'static OnceLock<Index> {
        &SEARCH_INDEX_ONCE
    }
}
