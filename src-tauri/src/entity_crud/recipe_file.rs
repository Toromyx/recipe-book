//! This module implements [`EntityCrudTrait`] for [`crate::entity::recipe_file`].

use std::{collections::HashSet, sync::OnceLock};

use async_trait::async_trait;
use milli::Index;
use sea_orm::{
    sea_query::IntoCondition, ActiveValue, ColumnTrait, Condition, DeriveIntoActiveModel,
    IntoActiveModel, QueryOrder, Select,
};
use sea_query::IntoIden;
use serde::{Deserialize, Serialize};

use crate::{
    entity::recipe_file::{ActiveModel, Column, Entity, Model, PrimaryKey, Relation},
    entity_crud::{EntityCrudTrait, EntityDocumentTrait, Filter, Order, OrderBy},
    event::channel::{
        ENTITY_ACTION_CREATED_RECIPE_FILE, ENTITY_ACTION_DELETED_RECIPE_FILE,
        ENTITY_ACTION_UPDATED_RECIPE_FILE,
    },
};

static SEARCH_INDEX_ONCE: OnceLock<Index> = OnceLock::new();

#[derive(Debug, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct RecipeFileCreate {
    pub order: i64,
    pub recipe_id: i64,
    pub file_id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeFileUpdate {
    pub id: i64,
    pub order: Option<i64>,
}

impl IntoActiveModel<ActiveModel> for RecipeFileUpdate {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: ActiveValue::Unchanged(self.id),
            order: match self.order {
                Some(order) => ActiveValue::Set(order),
                _ => ActiveValue::NotSet,
            },
            recipe_id: ActiveValue::NotSet,
            file_id: ActiveValue::NotSet,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RecipeFileDocument {}

impl EntityDocumentTrait for RecipeFileDocument {
    type Model = Model;

    fn from_model(_model: Self::Model) -> Self {
        todo!()
    }
}

pub type RecipeFileFilter = Filter<RecipeFileCondition, RecipeFileOrderBy>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeFileCondition {
    pub recipe_id: Option<i64>,
}

impl IntoCondition for RecipeFileCondition {
    fn into_condition(self) -> Condition {
        Condition::all().add_option(
            self.recipe_id
                .map(|recipe_id| Column::RecipeId.eq(recipe_id)),
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecipeFileOrderBy {
    Order(Order),
}

impl OrderBy for RecipeFileOrderBy {
    type Entity = Entity;

    fn add(self, select: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            RecipeFileOrderBy::Order(order) => select.order_by(Column::Order, order.into()),
        }
    }
}

pub struct RecipeFileCrud {}

#[async_trait]
impl EntityCrudTrait for RecipeFileCrud {
    type Entity = Entity;
    type Model = Model;
    type ActiveModel = ActiveModel;
    type Column = Column;
    type Relation = Relation;
    type PrimaryKey = PrimaryKey;
    type PrimaryKeyValue = i64;
    type EntityCreate = RecipeFileCreate;
    type EntityUpdate = RecipeFileUpdate;
    type EntityDocument = RecipeFileDocument;
    type EntityCondition = RecipeFileCondition;
    type EntityOrderBy = RecipeFileOrderBy;

    fn primary_key_value(model: &Model) -> i64 {
        model.id
    }

    fn primary_key_colum() -> Column {
        Column::Id
    }

    fn entity_action_created_channel() -> &'static str {
        ENTITY_ACTION_CREATED_RECIPE_FILE
    }

    fn entity_action_updated_channel() -> &'static str {
        ENTITY_ACTION_UPDATED_RECIPE_FILE
    }

    fn entity_action_deleted_channel() -> &'static str {
        ENTITY_ACTION_DELETED_RECIPE_FILE
    }

    fn searchable_fields() -> Vec<String> {
        vec![]
    }

    fn filterable_fields() -> HashSet<String> {
        HashSet::from([
            Column::RecipeId.into_iden().to_string(),
            Column::FileId.into_iden().to_string(),
        ])
    }

    fn sortable_fields() -> HashSet<String> {
        HashSet::from([Column::Order.into_iden().to_string()])
    }

    fn search_index_once() -> &'static OnceLock<Index> {
        &SEARCH_INDEX_ONCE
    }
}
