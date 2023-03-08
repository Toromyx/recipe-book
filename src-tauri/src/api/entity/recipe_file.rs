use async_trait::async_trait;
use mime_guess::mime;
use sea_orm::{
    sea_query::IntoCondition,
    ActiveModelTrait,
    ActiveValue::{NotSet, Set, Unchanged},
    ColumnTrait, Condition, DeriveIntoActiveModel, IntoActiveModel,
};
use serde::Deserialize;

use crate::{
    api::entity::{error::EntityApiError, EntityCrudTrait, Filter},
    database,
    entity::recipe_file::{ActiveModel, Column, Entity, Model, PrimaryKey, Relation},
    event::channel::{
        ENTITY_ACTION_CREATED_RECIPE_FILE, ENTITY_ACTION_DELETED_RECIPE_FILE,
        ENTITY_ACTION_UPDATED_RECIPE_FILE,
    },
    recipe_file_storage,
};

#[derive(Debug, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct RecipeFileCreate {
    pub name: String,
    pub order: i64,
    pub path: String,
    pub recipe_step_id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeFileUpdate {
    pub id: i64,
    pub name: Option<String>,
    pub order: Option<i64>,
}

impl IntoActiveModel<ActiveModel> for RecipeFileUpdate {
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

pub type RecipeFileFilter = Filter<RecipeFileCondition, RecipeFileOrderBy>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeFileCondition {
    pub recipe_step_id: Option<i64>,
}

impl IntoCondition for RecipeFileCondition {
    fn into_condition(self) -> Condition {
        Condition::all().add_option(
            self.recipe_step_id
                .map(|recipe_step_id| Column::RecipeStepId.eq(recipe_step_id)),
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecipeFileOrderBy {
    Order,
}

impl From<RecipeFileOrderBy> for Column {
    fn from(value: RecipeFileOrderBy) -> Self {
        match value {
            RecipeFileOrderBy::Order => Column::Order,
        }
    }
}

pub struct RecipeFileCrud {}

#[async_trait]
impl EntityCrudTrait for RecipeFileCrud {
    type Entity = Entity;
    type Model = Model;
    type ActiveModel = ActiveModel;
    type Column = Column;
    type Relation = Relation;
    type PrimaryKey = PrimaryKey;
    type EntityCreate = RecipeFileCreate;
    type EntityUpdate = RecipeFileUpdate;
    type EntityCondition = RecipeFileCondition;
    type EntityOrderBy = RecipeFileOrderBy;

    async fn pre_create(create: RecipeFileCreate) -> Result<ActiveModel, EntityApiError> {
        let mime = mime_guess::from_path(&create.path)
            .first_or(mime::APPLICATION_OCTET_STREAM)
            .to_string();
        let mut active_model = create.into_active_model();
        active_model.mime = Set(mime);
        Ok(active_model)
    }

    async fn post_create(model: Model) -> Result<Model, EntityApiError> {
        let db = database::connect().await;
        recipe_file_storage::create(&model).await?;
        let path_segments = recipe_file_storage::path_segments(&model).await?;
        let path = path_segments.join("/");
        let mut active_model = model.into_active_model();
        active_model.path = Set(path);
        let model = active_model.update(db).await?;
        Ok(model)
    }

    fn primary_key_value(model: &Model) -> i64 {
        model.id
    }

    fn primary_key_colum() -> Column {
        Column::Id
    }

    fn entity_action_created_channel() -> &'static str {
        ENTITY_ACTION_CREATED_RECIPE_FILE
    }

    fn entity_action_updated_channel() -> &'static str {
        ENTITY_ACTION_UPDATED_RECIPE_FILE
    }

    fn entity_action_deleted_channel() -> &'static str {
        ENTITY_ACTION_DELETED_RECIPE_FILE
    }
}
