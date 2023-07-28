//! This module implements [`ExternalRecipeGetterTrait`] for [Pinterest](https://www.pinterest.com/).

use std::sync::OnceLock;

use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;
use url::Url;

use crate::external_recipe::{
    ExternalRecipeData, ExternalRecipeGetterTrait, Instructions, UrlMatch,
};

static PIN_IT_PATH_REGEX: OnceLock<Regex> = OnceLock::new();

static PINTEREST_PATH_REGEX: OnceLock<Regex> = OnceLock::new();

/// The [`UrlMatch`] for Pinterest's short URLs.
fn pin_it_uri_match() -> UrlMatch<'static> {
    UrlMatch {
        schemes: &["http", "https"],
        domains: &["pin.it"],
        path_regex: PIN_IT_PATH_REGEX.get_or_init(|| Regex::new(r".*").unwrap()),
    }
}

/// The [`UrlMatch`] for Pinterest's standard URLs.
fn pinterest_uri_match() -> UrlMatch<'static> {
    UrlMatch {
        schemes: &["http", "https"],
        domains: &[
            "pinterest.at",
            "pinterest.ca",
            "pinterest.ch",
            "pinterest.cl",
            "pinterest.co.kr",
            "pinterest.co.uk",
            "pinterest.com",
            "pinterest.com.au",
            "pinterest.com.mx",
            "pinterest.de",
            "pinterest.dk",
            "pinterest.es",
            "pinterest.fr",
            "pinterest.ie",
            "pinterest.info",
            "pinterest.it",
            "pinterest.jp",
            "pinterest.net",
            "pinterest.nz",
            "pinterest.ph",
            "pinterest.pt",
            "pinterest.ru",
            "pinterest.se",
        ],
        path_regex: PINTEREST_PATH_REGEX.get_or_init(|| Regex::new(r"^/pin/.*$").unwrap()),
    }
}

pub struct ExternalRecipeGetter;

#[async_trait]
impl ExternalRecipeGetterTrait for ExternalRecipeGetter {
    /// For `pin.it` URLs, we first need to find the canonical URL before trying to parse the HTML for the recipe data.
    async fn get(&self, url: Url) -> Result<ExternalRecipeData> {
        let response = reqwest::get(url.clone()).await?;
        let mut text = response.text().await?;
        if let Some(prepared_url) = UrlMatch::prepare_url(&url) {
            if pin_it_uri_match().is_match(&prepared_url) {
                let dom = crate::scraper::dom_create(text).await?;
                let canonical_link = crate::scraper::dom_canonical_link(dom.id.clone()).await?;
                let response = reqwest::get(canonical_link).await?;
                text = response.text().await?;
            }
        }
        Ok(ExternalRecipeData {
            data: text,
            instructions: Instructions::JsModule {
                name: String::from("pinterest"),
            },
        })
    }

    fn url_matches(&self) -> Vec<UrlMatch<'static>> {
        vec![pin_it_uri_match(), pinterest_uri_match()]
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    const PIN_IT_URL: &str = "https://pin.it/3qVfEYG";

    const PINTEREST_URL: &str = "https://www.pinterest.de/pin/568227678004669298/";

    #[tokio::test]
    async fn test_pin_it() {
        crate::tests::run();
        let getter = ExternalRecipeGetter;
        let result = getter.get(Url::from_str(PIN_IT_URL).unwrap()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_pinterest() {
        crate::tests::run();
        let getter = ExternalRecipeGetter;
        let result = getter.get(Url::from_str(PINTEREST_URL).unwrap()).await;
        assert!(result.is_ok());
    }
}
