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
    entity::recipe_step::{
        ActiveModel, Column,
        Column::{Id, Order, RecipeId},
        Entity, Model,
    },
};

#[derive(Debug, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct RecipeStepCreate {
    pub order: i64,
    pub description: String,
    pub recipe_id: i64,
}

pub async fn create(create: RecipeStepCreate) -> Result<i64, DbErr> {
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
pub struct RecipeStepUpdate {
    pub id: i64,
    pub order: Option<i64>,
    pub description: Option<String>,
}

impl IntoActiveModel<ActiveModel> for RecipeStepUpdate {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: Unchanged(self.id),
            order: match self.order {
                Some(order) => Set(order),
                _ => NotSet,
            },
            description: match self.description {
                Some(description) => Set(description),
                _ => NotSet,
            },
            image: NotSet,
            recipe_id: NotSet,
        }
    }
}

pub async fn update(update: RecipeStepUpdate) -> Result<Model, DbErr> {
    let db = database::connect().await;
    let model = update.into_active_model().update(db).await?;
    Ok(model)
}

impl IntoActiveModel<ActiveModel> for i64 {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: Unchanged(self),
            order: NotSet,
            description: NotSet,
            image: NotSet,
            recipe_id: NotSet,
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

pub type RecipeStepFilter = Filter<RecipeStepCondition, RecipeStepOrderBy>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeStepCondition {
    pub recipe_id: Option<i64>,
}

impl IntoCondition for RecipeStepCondition {
    fn into_condition(self) -> Condition {
        Condition::all().add_option(self.recipe_id.map(|recipe_id| RecipeId.eq(recipe_id)))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecipeStepOrderBy {
    Order,
}

impl From<RecipeStepOrderBy> for Column {
    fn from(value: RecipeStepOrderBy) -> Self {
        match value {
            RecipeStepOrderBy::Order => Order,
        }
    }
}

pub async fn list(filter: RecipeStepFilter) -> Result<Vec<i64>, DbErr> {
    let db = database::connect().await;
    let mut select = Entity::find().select_only().column(Id);
    if let Some(condition) = filter.condition {
        select = select.filter(condition);
    }
    for order_by_item in get_order_by::<RecipeStepOrderBy, Column>(filter.order_by) {
        select = select.order_by(order_by_item.0, order_by_item.1);
    }
    let models = select.into_model::<IdColumn>().all(db).await?;
    Ok(models.iter().map(|id_column| id_column.id).collect())
}

pub async fn count(filter: RecipeStepFilter) -> Result<i64, DbErr> {
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
