use sea_orm::{
    sea_query::IntoCondition,
    ActiveValue::{NotSet, Set, Unchanged},
    ColumnTrait, Condition, DeriveIntoActiveModel, IntoActiveModel,
};
use serde::Deserialize;

use crate::{
    api::entity::{EntityCrudTrait, Filter},
    entity::recipe::{ActiveModel, Column, Entity, Model, PrimaryKey, Relation},
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
        Condition::all().add_option(
            self.name
                .map(|name| Column::Name.like(&format!("%{name}%"))),
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecipeOrderBy {
    Name,
}

impl From<RecipeOrderBy> for Column {
    fn from(value: RecipeOrderBy) -> Self {
        match value {
            RecipeOrderBy::Name => Column::Name,
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

    fn primary_key_value(model: Self::Model) -> i64 {
        model.id
    }

    fn primary_key_colum() -> Self::Column {
        Column::Id
    }
}
