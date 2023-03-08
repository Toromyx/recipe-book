use async_trait::async_trait;
use sea_orm::{
    sea_query, sea_query::IntoCondition, ActiveModelBehavior, ActiveModelTrait, ColumnTrait,
    EntityTrait, FromQueryResult, IntoActiveModel, ModelTrait, PrimaryKeyToColumn, PrimaryKeyTrait,
    QueryFilter, QueryOrder, QuerySelect, RelationTrait,
};
use serde::Deserialize;

use crate::{api::entity::error::EntityApiError, database, window::get_window};

pub mod error;
pub mod ingredient;
pub mod recipe;
pub mod recipe_file;
pub mod recipe_ingredient;
pub mod recipe_step;

#[derive(FromQueryResult)]
pub struct IdColumn {
    id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Order {
    Asc,
    Desc,
}

impl Default for Order {
    fn default() -> Self {
        Self::Asc
    }
}

impl From<Order> for sea_query::Order {
    fn from(value: Order) -> Self {
        match value {
            Order::Asc => sea_query::Order::Asc,
            Order::Desc => sea_query::Order::Desc,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderByItem<OrderByColumn> {
    column: OrderByColumn,
    #[serde(default)]
    order: Order,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter<Condition, OrderBy> {
    pub condition: Option<Condition>,
    pub order_by: Option<Vec<OrderByItem<OrderBy>>>,
}

fn get_order_by<OrderBy, Column: From<OrderBy>>(
    order_by: Option<Vec<OrderByItem<OrderBy>>>,
) -> Vec<(Column, sea_query::Order)> {
    let Some(order_by) = order_by else {
        return vec![];
    };
    order_by
        .into_iter()
        .map(|item| {
            (
                Column::from(item.column),
                sea_query::Order::from(item.order),
            )
        })
        .collect()
}

#[async_trait]
pub trait EntityCrudTrait {
    type Entity: EntityTrait<
        Model = Self::Model,
        Column = Self::Column,
        Relation = Self::Relation,
        PrimaryKey = Self::PrimaryKey,
    >;

    type Model: ModelTrait<Entity = Self::Entity>
        + FromQueryResult
        + IntoActiveModel<Self::ActiveModel>;

    type ActiveModel: ActiveModelTrait<Entity = Self::Entity> + ActiveModelBehavior + Send;

    type Column: ColumnTrait + From<Self::EntityOrderBy>;

    type Relation: RelationTrait;

    type PrimaryKey: PrimaryKeyTrait<ValueType = i64> + PrimaryKeyToColumn<Column = Self::Column>;

    type EntityCreate: IntoActiveModel<Self::ActiveModel> + Send;

    type EntityUpdate: IntoActiveModel<Self::ActiveModel> + Send;

    type EntityCondition: IntoCondition + Send;

    type EntityOrderBy: Send;

    async fn pre_create(create: Self::EntityCreate) -> Result<Self::ActiveModel, EntityApiError> {
        Ok(create.into_active_model())
    }

    async fn create(
        create: Self::EntityCreate,
    ) -> Result<<Self::PrimaryKey as PrimaryKeyTrait>::ValueType, EntityApiError> {
        let db = database::connect().await;
        let active_model = Self::pre_create(create).await?;
        let model = active_model.insert(db).await?;
        let model = Self::post_create(model).await?;
        get_window().emit(Self::entity_action_created_channel(), ())?;
        Ok(Self::primary_key_value(&model))
    }

    async fn post_create(model: Self::Model) -> Result<Self::Model, EntityApiError> {
        Ok(model)
    }

    async fn read(
        id: <Self::PrimaryKey as PrimaryKeyTrait>::ValueType,
    ) -> Result<Option<Self::Model>, EntityApiError> {
        let db = database::connect().await;
        let model = Self::Entity::find_by_id(id).one(db).await?;
        Ok(model)
    }

    async fn update(update: Self::EntityUpdate) -> Result<Self::Model, EntityApiError> {
        let db = database::connect().await;
        let model = update.into_active_model().update(db).await?;
        get_window().emit(
            Self::entity_action_updated_channel(),
            Self::primary_key_value(&model),
        )?;
        Ok(model)
    }

    async fn pre_delete(model: Self::Model) -> Result<Self::Model, EntityApiError> {
        Ok(model)
    }

    async fn delete(
        id: <Self::PrimaryKey as PrimaryKeyTrait>::ValueType,
    ) -> Result<(), EntityApiError> {
        let db = database::connect().await;
        let model_option = Self::Entity::find_by_id(id).one(db).await?;
        let Some(model) = model_option else {
            return Ok(());
        };
        let model = Self::pre_delete(model).await?;
        model.delete(db).await?;
        get_window().emit(Self::entity_action_deleted_channel(), id)?;
        Ok(())
    }

    async fn list(
        filter: Filter<Self::EntityCondition, Self::EntityOrderBy>,
    ) -> Result<Vec<<Self::PrimaryKey as PrimaryKeyTrait>::ValueType>, EntityApiError> {
        let db = database::connect().await;
        let mut select = Self::Entity::find()
            .select_only()
            .column_as(Self::primary_key_colum(), "id");
        if let Some(condition) = filter.condition {
            select = select.filter(condition);
        }
        for order_by_item in get_order_by::<Self::EntityOrderBy, Self::Column>(filter.order_by) {
            select = select.order_by(order_by_item.0, order_by_item.1);
        }
        let models = select.into_model::<IdColumn>().all(db).await?;
        Ok(models.iter().map(|id_column| id_column.id).collect())
    }

    async fn count(
        filter: Filter<Self::EntityCondition, Self::EntityOrderBy>,
    ) -> Result<i64, EntityApiError> {
        let db = database::connect().await;
        let mut select = Self::Entity::find()
            .select_only()
            .column_as(Self::primary_key_colum().count(), "id");
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

    fn primary_key_value(model: &Self::Model) -> <Self::PrimaryKey as PrimaryKeyTrait>::ValueType;

    fn primary_key_colum() -> Self::Column;

    fn entity_action_created_channel() -> &'static str;

    fn entity_action_updated_channel() -> &'static str;

    fn entity_action_deleted_channel() -> &'static str;
}
