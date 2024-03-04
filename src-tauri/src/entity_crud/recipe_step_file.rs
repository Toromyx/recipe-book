//! This module implements [`EntityCrudTrait`] for [`crate::entity::recipe_step_file`].

use std::{fs, str::FromStr};

use anyhow::Result;
use async_trait::async_trait;
use mime_guess::mime;
use reqwest::header;
use sea_orm::{
    sea_query::IntoCondition,
    ActiveModelTrait,
    ActiveValue::{NotSet, Set, Unchanged},
    ColumnTrait, Condition, DatabaseTransaction, IntoActiveModel, QueryOrder, Select,
};
use serde::Deserialize;
use tempfile::NamedTempFile;
use url::Url;

use crate::{
    entity::recipe_step_file::{ActiveModel, Column, Entity, Model, PrimaryKey, Relation},
    entity_crud::{EntityCrudTrait, Filter, Order, OrderBy, TryIntoActiveModel},
    event::channel::{
        ENTITY_ACTION_CREATED_RECIPE_STEP_FILE, ENTITY_ACTION_DELETED_RECIPE_STEP_FILE,
        ENTITY_ACTION_UPDATED_RECIPE_STEP_FILE,
    },
    recipe_step_file_storage,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecipeStepFileCreateUri {
    Path(String),
    Url(String),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeStepFileCreate {
    pub name: String,
    pub order: i64,
    pub uri: RecipeStepFileCreateUri,
    pub recipe_step_id: i64,
}

#[async_trait]
impl TryIntoActiveModel<ActiveModel> for RecipeStepFileCreate {
    /// Transform [`RecipeStepFileCreate`] into an [`ActiveModel`] by guessing the mime of and maybe downloading the file.
    async fn try_into_active_model(self) -> Result<ActiveModel> {
        let (mime, path) = match self.uri {
            RecipeStepFileCreateUri::Path(path) => {
                let mime = mime_guess::from_path(&path)
                    .first_or(mime::APPLICATION_OCTET_STREAM)
                    .to_string();
                (Set(mime), Set(path))
            }
            RecipeStepFileCreateUri::Url(url) => {
                let url = Url::from_str(&url)?;
                let response = reqwest::get(url).await?;
                let mime = response
                    .headers()
                    .get(header::CONTENT_TYPE)
                    .and_then(|content_type| content_type.to_str().ok())
                    .map(String::from)
                    .unwrap_or(mime::APPLICATION_OCTET_STREAM.to_string());
                let bytes = response.bytes().await?;
                let named_temp_file = NamedTempFile::new()?;
                let path = named_temp_file.path().to_string_lossy().to_string();
                named_temp_file.close()?;
                fs::write(&path, &bytes)?;
                (Set(mime), Set(path))
            }
        };
        Ok(ActiveModel {
            id: NotSet,
            name: Set(self.name),
            order: Set(self.order),
            mime,
            path,
            recipe_step_id: Set(self.recipe_step_id),
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeStepFileUpdate {
    pub id: i64,
    pub name: Option<String>,
    pub order: Option<i64>,
}

impl IntoActiveModel<ActiveModel> for RecipeStepFileUpdate {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: Unchanged(self.id),
            name: match self.name {
                Some(name) => Set(name),
                _ => NotSet,
            },
            order: match self.order {
                Some(order) => Set(order),
                _ => NotSet,
            },
            mime: NotSet,
            path: NotSet,
            recipe_step_id: NotSet,
        }
    }
}

pub type RecipeStepFileFilter = Filter<RecipeStepFileCondition, RecipeStepFileOrderBy>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeStepFileCondition {
    pub recipe_step_id: Option<i64>,
}

impl IntoCondition for RecipeStepFileCondition {
    fn into_condition(self) -> Condition {
        Condition::all().add_option(
            self.recipe_step_id
                .map(|recipe_step_id| Column::RecipeStepId.eq(recipe_step_id)),
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecipeStepFileOrderBy {
    Order(Order),
}

impl OrderBy for RecipeStepFileOrderBy {
    type Entity = Entity;

    fn add(self, select: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            RecipeStepFileOrderBy::Order(order) => select.order_by(Column::Order, order.into()),
        }
    }
}

pub struct RecipeStepFileCrud {}

#[async_trait]
impl EntityCrudTrait for RecipeStepFileCrud {
    type Entity = Entity;
    type Model = Model;
    type ActiveModel = ActiveModel;
    type Column = Column;
    type Relation = Relation;
    type PrimaryKey = PrimaryKey;
    type EntityCreate = RecipeStepFileCreate;
    type EntityUpdate = RecipeStepFileUpdate;
    type EntityCondition = RecipeStepFileCondition;
    type EntityOrderBy = RecipeStepFileOrderBy;

    async fn post_create(model: Model, txn: &DatabaseTransaction) -> Result<Model> {
        let path = recipe_step_file_storage::create(&model).await?;
        let mut active_model = model.into_active_model();
        active_model.path = Set(path.to_string_lossy().to_string());
        let model = active_model.update(txn).await?;
        Ok(model)
    }

    fn primary_key_value(model: &Model) -> i64 {
        model.id
    }

    fn primary_key_colum() -> Column {
        Column::Id
    }

    fn entity_action_created_channel() -> &'static str {
        ENTITY_ACTION_CREATED_RECIPE_STEP_FILE
    }

    fn entity_action_updated_channel() -> &'static str {
        ENTITY_ACTION_UPDATED_RECIPE_STEP_FILE
    }

    fn entity_action_deleted_channel() -> &'static str {
        ENTITY_ACTION_DELETED_RECIPE_STEP_FILE
    }
}
