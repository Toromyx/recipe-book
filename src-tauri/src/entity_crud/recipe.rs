//! This module implements [`EntityCrudTrait`] for [`crate::entity::recipe`].

use std::{collections::HashSet, sync::OnceLock};

use milli::Index;
use sea_orm::{
    sea_query::IntoCondition, ActiveValue, ColumnTrait, Condition, DeriveIntoActiveModel,
    IntoActiveModel, QueryOrder, Select,
};
use sea_query::IntoIden;
use serde::{Deserialize, Serialize};

use crate::{
    entity::recipe::{ActiveModel, Column, Entity, Model, PrimaryKey, Relation},
    entity_crud::{EntityCrudTrait, EntityDocumentTrait, Filter, Order, OrderBy},
    event::channel::{
        ENTITY_ACTION_CREATED_RECIPE, ENTITY_ACTION_DELETED_RECIPE, ENTITY_ACTION_UPDATED_RECIPE,
    },
};

static SEARCH_INDEX_ONCE: OnceLock<Index> = OnceLock::new();

#[derive(Debug, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct RecipeCreate {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeUpdate {
    pub id: i64,
    pub name: Option<String>,
}

impl IntoActiveModel<ActiveModel> for RecipeUpdate {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: ActiveValue::Unchanged(self.id),
            name: match self.name {
                Some(name) => ActiveValue::Set(name),
                _ => ActiveValue::NotSet,
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RecipeDocument {}

impl EntityDocumentTrait for RecipeDocument {
    type Model = Model;

    fn from_model(_model: Self::Model) -> Self {
        todo!()
    }
}

pub type RecipeFilter = Filter<RecipeCondition, RecipeOrderBy>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeCondition {
    pub name: Option<String>,
}

impl IntoCondition for RecipeCondition {
    fn into_condition(self) -> Condition {
        Condition::all().add_option(self.name.map(|name| Column::Name.like(format!("%{name}%"))))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecipeOrderBy {
    Name(Order),
}

impl OrderBy for RecipeOrderBy {
    type Entity = Entity;

    fn add(self, select: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            RecipeOrderBy::Name(order) => select.order_by(Column::Name, order.into()),
        }
    }
}

pub struct RecipeCrud {}

impl EntityCrudTrait for RecipeCrud {
    type Entity = Entity;
    type Model = Model;
    type ActiveModel = ActiveModel;
    type Column = Column;
    type Relation = Relation;
    type PrimaryKey = PrimaryKey;
    type PrimaryKeyValue = i64;
    type EntityCreate = RecipeCreate;
    type EntityUpdate = RecipeUpdate;
    type EntityDocument = RecipeDocument;
    type EntityCondition = RecipeCondition;
    type EntityOrderBy = RecipeOrderBy;

    fn primary_key_value(model: &Model) -> i64 {
        model.id
    }

    fn primary_key_colum() -> Column {
        Column::Id
    }

    fn entity_action_created_channel() -> &'static str {
        ENTITY_ACTION_CREATED_RECIPE
    }

    fn entity_action_updated_channel() -> &'static str {
        ENTITY_ACTION_UPDATED_RECIPE
    }

    fn entity_action_deleted_channel() -> &'static str {
        ENTITY_ACTION_DELETED_RECIPE
    }

    fn searchable_fields() -> Vec<String> {
        vec![Column::Name.into_iden().to_string()]
    }

    fn filterable_fields() -> HashSet<String> {
        HashSet::from([])
    }

    fn sortable_fields() -> HashSet<String> {
        HashSet::from([Column::Name.into_iden().to_string()])
    }

    fn search_index_once() -> &'static OnceLock<Index> {
        &SEARCH_INDEX_ONCE
    }
}
