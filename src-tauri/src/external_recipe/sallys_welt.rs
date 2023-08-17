//! This module implements [`ExternalRecipeGetterTrait`] for the domain `sallys-blog.de`.

use std::sync::{Arc, OnceLock};

use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;
use url::Url;

use crate::{
    external_recipe::{
        error::ExternalRecipeError, ExternalRecipe, ExternalRecipeGetterTrait, ExternalRecipeStep,
        UrlMatch,
    },
    scraper::{Dom, ParentNode},
};

static PATH_REGEX: OnceLock<Regex> = OnceLock::new();

pub struct ExternalRecipeGetter;

#[async_trait]
impl ExternalRecipeGetterTrait for ExternalRecipeGetter {
    async fn get(&self, url: Url) -> Result<ExternalRecipe, ExternalRecipeError> {
        let response = reqwest::get(url).await?;
        let text = response.text().await?;
        let dom = Arc::new(Dom::create(text).await?);
        let mut steps = vec![];
        for step_element in dom.select_all(".recipe").await? {
            let mut ingredients = vec![];
            for ingredient_element in dom
                .select_all(".flex.items-start.justify-start.mb-1.space-x-3.text-lg.sm\\:text-base")
                .await?
            {
                ingredients.push(String::from(
                    ingredient_element.text_content().await?.trim(),
                ));
            }
            let images_wrap_element = dom.select(".images-wrap").await?;
            let mut files = vec![];
            for img_element in images_wrap_element.select_all("img").await? {
                files.push(format!(
                    "https://sallys-blog.de{}",
                    img_element.get_attribute("src").await?
                ));
            }
            steps.push(ExternalRecipeStep {
                ingredients,
                description: step_element.text_content().await?,
                files,
            });
        }
        Ok(ExternalRecipe {
            name: dom.select("h1").await?.text_content().await?,
            steps,
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
