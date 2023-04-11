//! This module contains the [`std::error::Error`] for the [`crate::protocol::recipe_file`] module.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RecipeFileProtocolError {
    #[error(transparent)]
    UrlParse(#[from] url::ParseError),
    #[error(transparent)]
    StdIo(#[from] std::io::Error),
    #[error("Could not build the response.")]
    ResponseBuild,
}
