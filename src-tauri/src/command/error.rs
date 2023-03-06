use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

use crate::api::entity::error::EntityApiError;

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
    EntityApi(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        EntityApiError,
    ),
    #[error("Entity was not found.")]
    NotFound,
}
