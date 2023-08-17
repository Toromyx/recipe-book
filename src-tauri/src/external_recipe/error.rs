use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExternalRecipeError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error("The external recipe url \"{0}\" is not supported.")]
    UrlNotSupported(String),
    #[error("{0}")]
    ParseError(String),
}
