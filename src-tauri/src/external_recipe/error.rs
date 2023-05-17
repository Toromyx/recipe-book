use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExternalRecipeError {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error("This external recipe url is not supported.")]
    UrlNotSupported(),
}
