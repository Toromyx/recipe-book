//! This module handles getting data from external recipes in the world wide web.

use std::str::FromStr;

use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;
use url::Url;

use crate::external_recipe::error::ExternalRecipeError;

pub mod error;
pub mod pinterest;
pub mod sallys_welt;

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ExternalRecipe {
    pub name: String,
    pub steps: Vec<ExternalRecipeStep>,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ExternalRecipeStep {
    pub ingredients: Vec<String>,
    pub description: String,
    pub files: Vec<String>,
}

/// Get an external recipe from an URL.
pub async fn get(url_string: String) -> Result<ExternalRecipe, ExternalRecipeError> {
    let url = Url::from_str(&url_string).map_err(anyhow::Error::from)?;
    let external_recipe_getter_option = external_recipe_getters()
        .into_iter()
        .find(|external_recipe_getter| external_recipe_getter.can_get(&url));
    let Some(external_recipe_getter) = external_recipe_getter_option else {
        return Err(ExternalRecipeError::UrlNotSupported(url_string));
    };
    let external_recipe = external_recipe_getter.get(url).await?;
    Ok(external_recipe)
}

fn external_recipe_getters() -> Vec<Box<dyn ExternalRecipeGetterTrait>> {
    vec![
        Box::new(pinterest::ExternalRecipeGetter),
        Box::new(sallys_welt::ExternalRecipeGetter),
    ]
}

/// Represents an external recipe URL matching rule.
pub struct UrlMatch<'a> {
    pub schemes: &'a [&'a str],
    pub domains: &'a [&'a str],
    pub path_regex: &'a Regex,
}

/// A URL prepared for matching against [`UrlMatch`].
pub struct PreparedUrl<'a> {
    pub scheme: &'a str,
    /// This [`Vec`] contains an entry for each subdomain.
    pub domains: Vec<String>,
    pub path: &'a str,
}

impl UrlMatch<'_> {
    /// Prepare an URL for matching.
    ///
    /// This method returns [`None`] when the URL does not contain a domain, see [`Url::domain`].
    pub fn prepare_url(url: &Url) -> Option<PreparedUrl> {
        let (scheme, Some(domain), path) = (url.scheme(), url.domain(), url.path()) else {
            return None;
        };
        let domain_parts: Vec<&str> = domain.split('.').collect();
        let domains: Vec<String> = (0..(domain_parts.len() - 1))
            .map(|i| domain_parts[i..domain_parts.len()].join("."))
            .collect();
        Some(PreparedUrl {
            scheme,
            domains,
            path,
        })
    }

    /// Match against a prepared URL.
    pub fn is_match(&self, prepared_url: &PreparedUrl) -> bool {
        if !self.schemes.contains(&prepared_url.scheme) {
            return false;
        }
        if prepared_url
            .domains
            .iter()
            .all(|domain| !self.domains.contains(&&**domain))
        {
            return false;
        }
        self.path_regex.is_match(prepared_url.path)
    }
}

/// Implementors define which external recipes they can get and implement the getting itself.
#[async_trait]
pub trait ExternalRecipeGetterTrait: Send + Sync {
    /// Check whether this implementor can get an external recipe from a specific URL.
    fn can_get(&self, url: &Url) -> bool {
        let Some(prepared_url) = UrlMatch::prepare_url(url) else {
            return false;
        };
        for uri_match in self.url_matches() {
            if uri_match.is_match(&prepared_url) {
                return true;
            }
        }
        false
    }

    /// Get the external recipe from the URL.
    async fn get(&self, url: Url) -> Result<ExternalRecipe, ExternalRecipeError>;

    /// Get the [`Vec`] of [`UrlMatch`]es of this implementor.
    fn url_matches(&self) -> Vec<UrlMatch<'static>>;
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use pretty_assertions::assert_eq;
    use url::Url;

    use super::*;

    #[test]
    pub fn test_prepare_url_domains() {
        let url = Url::from_str("https://en.wikipedia.org").unwrap();
        let prepared_url = UrlMatch::prepare_url(&url).unwrap();
        assert_eq!(
            prepared_url.domains,
            vec!["en.wikipedia.org", "wikipedia.org"]
        )
    }
}
