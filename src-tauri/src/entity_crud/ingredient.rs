//! This module implements [`EntityCrudTrait`] for [`crate::entity::ingredient`].

use std::{collections::HashSet, sync::OnceLock};

use milli::Index;
use sea_orm::{
    sea_query::IntoCondition, ActiveValue, ColumnTrait, Condition, DeriveIntoActiveModel,
    EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, QuerySelect, QueryTrait, Select,
};
use sea_query::IntoIden;
use serde::{Deserialize, Serialize};

use crate::{
    entity::{
        ingredient::{ActiveModel, Column, Entity, Model, PrimaryKey, Relation},
        recipe_step_ingredient,
    },
    entity_crud::{EntityCrudTrait, EntityDocumentTrait, Filter, Order, OrderBy},
    event::channel::{
        ENTITY_ACTION_CREATED_INGREDIENT, ENTITY_ACTION_DELETED_INGREDIENT,
        ENTITY_ACTION_UPDATED_INGREDIENT,
    },
};

static SEARCH_INDEX_ONCE: OnceLock<Index> = OnceLock::new();

#[derive(Debug, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct IngredientCreate {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngredientUpdate {
    pub id: i64,
    pub name: Option<String>,
}

impl IntoActiveModel<ActiveModel> for IngredientUpdate {
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
pub struct IngredientDocument {
    pub id: i64,
    pub name: String,
}

impl EntityDocumentTrait for IngredientDocument {
    type Model = Model;

    fn from_model(model: Self::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
        }
    }
}

pub type IngredientFilter = Filter<IngredientCondition, IngredientOrderBy>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngredientCondition {
    pub name: Option<String>,
    pub name_exact: Option<String>,
    pub recipe_step_id: Option<i64>,
}

impl IntoCondition for IngredientCondition {
    fn into_condition(self) -> Condition {
        Condition::all()
            .add_option(self.name.map(|name| Column::Name.like(format!("%{name}%"))))
            .add_option(self.name_exact.map(|name| Column::Name.eq(name)))
            .add_option(self.recipe_step_id.map(|recipe_step_id| {
                Column::Id.in_subquery(
                    recipe_step_ingredient::Entity::find()
                        .select_only()
                        .column(recipe_step_ingredient::Column::IngredientId)
                        .filter(recipe_step_ingredient::Column::RecipeStepId.eq(recipe_step_id))
                        .into_query(),
                )
            }))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IngredientOrderBy {
    Name(Order),
}

impl OrderBy for IngredientOrderBy {
    type Entity = Entity;

    fn add(self, select: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            IngredientOrderBy::Name(order) => select.order_by(Column::Name, order.into()),
        }
    }
}

// TODO special indexing of recipe step IDs

pub struct IngredientCrud {}

impl EntityCrudTrait for IngredientCrud {
    type Entity = Entity;
    type Model = Model;
    type ActiveModel = ActiveModel;
    type Column = Column;
    type Relation = Relation;
    type PrimaryKey = PrimaryKey;
    type PrimaryKeyValue = i64;
    type EntityCreate = IngredientCreate;
    type EntityUpdate = IngredientUpdate;
    type EntityDocument = IngredientDocument;
    type EntityCondition = IngredientCondition;
    type EntityOrderBy = IngredientOrderBy;

    fn primary_key_value(model: &Model) -> i64 {
        model.id
    }

    fn primary_key_colum() -> Column {
        Column::Id
    }

    fn entity_action_created_channel() -> &'static str {
        ENTITY_ACTION_CREATED_INGREDIENT
    }

    fn entity_action_updated_channel() -> &'static str {
        ENTITY_ACTION_UPDATED_INGREDIENT
    }

    fn entity_action_deleted_channel() -> &'static str {
        ENTITY_ACTION_DELETED_INGREDIENT
    }

    fn searchable_fields() -> Vec<String> {
        vec![Column::Name.into_iden().to_string()]
    }

    fn filterable_fields() -> HashSet<String> {
        HashSet::from(["recipe_step_id".to_string()])
    }

    fn sortable_fields() -> HashSet<String> {
        HashSet::from([Column::Name.into_iden().to_string()])
    }

    fn search_index_once() -> &'static OnceLock<Index> {
        &SEARCH_INDEX_ONCE
    }
}
