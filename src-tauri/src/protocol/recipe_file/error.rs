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
