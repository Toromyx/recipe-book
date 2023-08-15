//! This module implements [`ExternalRecipeGetterTrait`] for [Pinterest](https://www.pinterest.com/).

use std::sync::OnceLock;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use regex::Regex;
use scraper::Dom;
use url::Url;

static PIN_IT_PATH_REGEX: OnceLock<Regex> = OnceLock::new();

static PINTEREST_PATH_REGEX: OnceLock<Regex> = OnceLock::new();

use crate::{
    external_recipe::{
        pinterest::relay_response::{PinterestRelay, PinterestRelayResponse},
        ExternalRecipe, ExternalRecipeGetterTrait, ExternalRecipeStep, UrlMatch,
    },
    scraper,
    scraper::ParentNode,
};

mod relay_response;

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
    async fn get(&self, url: Url) -> Result<ExternalRecipe> {
        let response = reqwest::get(url.clone()).await?;
        let mut text = response.text().await?;
        if let Some(prepared_url) = UrlMatch::prepare_url(&url) {
            if pin_it_uri_match().is_match(&prepared_url) {
                let dom = Dom::create(text).await?;
                let canonical_link = dom.canonical_link().await?;
                let response = reqwest::get(canonical_link).await?;
                text = response.text().await?;
            }
        }
        let dom = Dom::create(text).await?;
        let elements = dom
            .select_all("script[data-relay-response=\"true\"][type=\"application/json\"]")
            .await?;
        let mut relay_response_option: Option<PinterestRelayResponse> = None;
        for element in elements {
            let text_content = element.text_content().await?;
            let relay_result = serde_json::from_str::<'_, PinterestRelay>(&text_content);
            if let Ok(relay) = relay_result {
                relay_response_option = Some(relay.response);
                break;
            }
        }
        let Some(relay_response) = relay_response_option else {
            return Err(anyhow!("HTML does not contain a usable relay response."));
        };
        Ok(ExternalRecipe {
            name: relay_response.data.pin_query.data.title,
            steps: vec![ExternalRecipeStep {
                ingredients: vec![],
                description: relay_response
                    .data
                    .pin_query
                    .data
                    .story_pin_data
                    .metadata
                    .basics
                    .list_blocks
                    .into_iter()
                    .map(|list_block| {
                        format!(
                            "{}\n{}",
                            list_block.heading,
                            list_block
                                .blocks
                                .into_iter()
                                .map(|block_item| block_item.text)
                                .collect::<Vec<String>>()
                                .join("\n")
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n"),
                files: relay_response
                    .data
                    .pin_query
                    .data
                    .story_pin_data
                    .pages
                    .into_iter()
                    .flat_map(|page| page.blocks)
                    .map(|block| block.video_data.video_list.video.url)
                    .collect(),
            }],
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

    #[derive(Debug, Clone)]
    struct ExpectedGet {
        input: String,
        output: ExternalRecipe,
    }

    fn expected_roasted_pepper_pasta_sauce() -> ExternalRecipe {
        ExternalRecipe {
            name: String::from("Roasted Pepper Pasta Soße"),
            steps: vec![ExternalRecipeStep {
                ingredients: vec![],
                description: String::from("Zutaten:\n\n- 3-4 Rote Paprika\n- 1 Zwiebel\n- 1 Knoblauchknolle\n- Olivenöl\n- 500g Pasta der Wahl\n- Handvoll Parmesan\n\nZubereitung:\n\n1. Um Zeit zu sparen schonmal vorab den Ofen auf 220 Grad Ober/Unter vorheizen\n2. Paprika waschen und Stiel entfernen. Anschliessend die Paprika, die geviertelte Zwiebel und die queer halbierte Knoblauchzehe in einer Auflaufform verteilen. Sparsam etwas Olivenöl verteilen und das Gemüse im Backofen für ca. 30-40 Minuten backen. (Solange bis sie aussehen wie im Video)\n3. Den Inhalt der Auflaufform etwas abkühlen lassen und dann in einen Mixbehälter geben und zusammen mit einer Handvoll Parmesan und etwas Olivenöl zu einer Creme mixen\n4. Pasta in sehr gut gesalzenem Wasser kochen (in der Sauce ist kein Salz)\n5. Sosse, in einer Pfanne kurz anbraten und zusammen mit 1-2 Kellen Pastawasser mit den gekochten Pasta zu einer Creme vermischen\n\nAm besten zusammen mit noch mehr Parmesan servieren.\n"),
                files: vec![String::from("https://v1.pinimg.com/videos/mc/expMp4/a9/a2/66/a9a266243020587a931e459fa4fd9853_t1.mp4")],
            }],
        }
    }

    fn expected_gets() -> Vec<ExpectedGet> {
        vec![
            ExpectedGet {
                input: String::from("https://pin.it/3qVfEYG"),
                output: expected_roasted_pepper_pasta_sauce(),
            },
            ExpectedGet {
                input: String::from("https://www.pinterest.de/pin/568227678004669298/"),
                output: expected_roasted_pepper_pasta_sauce(),
            },
        ]
    }

    #[tokio::test]
    async fn test_get() {
        crate::tests::run();
        let getter = ExternalRecipeGetter;
        for expected_get in expected_gets() {
            let actual = getter
                .get(Url::from_str(&expected_get.input).unwrap())
                .await
                .unwrap();
            assert_eq!(actual, expected_get.output);
        }
    }
}
