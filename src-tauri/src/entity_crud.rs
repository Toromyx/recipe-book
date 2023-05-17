//! This module implements create, read, update, delete, list, and count operations for the entities in [`crate::entity`].  

use async_trait::async_trait;
use sea_orm::{
    sea_query, sea_query::IntoCondition, ActiveModelBehavior, ActiveModelTrait, ColumnTrait,
    DatabaseTransaction, EntityTrait, FromQueryResult, IntoActiveModel, ModelTrait,
    PrimaryKeyToColumn, PrimaryKeyTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait,
    TransactionTrait,
};
use serde::Deserialize;

use crate::{database, entity_crud::error::EntityCrudError, window::get_window};

pub mod error;
pub mod ingredient;
pub mod recipe;
pub mod recipe_file;
pub mod recipe_ingredient;
pub mod recipe_ingredient_draft;
pub mod recipe_step;

/// This struct represents just getting the id column of a database table.
///
/// This is needed for listing entity ids.
#[derive(FromQueryResult)]
pub struct IdColumn {
    id: i64,
}

/// This enum is used to specify how to order results when listing an entity.
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

/// This struct represents a single ordering instruction when listing entities.
///
/// The type parameter [`OrderByColumn`] represents the column to be sorted.
/// This struct is used in [`Filter`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderByItem<OrderByColumn> {
    column: OrderByColumn,
    #[serde(default)]
    order: Order,
}

/// This struct combines conditional filtering and ordering when listing entities.
///
/// This struct is used in [`EntityCrudTrait`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter<Condition, OrderBy> {
    pub condition: Option<Condition>,
    pub order_by: Option<Vec<OrderByItem<OrderBy>>>,
}

/// Get a [`sea_query::Order`] for each [`Column`].
///
/// This function is used when listing entities in [`EntityCrudTrait`].
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

/// This trait implements create, read, update, delete, list, and count operation for an entity.
///
/// When implementing this trait, only the associated types and some simple helper function need to be implemented.
///
/// This trait currently uses [`async_trait`] but also works with `#![feature(async_fn_in_trait)]`.
/// It should drop the usage of [`async_trait`] once `async_fn_in_trait` is stabilized.
/// For more information see <https://github.com/rust-lang/rust/issues/91611>.
#[async_trait]
pub trait EntityCrudTrait {
    /// the entity, implementing [`EntityTrait`]
    type Entity: EntityTrait<
        Model = Self::Model,
        Column = Self::Column,
        Relation = Self::Relation,
        PrimaryKey = Self::PrimaryKey,
    >;

    /// the entity's model, implementing [`ModelTrait`]
    type Model: ModelTrait<Entity = Self::Entity>
        + FromQueryResult
        + IntoActiveModel<Self::ActiveModel>;

    /// the entity's active model, implementing [`ActiveModelTrait`]
    type ActiveModel: ActiveModelTrait<Entity = Self::Entity> + ActiveModelBehavior + Send;

    /// the entity's column enum, implementing [`ColumnTrait`] and [`From<Self::EntityOrderBy>`]
    type Column: ColumnTrait + From<Self::EntityOrderBy>;

    /// the entity's relation enum, implementing [`RelationTrait`]
    type Relation: RelationTrait;

    /// the entity's primary key, its value type needs to be `i64`
    type PrimaryKey: PrimaryKeyTrait<ValueType = i64> + PrimaryKeyToColumn<Column = Self::Column>;

    /// the struct with which to create an entity, implementing [`IntoActiveModel<Self::ActiveModel>`]
    type EntityCreate: IntoActiveModel<Self::ActiveModel> + Send;

    /// the struct with which to update an entity, implementing [`IntoActiveModel<Self::ActiveModel>`]
    type EntityUpdate: IntoActiveModel<Self::ActiveModel> + Send;

    /// the struct with which to filter an entity list or count, implementing [`IntoCondition`]
    type EntityCondition: IntoCondition + Send;

    /// used when ordering an entity list
    type EntityOrderBy: Send;

    /// Implement this to run code before creating the entity.
    ///
    /// see [`Self::create`]
    async fn pre_create(
        create: Self::EntityCreate,
        _txn: &DatabaseTransaction,
    ) -> Result<Self::ActiveModel, EntityCrudError> {
        Ok(create.into_active_model())
    }

    /// Create an entity.
    ///
    /// # Errors
    ///
    /// - [`EntityCrudError::Db`] when there is any problem with the database
    /// - [`EntityCrudError::Tauri`] when the tauri window can't be messaged about the created entity
    /// - when there is an error in [`Self::pre_create`] or [`Self::post_create`]
    async fn create(
        create: Self::EntityCreate,
    ) -> Result<<Self::PrimaryKey as PrimaryKeyTrait>::ValueType, EntityCrudError> {
        let db = database::connect().await;
        let txn = db.begin().await?;
        let active_model = Self::pre_create(create, &txn).await?;
        let model = active_model.insert(&txn).await?;
        let model = Self::post_create(model, &txn).await?;
        txn.commit().await?;
        get_window().emit(Self::entity_action_created_channel(), ())?;
        Ok(Self::primary_key_value(&model))
    }

    /// Implement this to run code after creating the entity.
    ///
    /// see [`Self::create`]
    async fn post_create(
        model: Self::Model,
        _txn: &DatabaseTransaction,
    ) -> Result<Self::Model, EntityCrudError> {
        Ok(model)
    }

    /// Read an entity.
    ///
    /// # Errors
    ///
    /// - [`EntityCrudError::Db`] when there is any problem with the database
    async fn read(
        id: <Self::PrimaryKey as PrimaryKeyTrait>::ValueType,
    ) -> Result<Option<Self::Model>, EntityCrudError> {
        let db = database::connect().await;
        let model = Self::Entity::find_by_id(id).one(db).await?;
        Ok(model)
    }

    /// Update an entity.
    ///
    /// # Errors
    ///
    /// - [`EntityCrudError::Db`] when there is any problem with the database
    /// - [`EntityCrudError::Tauri`] when the tauri window can't be messaged about the updated entity
    async fn update(update: Self::EntityUpdate) -> Result<Self::Model, EntityCrudError> {
        let db = database::connect().await;
        let txn = db.begin().await?;
        let model = update.into_active_model().update(&txn).await?;
        txn.commit().await?;
        get_window().emit(
            Self::entity_action_updated_channel(),
            Self::primary_key_value(&model),
        )?;
        Ok(model)
    }

    /// Implement this to run code before deleting the entity.
    ///
    /// see [`Self::delete`]
    async fn pre_delete(
        model: Self::Model,
        _txn: &DatabaseTransaction,
    ) -> Result<Self::Model, EntityCrudError> {
        Ok(model)
    }

    /// Delete an entity.
    ///
    /// # Errors
    ///
    /// - [`EntityCrudError::Db`] when there is any problem with the database
    /// - [`EntityCrudError::Tauri`] when the tauri window can't be messaged about the deleted entity
    /// - when there is an error in [`Self::pre_delete`]
    async fn delete(
        id: <Self::PrimaryKey as PrimaryKeyTrait>::ValueType,
    ) -> Result<(), EntityCrudError> {
        let db = database::connect().await;
        let txn = db.begin().await?;
        let model_option = Self::Entity::find_by_id(id).one(&txn).await?;
        let Some(model) = model_option else {
            return Ok(());
        };
        let model = Self::pre_delete(model, &txn).await?;
        model.delete(&txn).await?;
        txn.commit().await?;
        get_window().emit(Self::entity_action_deleted_channel(), id)?;
        Ok(())
    }

    /// List entities.
    ///
    /// # Errors
    ///
    /// - [`EntityCrudError::Db`] when there is any problem with the database
    async fn list(
        filter: Filter<Self::EntityCondition, Self::EntityOrderBy>,
    ) -> Result<Vec<<Self::PrimaryKey as PrimaryKeyTrait>::ValueType>, EntityCrudError> {
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

    /// Count entities.
    ///
    /// # Errors
    ///
    /// - [`EntityCrudError::Db`] when there is any problem with the database
    async fn count(
        filter: Filter<Self::EntityCondition, Self::EntityOrderBy>,
    ) -> Result<i64, EntityCrudError> {
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

    /// Get the primary key value from the entity model.
    fn primary_key_value(model: &Self::Model) -> <Self::PrimaryKey as PrimaryKeyTrait>::ValueType;

    /// Get the primary key column.
    fn primary_key_colum() -> Self::Column;

    /// Get the tauri event channel for a created entity.
    fn entity_action_created_channel() -> &'static str;

    /// Get the tauri event channel for an update entity.
    fn entity_action_updated_channel() -> &'static str;

    /// Get the tauri event channel for a deleted entity.
    fn entity_action_deleted_channel() -> &'static str;
}
