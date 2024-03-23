use async_trait::async_trait;
use regex::Regex;
use url::Url;

use crate::external_recipe::{error::ExternalRecipeError, ExternalRecipe};

mod knusperstuebchen;
mod pinterest;
mod sallys_welt;

/// Implementors define which external recipes they can get and implement the getting itself.
#[async_trait]
pub trait SpecificExternalRecipeGetterTrait: Send + Sync {
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

pub fn specific_external_recipe_getters() -> Vec<Box<dyn SpecificExternalRecipeGetterTrait>> {
    vec![
        Box::new(pinterest::ExternalRecipeGetter),
        Box::new(sallys_welt::ExternalRecipeGetter),
        Box::new(knusperstuebchen::ExternalRecipeGetter),
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
    /// Prepare a URL for matching.
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
    fn is_match(&self, prepared_url: &PreparedUrl) -> bool {
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use url::Url;

    use super::*;

    #[test]
    pub fn test_prepare_url_domains() {
        let url = Url::from_str("https://en.wikipedia.org").unwrap();
        let prepared_url = UrlMatch::prepare_url(&url).unwrap();
        pretty_assertions::assert_eq!(
            prepared_url.domains,
            vec!["en.wikipedia.org", "wikipedia.org"]
        )
    }
}
