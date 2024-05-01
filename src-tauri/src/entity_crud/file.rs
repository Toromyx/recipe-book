//! This module implements [`EntityCrudTrait`] for [`crate::entity::file`].

use std::{collections::HashSet, fs, str::FromStr, sync::OnceLock};

use anyhow::Result;
use async_trait::async_trait;
use milli::Index;
use mime_guess::mime;
use reqwest::header;
use sea_orm::{
    sea_query::IntoCondition, ActiveValue, ColumnTrait, Condition, IntoActiveModel, QueryOrder,
    Select,
};
use sea_query::IntoIden;
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use url::Url;

use crate::{
    entity::file::{ActiveModel, Column, Entity, Model, PrimaryKey, Relation},
    entity_crud::{
        EntityCrudTrait, EntityDocumentTrait, Filter, Order, OrderBy, TryIntoActiveModel,
    },
    event::channel::{
        ENTITY_ACTION_CREATED_FILE, ENTITY_ACTION_DELETED_FILE, ENTITY_ACTION_UPDATED_FILE,
    },
    file_storage,
};

static SEARCH_INDEX_ONCE: OnceLock<Index> = OnceLock::new();

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FileCreateUri {
    Path(String),
    Url(String),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileCreate {
    pub name: String,
    pub uri: FileCreateUri,
}

#[async_trait]
impl TryIntoActiveModel<ActiveModel> for FileCreate {
    /// Transform [`FileCreate`] into an [`ActiveModel`] by guessing the mime of and maybe downloading the file.
    ///
    /// This also creates the file in the [`file_storage`].
    async fn try_into_active_model(self) -> Result<ActiveModel> {
        let (mime, path) = match self.uri {
            FileCreateUri::Path(path) => {
                let mime = mime_guess::from_path(&path)
                    .first_or(mime::APPLICATION_OCTET_STREAM)
                    .to_string();
                (mime, path)
            }
            FileCreateUri::Url(url) => {
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
                (mime, path)
            }
        };
        let path = file_storage::create(&path, &mime).await?;
        Ok(ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(self.name),
            mime: ActiveValue::Set(mime),
            path: ActiveValue::Set(path.to_string_lossy().to_string()),
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUpdate {
    pub id: i64,
    pub name: Option<String>,
    pub order: Option<i64>,
}

impl IntoActiveModel<ActiveModel> for FileUpdate {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: ActiveValue::Unchanged(self.id),
            name: match self.name {
                Some(name) => ActiveValue::Set(name),
                _ => ActiveValue::NotSet,
            },
            mime: ActiveValue::NotSet,
            path: ActiveValue::NotSet,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FileDocument {}

impl EntityDocumentTrait for FileDocument {
    type Model = Model;

    fn from_model(_model: Self::Model) -> Self {
        todo!()
    }
}

pub type FileFilter = Filter<FileCondition, FileOrderBy>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileCondition {
    pub name: Option<String>,
}

impl IntoCondition for FileCondition {
    fn into_condition(self) -> Condition {
        Condition::all().add_option(self.name.map(|name| Column::Name.like(format!("%{name}%"))))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FileOrderBy {
    Name(Order),
}

impl OrderBy for FileOrderBy {
    type Entity = Entity;

    fn add(self, select: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            FileOrderBy::Name(order) => select.order_by(Column::Name, order.into()),
        }
    }
}

pub struct FileCrud {}

#[async_trait]
impl EntityCrudTrait for FileCrud {
    type Entity = Entity;
    type Model = Model;
    type ActiveModel = ActiveModel;
    type Column = Column;
    type Relation = Relation;
    type PrimaryKey = PrimaryKey;
    type PrimaryKeyValue = i64;
    type EntityCreate = FileCreate;
    type EntityUpdate = FileUpdate;
    type EntityDocument = FileDocument;
    type EntityCondition = FileCondition;
    type EntityOrderBy = FileOrderBy;

    fn primary_key_value(model: &Model) -> i64 {
        model.id
    }

    fn primary_key_colum() -> Column {
        Column::Id
    }

    fn entity_action_created_channel() -> &'static str {
        ENTITY_ACTION_CREATED_FILE
    }

    fn entity_action_updated_channel() -> &'static str {
        ENTITY_ACTION_UPDATED_FILE
    }

    fn entity_action_deleted_channel() -> &'static str {
        ENTITY_ACTION_DELETED_FILE
    }

    fn searchable_fields() -> Vec<String> {
        vec![Column::Name.into_iden().to_string()]
    }

    fn filterable_fields() -> HashSet<String> {
        HashSet::from([])
    }

    fn sortable_fields() -> HashSet<String> {
        HashSet::from([Column::Name.into_iden().to_string()])
    }

    fn search_index_once() -> &'static OnceLock<Index> {
        &SEARCH_INDEX_ONCE
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{create_temp_file, TEST_NAME};

    #[tokio::test]
    async fn test_try_into_active_model_local() {
        TEST_NAME.set(Some(
            "entity_crud__file__test_try_into_active_model".to_string(),
        ));
        crate::tests::run();

        let expected_name = "name";
        let expected_content = "content";
        let temp_path = create_temp_file(
            "entity_crud__file__test_try_into_active_model.bin",
            expected_content,
        );
        let file_create = FileCreate {
            name: expected_name.to_string(),
            uri: FileCreateUri::Path(temp_path.to_string_lossy().to_string()),
        };
        let active_model = file_create.try_into_active_model().await.unwrap();
        assert!(active_model.mime.is_set());
        let name = active_model.name.unwrap();
        assert_eq!(name, expected_name);
        let path = active_model.path.unwrap();
        let content = fs::read(path).unwrap();
        assert_eq!(&content, expected_content.as_bytes());

        TEST_NAME.set(None);
    }
}
