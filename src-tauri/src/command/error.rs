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
    Ocr(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        tesseract::TesseractError,
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
