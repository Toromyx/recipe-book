//! This module contains the [`std::error::Error`] for the [`crate::entity_crud`] module.

use thiserror::Error;

use crate::recipe_file_storage::error::RecipeFileStorageError;

#[derive(Debug, Error)]
pub enum EntityCrudError {
    #[error(transparent)]
    Db(#[from] sea_orm::DbErr),
    #[error(transparent)]
    Tauri(#[from] tauri::Error),
    #[error(transparent)]
    RecipeFileStorage(#[from] RecipeFileStorageError),
}
