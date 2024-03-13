//! This module implements [`EntityCrudTrait`] for [`crate::entity::ingredient`].

use sea_orm::{
    sea_query::IntoCondition, ActiveValue, ColumnTrait, Condition, DeriveIntoActiveModel,
    EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, QuerySelect, QueryTrait, Select,
};
use serde::Deserialize;

use crate::{
    entity::{
        ingredient::{ActiveModel, Column, Entity, Model, PrimaryKey, Relation},
        recipe_step_ingredient,
    },
    entity_crud::{EntityCrudTrait, Filter, Order, OrderBy},
    event::channel::{
        ENTITY_ACTION_CREATED_INGREDIENT, ENTITY_ACTION_DELETED_INGREDIENT,
        ENTITY_ACTION_UPDATED_INGREDIENT,
    },
};

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

pub struct IngredientCrud {}

impl EntityCrudTrait for IngredientCrud {
    type Entity = Entity;
    type Model = Model;
    type ActiveModel = ActiveModel;
    type Column = Column;
    type Relation = Relation;
    type PrimaryKey = PrimaryKey;
    type EntityCreate = IngredientCreate;
    type EntityUpdate = IngredientUpdate;
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
}
