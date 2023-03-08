use thiserror::Error;

use crate::recipe_file_storage::error::RecipeFileStorageError;

#[derive(Debug, Error)]
pub enum EntityApiError {
    #[error(transparent)]
    Db(#[from] sea_orm::DbErr),
    #[error(transparent)]
    Tauri(#[from] tauri::Error),
    #[error(transparent)]
    RecipeFileStorage(#[from] RecipeFileStorageError),
}
