//! This module implements create, read, update, delete, list, and count operations for the entities in [`crate::entity`].  

use anyhow::Result;
use async_trait::async_trait;
use sea_orm::{
    sea_query, sea_query::IntoCondition, ActiveModelBehavior, ActiveModelTrait, ColumnTrait,
    EntityTrait, FromQueryResult, IntoActiveModel, ModelTrait, PrimaryKeyToColumn, PrimaryKeyTrait,
    QueryFilter, QuerySelect, RelationTrait, Select, TransactionTrait,
};
use serde::Deserialize;

use crate::{database, window::get_window};

pub mod file;
pub mod ingredient;
pub mod recipe;
pub mod recipe_file;
pub mod recipe_ingredient_draft;
pub mod recipe_step;
pub mod recipe_step_file;
pub mod recipe_step_ingredient;
pub mod recipe_step_ingredient_draft;
pub mod unit_name;

/// A trait to transform from any type into an [active model](ActiveModelTrait).
///
/// This is a async and fallible version of [`IntoActiveModel`].
#[async_trait]
pub trait TryIntoActiveModel<A>
where
    A: ActiveModelTrait,
{
    async fn try_into_active_model(self) -> Result<A>;
}

#[async_trait]
impl<A, T> TryIntoActiveModel<A> for T
where
    A: ActiveModelTrait,
    T: IntoActiveModel<A> + Send,
{
    async fn try_into_active_model(self) -> Result<A> {
        Ok(IntoActiveModel::into_active_model(self))
    }
}

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

/// This struct combines conditional filtering and ordering when listing entities.
///
/// This struct is used in [`EntityCrudTrait`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter<Condition, OrderBy> {
    pub condition: Option<Condition>,
    pub order_by: Option<Vec<OrderBy>>,
}

/// Implementors of this trait should add their order-by statements in the [`Self::add`] function.
pub trait OrderBy {
    type Entity: EntityTrait;

    /// Add an ordering to the select statement.
    fn add(self, select: Select<Self::Entity>) -> Select<Self::Entity>;
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

    /// the entity's column enum, implementing [`ColumnTrait`]
    type Column: ColumnTrait;

    /// the entity's relation enum, implementing [`RelationTrait`]
    type Relation: RelationTrait;

    /// the entity's primary key, its value type needs to be `i64`
    type PrimaryKey: PrimaryKeyTrait<ValueType = i64> + PrimaryKeyToColumn<Column = Self::Column>;

    /// the struct with which to create an entity, implementing [`TryIntoActiveModel<Self::ActiveModel>`]
    type EntityCreate: TryIntoActiveModel<Self::ActiveModel> + Send;

    /// the struct with which to update an entity, implementing [`TryIntoActiveModel<Self::ActiveModel>`]
    type EntityUpdate: TryIntoActiveModel<Self::ActiveModel> + Send;

    /// the struct with which to filter an entity list or count, implementing [`IntoCondition`]
    type EntityCondition: IntoCondition + Send;

    /// the struct with which to order an entity list, implementing [`OrderBy`]
    type EntityOrderBy: OrderBy<Entity = Self::Entity> + Send;

    /// Create an entity.
    ///
    /// # Errors
    ///
    /// - when there is any problem with the database
    /// - when the tauri window can't be messaged about the created entity
    async fn create(
        create: Self::EntityCreate,
    ) -> Result<<Self::PrimaryKey as PrimaryKeyTrait>::ValueType> {
        let db = database::connect_writing().await;
        let txn = db.begin().await?;
        let active_model = create.try_into_active_model().await?;
        let model = active_model.insert(&txn).await?;
        txn.commit().await?;
        get_window().emit(Self::entity_action_created_channel(), ())?;
        Ok(Self::primary_key_value(&model))
    }

    /// Read an entity.
    ///
    /// # Errors
    ///
    /// - when there is any problem with the database
    async fn read(
        id: <Self::PrimaryKey as PrimaryKeyTrait>::ValueType,
    ) -> Result<Option<Self::Model>> {
        let db = database::connect().await;
        let model = Self::Entity::find_by_id(id).one(db).await?;
        Ok(model)
    }

    /// Update an entity.
    ///
    /// # Errors
    ///
    /// - when there is any problem with the database
    /// - when the tauri window can't be messaged about the updated entity
    async fn update(update: Self::EntityUpdate) -> Result<Self::Model> {
        let db = database::connect_writing().await;
        let txn = db.begin().await?;
        let model = update.try_into_active_model().await?.update(&txn).await?;
        txn.commit().await?;
        get_window().emit(
            Self::entity_action_updated_channel(),
            Self::primary_key_value(&model),
        )?;
        Ok(model)
    }

    /// Delete an entity.
    ///
    /// # Errors
    ///
    /// - when there is any problem with the database
    /// - when the tauri window can't be messaged about the deleted entity
    /// - when there is an error in [`Self::pre_delete`]
    async fn delete(id: <Self::PrimaryKey as PrimaryKeyTrait>::ValueType) -> Result<()> {
        let db = database::connect_writing().await;
        let txn = db.begin().await?;
        let model_option = Self::Entity::find_by_id(id).one(&txn).await?;
        let Some(model) = model_option else {
            return Ok(());
        };
        model.delete(&txn).await?;
        txn.commit().await?;
        get_window().emit(Self::entity_action_deleted_channel(), id)?;
        Ok(())
    }

    /// List entities.
    ///
    /// # Errors
    ///
    /// - when there is any problem with the database
    async fn list(
        filter: Filter<Self::EntityCondition, Self::EntityOrderBy>,
    ) -> Result<Vec<<Self::PrimaryKey as PrimaryKeyTrait>::ValueType>> {
        let db = database::connect().await;
        let mut select = Self::Entity::find()
            .select_only()
            .column_as(Self::primary_key_colum(), "id");
        if let Some(condition) = filter.condition {
            select = select.filter(condition);
        }
        for order_by in filter.order_by.into_iter().flatten() {
            select = order_by.add(select);
        }
        let models = select.into_model::<IdColumn>().all(db).await?;
        Ok(models.iter().map(|id_column| id_column.id).collect())
    }

    /// Count entities.
    ///
    /// # Errors
    ///
    /// - when there is any problem with the database
    async fn count(condition: Option<Self::EntityCondition>) -> Result<i64> {
        let db = database::connect().await;
        let mut select = Self::Entity::find()
            .select_only()
            .column_as(Self::primary_key_colum().count(), "id");
        if let Some(condition) = condition {
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
