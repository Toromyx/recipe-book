//! This module handles getting data from external recipes in the world wide web.

use std::str::FromStr;

use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;
use serde::Serialize;
use url::Url;

use crate::external_recipe::error::ExternalRecipeError;

pub mod error;
pub mod sallys_welt;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalRecipeData {
    data: String,
    instructions: Instructions,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Instructions {
    JsModule { name: String },
}

pub async fn get(url_string: String) -> Result<ExternalRecipeData, ExternalRecipeError> {
    let url = Url::from_str(&url_string).map_err(anyhow::Error::from)?;
    let external_recipe_getter_option = external_recipe_getters()
        .into_iter()
        .find(|external_recipe_getter| external_recipe_getter.can_get(&url));
    let Some(external_recipe_getter) = external_recipe_getter_option else {
        return Err(ExternalRecipeError::UrlNotSupported());
    };
    let external_recipe = external_recipe_getter.get(url).await?;
    Ok(external_recipe)
}

fn external_recipe_getters() -> Vec<impl ExternalRecipeGetterTrait> {
    vec![sallys_welt::ExternalRecipeGetter]
}

#[async_trait]
pub trait ExternalRecipeGetterTrait {
    fn can_get(&self, url: &Url) -> bool {
        let Some(host) = url.host_str() else {
            return false
        };
        if !Self::hosts().contains(&host) {
            return false;
        };
        let path = url.path();
        Self::path_regex()
            .unwrap_or_else(|err| panic!("Could not get path regex for domain \"{host}\": {err}"))
            .is_match(path)
    }

    async fn get(&self, url: Url) -> Result<ExternalRecipeData>;

    fn hosts() -> Vec<&'static str>;

    fn path_regex() -> Result<Regex, regex::Error>;
}
