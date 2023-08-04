//! This module implements [`EntityCrudTrait`] for [`crate::entity::recipe`].

use sea_orm::{
    sea_query::IntoCondition,
    ActiveValue::{NotSet, Set, Unchanged},
    ColumnTrait, Condition, DeriveIntoActiveModel, IntoActiveModel, QueryOrder, Select,
};
use serde::Deserialize;

use crate::{
    entity::recipe::{ActiveModel, Column, Entity, Model, PrimaryKey, Relation},
    entity_crud::{EntityCrudTrait, Filter, Order, OrderBy},
    event::channel::{
        ENTITY_ACTION_CREATED_RECIPE, ENTITY_ACTION_DELETED_RECIPE, ENTITY_ACTION_UPDATED_RECIPE,
    },
};

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
            id: Unchanged(self.id),
            name: match self.name {
                Some(name) => Set(name),
                _ => NotSet,
            },
        }
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
    type EntityCreate = RecipeCreate;
    type EntityUpdate = RecipeUpdate;
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
}
