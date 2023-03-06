use sea_orm::{
    sea_query::IntoCondition,
    ActiveModelTrait,
    ActiveValue::{NotSet, Set, Unchanged},
    ColumnTrait, Condition, DbErr, DeriveIntoActiveModel, EntityTrait, IntoActiveModel, ModelTrait,
    QueryFilter, QueryOrder, QuerySelect,
};
use serde::Deserialize;

use crate::{
    api::entity::{get_order_by, Filter, IdColumn},
    database,
    entity::recipe::{
        ActiveModel, Column,
        Column::{Id, Name},
        Entity, Model,
    },
};

#[derive(Debug, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct RecipeCreate {
    pub name: String,
}

pub async fn create(create: RecipeCreate) -> Result<i64, DbErr> {
    let db = database::connect().await;
    let model = create.into_active_model().insert(db).await?;
    Ok(model.id)
}

pub async fn read(id: i64) -> Result<Option<Model>, DbErr> {
    let db = database::connect().await;
    let model = Entity::find_by_id(id).one(db).await?;
    Ok(model)
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

pub async fn update(update: RecipeUpdate) -> Result<Model, DbErr> {
    let db = database::connect().await;
    let model = update.into_active_model().update(db).await?;
    Ok(model)
}

pub async fn delete(id: i64) -> Result<(), DbErr> {
    let db = database::connect().await;
    let model_option = Entity::find_by_id(id).one(db).await?;
    let Some(model) = model_option else {
        return Ok(());
    };
    model.delete(db).await?;
    Ok(())
}

pub type RecipeFilter = Filter<RecipeCondition, RecipeOrderBy>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeCondition {
    pub name: Option<String>,
}

impl IntoCondition for RecipeCondition {
    fn into_condition(self) -> Condition {
        Condition::all().add_option(self.name.map(|name| Name.like(&format!("%{name}%"))))
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
            RecipeOrderBy::Name => Name,
        }
    }
}

pub async fn list(filter: RecipeFilter) -> Result<Vec<i64>, DbErr> {
    let db = database::connect().await;
    let mut select = Entity::find().select_only().column(Id);
    if let Some(condition) = filter.condition {
        select = select.filter(condition);
    }
    for order_by_item in get_order_by::<RecipeOrderBy, Column>(filter.order_by) {
        select = select.order_by(order_by_item.0, order_by_item.1);
    }
    let models = select.into_model::<IdColumn>().all(db).await?;
    Ok(models.iter().map(|id_column| id_column.id).collect())
}

pub async fn count(filter: RecipeFilter) -> Result<i64, DbErr> {
    let db = database::connect().await;
    let mut select = Entity::find().select_only().column_as(Id.count(), "id");
    if let Some(condition) = filter.condition {
        select = select.filter(condition);
    }
    let count_option = select.into_model::<IdColumn>().one(db).await?;
    let count = match count_option {
        Some(id_column) => id_column.id,
        _ => 0,
    };
    Ok(count)
}
