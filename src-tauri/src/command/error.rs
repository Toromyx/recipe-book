//! This module implements [`EntityCrudTrait`] for [`crate::command`].

use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

use crate::entity_crud::error::EntityCrudError;

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
    EntityCrud(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        EntityCrudError,
    ),
    #[error("Entity was not found.")]
    NotFound,
}
