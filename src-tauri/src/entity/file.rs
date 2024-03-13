//! This module implements the file entity.
//!
//! See [`Model`] for more information.

use async_trait::async_trait;
use log;
use sea_orm::{entity::prelude::*, IntoActiveModel, TryIntoModel};
use serde::Serialize;

/// This struct represents a file.
///
/// A file is a binary file.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "file")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub mime: String,
    pub path: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::recipe_step_file::Entity")]
    RecipeStepFile,
    #[sea_orm(has_many = "super::recipe_file::Entity")]
    RecipeFile,
}

impl Related<super::recipe_step_file::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecipeStepFile.def()
    }
}

impl Related<super::recipe_file::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecipeFile.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn after_delete<C>(self, _db: &C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let model = self.try_into_model()?;
        if let Err(err) = crate::file_storage::delete(&model.path).await {
            log::warn!(
                "Could not delete file from storage after deleting entity: {}",
                err
            );
        };
        Ok(model.into_active_model())
    }
}

/// Remove orphaned file entities.
///
/// An entity is orphaned if no other entity references it.
pub async fn remove_orphans<C>(db: &C) -> Result<(), DbErr>
where
    C: ConnectionTrait,
{
    let orphaned_files = Entity::find()
        .left_join(super::recipe_step_file::Entity)
        .filter(super::recipe_step_file::Column::Id.is_null())
        .left_join(super::recipe_file::Entity)
        .filter(super::recipe_file::Column::Id.is_null())
        .all(db)
        .await?;
    for orphaned_file in orphaned_files {
        orphaned_file.delete(db).await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::future::Future;

    use mime_guess::mime;
    use sea_orm::{ActiveValue, Iterable};
    use tokio::fs;

    use super::*;
    use crate::{
        file_storage,
        migrator::tests::get_memory_database_migrated,
        tests::{create_temp_file, TEST_NAME},
    };

    #[tokio::test]
    async fn test_after_delete() {
        TEST_NAME.set(Some("entity__file__test_after_delete".to_string()));
        crate::tests::run();

        let db = get_memory_database_migrated().await;
        let temp_path = create_temp_file("entity__file__test_after_delete.bin", "");
        let mime = mime::APPLICATION_OCTET_STREAM.to_string();
        let path = file_storage::create(&temp_path.to_string_lossy(), &mime)
            .await
            .unwrap();
        let active_model = ActiveModel {
            name: ActiveValue::Set("test".to_string()),
            mime: ActiveValue::Set(mime),
            path: ActiveValue::Set(path.to_string_lossy().to_string()),
            ..Default::default()
        };
        let model = active_model.insert(&db).await.unwrap();
        assert!(path.exists());
        model.delete(&db).await.unwrap();
        assert!(!path.exists());

        TEST_NAME.set(None);
    }

    #[tokio::test]
    async fn test_remove_orphans() {
        TEST_NAME.set(Some("entity__file__test_remove_orphans".to_string()));
        crate::tests::run();

        let db = get_memory_database_migrated().await;
        for relation in Relation::iter() {
            match relation {
                Relation::RecipeStepFile | Relation::RecipeFile => {
                    // known relation, add other known relations here if they are tested below or are irrelevant for orphan removal
                }
            }
        }

        async fn create_file(db: &DatabaseConnection) -> Model {
            let temp_file = create_temp_file("entity__file__test_remove_orphans.bin", "");
            ActiveModel {
                name: ActiveValue::Set("entity__file__test_remove_orphans".to_string()),
                mime: ActiveValue::Set(mime::APPLICATION_OCTET_STREAM.to_string()),
                path: ActiveValue::Set(
                    file_storage::create(
                        &temp_file.to_string_lossy(),
                        mime::APPLICATION_OCTET_STREAM.as_ref(),
                    )
                    .await
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                ),
                ..Default::default()
            }
            .insert(db)
            .await
            .unwrap()
        }

        async fn recipe_step_file_create(
            file_id: i64,
            db: &DatabaseConnection,
        ) -> super::super::recipe_step_file::Model {
            let recipe = super::super::recipe::ActiveModel {
                name: ActiveValue::Set("Recipe".to_string()),
                ..Default::default()
            }
            .insert(db)
            .await
            .unwrap();
            let recipe_step = super::super::recipe_step::ActiveModel {
                order: ActiveValue::Set(1),
                description: ActiveValue::Set("Recipe Step".to_string()),
                recipe_id: ActiveValue::Set(recipe.id),
                ..Default::default()
            }
            .insert(db)
            .await
            .unwrap();
            super::super::recipe_step_file::ActiveModel {
                order: ActiveValue::Set(1),
                recipe_step_id: ActiveValue::Set(recipe_step.id),
                file_id: ActiveValue::Set(file_id),
                ..Default::default()
            }
            .insert(db)
            .await
            .unwrap()
        }

        async fn recipe_step_file_update(
            recipe_step_file: super::super::recipe_step_file::Model,
            file_id: i64,
            db: &DatabaseConnection,
        ) -> super::super::recipe_step_file::Model {
            let mut recipe_step_file_am = recipe_step_file.into_active_model();
            recipe_step_file_am.file_id = ActiveValue::Set(file_id);
            recipe_step_file_am.update(db).await.unwrap()
        }

        async fn recipe_file_create(
            file_id: i64,
            db: &DatabaseConnection,
        ) -> super::super::recipe_file::Model {
            let recipe = super::super::recipe::ActiveModel {
                name: ActiveValue::Set("Recipe".to_string()),
                ..Default::default()
            }
            .insert(db)
            .await
            .unwrap();
            super::super::recipe_file::ActiveModel {
                order: ActiveValue::Set(1),
                recipe_id: ActiveValue::Set(recipe.id),
                file_id: ActiveValue::Set(file_id),
                ..Default::default()
            }
            .insert(db)
            .await
            .unwrap()
        }

        async fn recipe_file_update(
            recipe_file: super::super::recipe_file::Model,
            file_id: i64,
            db: &DatabaseConnection,
        ) -> super::super::recipe_file::Model {
            let mut recipe_file_am = recipe_file.into_active_model();
            recipe_file_am.file_id = ActiveValue::Set(file_id);
            recipe_file_am.update(db).await.unwrap()
        }

        async fn test_related_entity_orphan_removal<
            'db,
            RelatedModel,
            RelatedActiveModel,
            RelatedEntity,
            Create,
            CreateReturn,
            Update,
            UpdateReturn,
        >(
            related_model_create: Create,
            related_model_update: Update,
            db: &'db DatabaseConnection,
        ) where
            RelatedModel: ModelTrait<Entity = RelatedEntity> + IntoActiveModel<RelatedActiveModel>,
            RelatedActiveModel:
                ActiveModelTrait<Entity = RelatedEntity> + ActiveModelBehavior + Send,
            RelatedEntity: EntityTrait<Model = RelatedModel>,
            Create: Fn(i64, &'db DatabaseConnection) -> CreateReturn,
            CreateReturn: Future<Output = RelatedModel>,
            Update: Fn(RelatedModel, i64, &'db DatabaseConnection) -> UpdateReturn,
            UpdateReturn: Future<Output = RelatedModel>,
        {
            let file_a = create_file(db).await;
            let related_model = related_model_create(file_a.id, db).await;
            remove_orphans(db).await.unwrap();
            assert!(
                Entity::find_by_id(file_a.id)
                    .one(db)
                    .await
                    .unwrap()
                    .is_some()
            );
            assert!(fs::try_exists(&file_a.path).await.unwrap());
            let file_b = create_file(db).await;
            let related_model = related_model_update(related_model, file_b.id, db).await;
            remove_orphans(db).await.unwrap();
            assert!(
                Entity::find_by_id(file_b.id)
                    .one(db)
                    .await
                    .unwrap()
                    .is_some()
            );
            assert!(fs::try_exists(&file_b.path).await.unwrap());
            assert!(
                Entity::find_by_id(file_a.id)
                    .one(db)
                    .await
                    .unwrap()
                    .is_none()
            );
            assert!(!fs::try_exists(&file_a.path).await.unwrap());
            related_model.delete(db).await.unwrap();
            assert!(
                Entity::find_by_id(file_b.id)
                    .one(db)
                    .await
                    .unwrap()
                    .is_none()
            );
            assert!(!fs::try_exists(&file_b.path).await.unwrap());
        }

        test_related_entity_orphan_removal(recipe_step_file_create, recipe_step_file_update, &db)
            .await;
        test_related_entity_orphan_removal(recipe_file_create, recipe_file_update, &db).await;

        TEST_NAME.set(None);
    }
}
