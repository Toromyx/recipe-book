//! This module contains the [`std::error::Error`] for the [`crate::recipe_step_file_storage`] module.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RecipeStepFileStorageError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Db(#[from] sea_orm::DbErr),
}
