//! This module implements [`EntityCrudTrait`] for [`crate::entity::unit_name`].

use sea_orm::{
    sea_query::IntoCondition, ActiveValue, Condition, IntoActiveModel, IntoActiveValue, Select,
};
use serde::Deserialize;

use crate::{
    entity::unit_name::{unit::Unit, ActiveModel, Column, Entity, Model, PrimaryKey, Relation},
    entity_crud::{EntityCrudTrait, Filter, OrderBy},
    event::channel::{
        ENTITY_ACTION_CREATED_UNIT_NAME, ENTITY_ACTION_DELETED_UNIT_NAME,
        ENTITY_ACTION_UPDATED_UNIT_NAME,
    },
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitNameCreate {
    pub name: String,
    pub unit: Unit,
}

impl IntoActiveModel<ActiveModel> for UnitNameCreate {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            name: self.name.into_active_value(),
            unit: self.unit.into_active_value(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitNameUpdate {
    pub name: String,
    pub unit: Option<Unit>,
}

impl IntoActiveModel<ActiveModel> for UnitNameUpdate {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            name: ActiveValue::Unchanged(self.name),
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
pub struct UnitNameCondition {}

impl IntoCondition for UnitNameCondition {
    fn into_condition(self) -> Condition {
        Condition::all()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UnitNameOrderBy {}

impl OrderBy for UnitNameOrderBy {
    type Entity = Entity;

    fn add(self, select: Select<Self::Entity>) -> Select<Self::Entity> {
        select
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
    type PrimaryKeyValue = String;
    type EntityCreate = UnitNameCreate;
    type EntityUpdate = UnitNameUpdate;
    type EntityCondition = UnitNameCondition;
    type EntityOrderBy = UnitNameOrderBy;

    fn primary_key_value(model: &Model) -> String {
        model.name.clone()
    }

    fn primary_key_colum() -> Column {
        Column::Name
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
