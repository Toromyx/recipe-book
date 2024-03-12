//! This module contains the [`std::error::Error`] for the [`crate::file_storage`] module.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileStorageError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
