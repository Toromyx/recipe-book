//! This module implements [`EntityCrudTrait`] for [`crate::command`].

use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

use crate::external_recipe::error::ExternalRecipeError;

#[serde_as]
#[derive(Debug, Error, Serialize)]
pub enum CommandError {
    #[error(transparent)]
    Db(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        sea_orm::error::DbErr,
    ),
    #[error(transparent)]
    OcrInitialize(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        tesseract::InitializeError,
    ),
    #[error(transparent)]
    OcrSetImage(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        tesseract::SetImageError,
    ),
    #[error(transparent)]
    OcrGetHocrText(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        tesseract::plumbing::TessBaseApiGetHocrTextError,
    ),
    #[error(transparent)]
    Reqwest(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        reqwest::Error,
    ),
    #[error(transparent)]
    Tauri(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        tauri::Error,
    ),
    #[error(transparent)]
    Anyhow(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        anyhow::Error,
    ),
    #[error("The external recipe url \"{0}\" is not supported.")]
    ExternalRecipeUrlNotSupported(String),
    #[error(transparent)]
    ExternalRecipeParseError(#[serde_as(as = "DisplayFromStr")] ExternalRecipeError),
    #[error("Entity was not found.")]
    NotFound,
}

impl From<ExternalRecipeError> for CommandError {
    fn from(value: ExternalRecipeError) -> Self {
        match value {
            ExternalRecipeError::Reqwest(reqwest) => Self::Reqwest(reqwest),
            ExternalRecipeError::Anyhow(anyhow) => Self::Anyhow(anyhow),
            ExternalRecipeError::UrlNotSupported(url) => Self::ExternalRecipeUrlNotSupported(url),
            ExternalRecipeError::ParseError(_) => Self::ExternalRecipeParseError(value),
        }
    }
}
