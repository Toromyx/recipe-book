use tauri::{
    http::{Request, Response, ResponseBuilder},
    AppHandle,
};
use url::Url;

use crate::{protocol::recipe_file::error::RecipeFileProtocolError, recipe_file_storage};

pub mod error;

pub const URI_SCHEME: &str = "recipe-file";

pub fn protocol(
    app_handle: &AppHandle,
    http_request: &Request,
) -> Result<Response, RecipeFileProtocolError> {
    let response = get(app_handle, http_request)?;
    Ok(response)
}

fn get(_app_handle: &AppHandle, request: &Request) -> Result<Response, RecipeFileProtocolError> {
    let mut path = recipe_file_storage::dir();
    let url = Url::parse(request.uri())?;
    let relative_path = percent_encoding::percent_decode_str(url.path())
        .decode_utf8_lossy()
        .to_string();
    for path_segment in relative_path.split('/') {
        path.push(path_segment);
    }
    let response = ResponseBuilder::new()
        .status(200)
        .body(std::fs::read(path)?)
        .map_err(|_| RecipeFileProtocolError::ResponseBuild)?;
    Ok(response)
}
