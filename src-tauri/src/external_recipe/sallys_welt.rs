//! This module implements [`ExternalRecipeGetterTrait`] for the domain `sallys-blog.de`.

use std::sync::OnceLock;

use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;
use url::Url;

use crate::external_recipe::{
    ExternalRecipeData, ExternalRecipeGetterTrait, Instructions, UrlMatch,
};

static PATH_REGEX: OnceLock<Regex> = OnceLock::new();

pub struct ExternalRecipeGetter;

#[async_trait]
impl ExternalRecipeGetterTrait for ExternalRecipeGetter {
    async fn get(&self, url: Url) -> Result<ExternalRecipeData> {
        let response = reqwest::get(url).await?;
        let text = response.text().await?;
        Ok(ExternalRecipeData {
            data: text,
            instructions: Instructions::JsModule {
                name: String::from("sallys-welt"),
            },
        })
    }

    fn url_matches(&self) -> Vec<UrlMatch<'static>> {
        vec![UrlMatch {
            schemes: &["http", "https"],
            domains: &["sallys-blog.de"],
            path_regex: PATH_REGEX.get_or_init(|| Regex::new(r"^/rezepte/.*$").unwrap()),
        }]
    }
}
