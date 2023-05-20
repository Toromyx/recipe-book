//! This module implements the unit name entity.
//!
//! See [`Model`] for more information.

pub mod unit;

use sea_orm::entity::prelude::*;
use serde::Serialize;

use crate::entity::unit_name::unit::Unit;

/// This struct represents a known unit name.
///
/// Each unit name is related to a value of the enumeration [`Unit`]. This is meant to help with unit conversions.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "unit_name")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub unit: Unit,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
