use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

#[serde_as]
#[derive(Debug, Error, Serialize)]
pub enum CommandError {
    #[error(transparent)]
    DbErr(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        sea_orm::error::DbErr,
    ),
    #[error("Entity was not found.")]
    NotFound,
}
