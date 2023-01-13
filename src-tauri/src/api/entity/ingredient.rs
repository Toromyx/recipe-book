use sea_orm::{
    sea_query::IntoCondition,
    ActiveModelTrait,
    ActiveValue::{NotSet, Set, Unchanged},
    ColumnTrait, Condition, DbErr, DeriveIntoActiveModel, EntityTrait, IntoActiveModel,
    QueryFilter, QueryOrder, QuerySelect,
};
use serde::Deserialize;

use crate::{
    api::entity::{get_order_by, Filter, IdColumn},
    database,
    entity::ingredient::{
        ActiveModel, Column,
        Column::{Id, Name},
        Entity, Model,
    },
};

#[derive(Debug, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct IngredientCreate {
    pub name: String,
}

pub async fn create(create: IngredientCreate) -> Result<i64, DbErr> {
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
pub struct IngredientUpdate {
    pub id: i64,
    pub name: Option<String>,
}

impl IntoActiveModel<ActiveModel> for IngredientUpdate {
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

pub async fn update(update: IngredientUpdate) -> Result<Model, DbErr> {
    let db = database::connect().await;
    let model = update.into_active_model().update(db).await?;
    Ok(model)
}

impl IntoActiveModel<ActiveModel> for i64 {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: Unchanged(self),
            name: NotSet,
        }
    }
}

pub async fn delete(id: i64) -> Result<(), DbErr> {
    let db = database::connect().await;
    <i64 as IntoActiveModel<ActiveModel>>::into_active_model(id)
        .delete(db)
        .await?;
    Ok(())
}

pub type IngredientFilter = Filter<IngredientCondition, IngredientOrderBy>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngredientCondition {
    pub name: Option<String>,
}

impl IntoCondition for IngredientCondition {
    fn into_condition(self) -> Condition {
        Condition::all().add_option(self.name.map(|name| Name.like(&format!("%{name}%"))))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IngredientOrderBy {
    Name,
}

impl From<IngredientOrderBy> for Column {
    fn from(value: IngredientOrderBy) -> Self {
        match value {
            IngredientOrderBy::Name => Name,
        }
    }
}

pub async fn list(filter: IngredientFilter) -> Result<Vec<i64>, DbErr> {
    let db = database::connect().await;
    let mut select = Entity::find().select_only().column(Id);
    if let Some(condition) = filter.condition {
        select = select.filter(condition);
    }
    for order_by_item in get_order_by::<IngredientOrderBy, Column>(filter.order_by) {
        select = select.order_by(order_by_item.0, order_by_item.1);
    }
    let models = select.into_model::<IdColumn>().all(db).await?;
    Ok(models.iter().map(|id_column| id_column.id).collect())
}

pub async fn count(filter: IngredientFilter) -> Result<i64, DbErr> {
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
