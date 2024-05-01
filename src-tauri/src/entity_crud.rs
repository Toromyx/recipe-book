//! This module implements create, read, update, delete, list, and count operations for the entities in [`crate::entity`].  

use std::{collections::HashSet, fmt::Debug, sync::OnceLock};

use anyhow::Result;
use async_trait::async_trait;
use milli::Index;
use sea_orm::{
    sea_query, sea_query::IntoCondition, ActiveModelBehavior, ActiveModelTrait, ColumnTrait,
    EntityName, EntityTrait, FromQueryResult, IntoActiveModel, ModelTrait, PrimaryKeyToColumn,
    PrimaryKeyTrait, QueryFilter, QuerySelect, RelationTrait, Select, TransactionTrait, TryFromU64,
    TryGetable, TryGetableMany,
};
use sea_query::{FromValueTuple, Iden, IntoValueTuple};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    database,
    search_index::{search_index_add, search_index_delete, search_index_init},
    window::get_window,
};

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

pub trait EntityDocumentTrait: Serialize + Sized {
    type Model;

    fn from_model(model: Self::Model) -> Self;

    fn into_object(self) -> milli::Object {
        let value = serde_json::to_value(self)
            .expect("Serializing a struct to a serde_json::Value should not fail.");
        match value {
            Value::Object(map) => map,
            _ => unreachable!("A struct converted to a serde_json::Value should always be a map."),
        }
    }
}

/// This struct represents just getting the id column of a database table.
///
/// This is needed for listing entity ids.
#[derive(FromQueryResult)]
pub struct IdColumn<T>
where
    T: TryGetable,
{
    id: T,
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
        > + Default;

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

    /// the entity's primary key
    type PrimaryKey: PrimaryKeyTrait<ValueType = Self::PrimaryKeyValue>
        + PrimaryKeyToColumn<Column = Self::Column>;

    /// the entity's primary key value
    type PrimaryKeyValue: Sized
        + Send
        + Debug
        + PartialEq
        + IntoValueTuple
        + FromValueTuple
        + TryGetableMany
        + TryFromU64
        + TryGetable
        + Serialize
        + Clone
        + ToString;

    /// the struct with which to create an entity, implementing [`TryIntoActiveModel<Self::ActiveModel>`]
    type EntityCreate: TryIntoActiveModel<Self::ActiveModel> + Send;

    /// the struct with which to update an entity, implementing [`TryIntoActiveModel<Self::ActiveModel>`]
    type EntityUpdate: TryIntoActiveModel<Self::ActiveModel> + Send;

    /// the struct with which the search index is built, implementing [`EntityDocumentTrait`]
    type EntityDocument: EntityDocumentTrait<Model = Self::Model> + Send;

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
    /// - when there is any problem with the search index
    async fn create(
        create: Self::EntityCreate,
    ) -> Result<<Self::PrimaryKey as PrimaryKeyTrait>::ValueType> {
        let db = database::connect_writing().await;
        let txn = db.begin().await?;
        let active_model = create.try_into_active_model().await?;
        let model = active_model.insert(&txn).await?;
        txn.commit().await?;
        let search_index = Self::search_index();
        let document = Self::EntityDocument::from_model(model.clone());
        search_index_add(search_index, &document.into_object())?;
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
    /// - when there is any problem with the search index
    async fn update(update: Self::EntityUpdate) -> Result<Self::Model> {
        let db = database::connect_writing().await;
        let txn = db.begin().await?;
        let model = update.try_into_active_model().await?.update(&txn).await?;
        txn.commit().await?;
        let search_index = Self::search_index();
        let document = Self::EntityDocument::from_model(model.clone());
        search_index_add(search_index, &document.into_object())?;
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
    /// - when there is any problem with the search index
    async fn delete(id: <Self::PrimaryKey as PrimaryKeyTrait>::ValueType) -> Result<()> {
        let db = database::connect_writing().await;
        let txn = db.begin().await?;
        let model_option = Self::Entity::find_by_id(id.clone()).one(&txn).await?;
        let Some(model) = model_option else {
            return Ok(());
        };
        let search_index_primary_key_value = Self::search_index_primary_key_value(&model);
        model.delete(&txn).await?;
        txn.commit().await?;
        let search_index = Self::search_index();
        search_index_delete(search_index, search_index_primary_key_value)?;
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
    ) -> Result<Vec<Self::PrimaryKeyValue>> {
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
        let models = select
            .into_model::<IdColumn<Self::PrimaryKeyValue>>()
            .all(db)
            .await?;
        Ok(models.into_iter().map(|id_column| id_column.id).collect())
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
        let count_option = select.into_model::<IdColumn<i64>>().one(db).await?;
        let count = match count_option {
            Some(id_column) => id_column.id,
            _ => 0,
        };
        Ok(count)
    }

    /// Search the search index for entities.
    ///
    /// # Errors
    ///
    /// - when there is any problem with the search index
    async fn search() -> Result<Vec<Self::PrimaryKeyValue>> {
        // TODO search search_index, build parameters according to what milli can do
        // https://www.meilisearch.com/docs/reference/api/search#customize-attributes-to-search-on-at-search-time
        // https://www.meilisearch.com/docs/learn/fine_tuning_results/filtering
        todo!()
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

    /// Get the primary key name for the search index.
    ///
    /// This is used in [`milli::update::Settings::set_primary_key`].
    fn search_index_primary_key_field() -> String {
        Self::primary_key_colum().to_string()
    }

    /// Get the primary key value for the search index.
    ///
    /// This is used for [`milli::update::index_documents::IndexDocuments::remove_documents`].
    fn search_index_primary_key_value(model: &Self::Model) -> String {
        Self::primary_key_value(model).to_string()
    }

    /// Get the searchable fields for the search index.
    ///
    /// This is used in [`milli::update::Settings::set_searchable_fields`].
    /// The order is the [attribute ranking order](https://www.meilisearch.com/docs/learn/core_concepts/relevancy#attribute-ranking-order).
    /// TODO what fields should be searchable?
    fn searchable_fields() -> Vec<String>;

    /// Get the filterable fields for the search index.
    ///
    /// This is used in [`milli::update::Settings::set_filterable_fields`].
    /// TODO what fields should be searchable?
    fn filterable_fields() -> HashSet<String>;

    /// Get the sortable fields for the search index.
    ///
    /// This is used in [`milli::update::Settings::set_sortable_fields`].
    /// TODO what fields should be sortable?
    fn sortable_fields() -> HashSet<String>;

    fn search_index_once() -> &'static OnceLock<Index>;

    fn search_index() -> &'static Index {
        Self::search_index_once().get_or_init(|| {
            search_index_init(
                Self::Entity::default().table_name(),
                Self::search_index_primary_key_field(),
                Self::searchable_fields(),
                Self::filterable_fields(),
                Self::sortable_fields(),
            )
        })
    }
}
