//! This module implements [`ExternalRecipeGetterTrait`] for the domain `sallys-blog.de`.

use std::str::FromStr;

use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;
use url::Url;

use crate::external_recipe::{ExternalRecipeData, ExternalRecipeGetterTrait, Instructions};

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

    fn hosts() -> Vec<&'static str> {
        vec!["sallys-blog.de"]
    }

    fn path_regex() -> Result<Regex, regex::Error> {
        Regex::from_str(r"^/rezepte/.*$")
    }
}
