//! This module implements [`EntityCrudTrait`] for [`crate::entity::unit_name`].

use sea_orm::{
    sea_query::IntoCondition, ActiveValue, ColumnTrait, Condition, DeriveIntoActiveModel,
    IntoActiveModel, QueryOrder, Select,
};
use serde::Deserialize;

use crate::{
    entity::unit_name::{unit::Unit, ActiveModel, Column, Entity, Model, PrimaryKey, Relation},
    entity_crud::{EntityCrudTrait, Filter, Order, OrderBy},
    event::channel::{
        ENTITY_ACTION_CREATED_UNIT_NAME, ENTITY_ACTION_DELETED_UNIT_NAME,
        ENTITY_ACTION_UPDATED_UNIT_NAME,
    },
};

#[derive(Debug, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct UnitNameCreate {
    pub name: String,
    pub unit: Unit,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitNameUpdate {
    pub id: i64,
    pub name: Option<String>,
    pub unit: Option<Unit>,
}

impl IntoActiveModel<ActiveModel> for UnitNameUpdate {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: ActiveValue::Unchanged(self.id),
            name: match self.name {
                Some(name) => ActiveValue::Set(name),
                _ => ActiveValue::NotSet,
            },
            unit: match self.unit {
                Some(unit) => ActiveValue::Set(unit),
                _ => ActiveValue::NotSet,
            },
        }
    }
}

pub type UnitNameFilter = Filter<UnitNameCondition, UnitNameOrderBy>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitNameCondition {
    pub name_exact: Option<String>,
}

impl IntoCondition for UnitNameCondition {
    fn into_condition(self) -> Condition {
        Condition::all().add_option(self.name_exact.map(|name| Column::Name.eq(name)))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UnitNameOrderBy {
    Name(Order),
}

impl OrderBy for UnitNameOrderBy {
    type Entity = Entity;

    fn add(self, select: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            UnitNameOrderBy::Name(order) => select.order_by(Column::Name, order.into()),
        }
    }
}

pub struct UnitNameCrud {}

impl EntityCrudTrait for UnitNameCrud {
    type Entity = Entity;
    type Model = Model;
    type ActiveModel = ActiveModel;
    type Column = Column;
    type Relation = Relation;
    type PrimaryKey = PrimaryKey;
    type EntityCreate = UnitNameCreate;
    type EntityUpdate = UnitNameUpdate;
    type EntityCondition = UnitNameCondition;
    type EntityOrderBy = UnitNameOrderBy;

    fn primary_key_value(model: &Model) -> i64 {
        model.id
    }

    fn primary_key_colum() -> Column {
        Column::Id
    }

    fn entity_action_created_channel() -> &'static str {
        ENTITY_ACTION_CREATED_UNIT_NAME
    }

    fn entity_action_updated_channel() -> &'static str {
        ENTITY_ACTION_UPDATED_UNIT_NAME
    }

    fn entity_action_deleted_channel() -> &'static str {
        ENTITY_ACTION_DELETED_UNIT_NAME
    }
}
