//! This module implements [`ExternalRecipeGetterTrait`] for [Pinterest](https://www.pinterest.com/).

use std::sync::OnceLock;

use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;
use scraper::Dom;
use url::Url;

static PIN_IT_PATH_REGEX: OnceLock<Regex> = OnceLock::new();

static PINTEREST_PATH_REGEX: OnceLock<Regex> = OnceLock::new();

use crate::{
    external_recipe::{
        error::ExternalRecipeError,
        pinterest::relay_response::{
            PinterestRelay, PinterestRelayPinQueryData, PinterestRelayResponse,
        },
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
    async fn get(&self, url: Url) -> Result<ExternalRecipe, ExternalRecipeError> {
        let response = super::client().get(url.clone()).send().await?;
        let text = response.text().await?;
        let dom = Dom::create(text).await?;
        let elements = dom
            .select_all("script[data-relay-response=\"true\"][type=\"application/json\"]")
            .await?;
        // This is done inside a for-loop because of the "await?". https://github.com/rust-lang/rust/issues/62290
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
            return Err(ExternalRecipeError::ParseError(String::from(
                "HTML does not contain a usable relay response.",
            )));
        };
        match relay_response.data.pin_query.data {
            PinterestRelayPinQueryData::Uploaded(data) => Ok(ExternalRecipe {
                name: data.title,
                steps: vec![ExternalRecipeStep {
                    ingredients: vec![],
                    description: data
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
                    files: data
                        .story_pin_data
                        .pages
                        .into_iter()
                        .flat_map(|page| page.blocks)
                        .map(|block| block.video_data.video_list.video.url)
                        .collect(),
                }],
                ..Default::default()
            }),
            PinterestRelayPinQueryData::External(data) => {
                let external_recipe = crate::external_recipe::get(data.link).await?;
                Ok(external_recipe)
            }
        }
    }

    fn url_matches(&self) -> Vec<UrlMatch<'static>> {
        vec![pin_it_uri_match(), pinterest_uri_match()]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::external_recipe::tests::{assert_expected_gets, ExpectedGet};

    fn expected_gets() -> Vec<ExpectedGet> {
        vec![
            ExpectedGet {
                // pin to https://www.pinterest.de/pin/568227678004669298/
                url: String::from("https://pin.it/17F496pQJ"),
                external_recipe: ExternalRecipe {
                    name: String::from("Roasted Pepper Pasta Soße"),
                    steps: vec![ExternalRecipeStep {
                        description: String::from(
                            "Zutaten:\n\n- 3-4 Rote Paprika\n- 1 Zwiebel\n- 1 Knoblauchknolle\n- Olivenöl\n- 500g Pasta der Wahl\n- Handvoll Parmesan\n\nZubereitung:\n\n1. Um Zeit zu sparen schonmal vorab den Ofen auf 220 Grad Ober/Unter vorheizen\n2. Paprika waschen und Stiel entfernen. Anschliessend die Paprika, die geviertelte Zwiebel und die queer halbierte Knoblauchzehe in einer Auflaufform verteilen. Sparsam etwas Olivenöl verteilen und das Gemüse im Backofen für ca. 30-40 Minuten backen. (Solange bis sie aussehen wie im Video)\n3. Den Inhalt der Auflaufform etwas abkühlen lassen und dann in einen Mixbehälter geben und zusammen mit einer Handvoll Parmesan und etwas Olivenöl zu einer Creme mixen\n4. Pasta in sehr gut gesalzenem Wasser kochen (in der Sauce ist kein Salz)\n5. Sosse, in einer Pfanne kurz anbraten und zusammen mit 1-2 Kellen Pastawasser mit den gekochten Pasta zu einer Creme vermischen\n\nAm besten zusammen mit noch mehr Parmesan servieren.\n",
                        ),
                        files: vec![String::from(
                            "https://v1.pinimg.com/videos/mc/expMp4/a9/a2/66/a9a266243020587a931e459fa4fd9853_t1.mp4",
                        )],
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            },
            ExpectedGet {
                // pin to https://knusperstuebchen.net/2019/03/14/blumenkohl-curry-vegetarisches-soulfood/
                url: String::from("https://pin.it/3Zy729tUR"),
                external_recipe: ExternalRecipe {
                    name: String::from("Blumenkohl-Curry: Vegetarisches Soulfood"),
                    steps: vec![ExternalRecipeStep {
                        ingredients: vec![
                            String::from("1 Blumenkohl (ca. 750-1000g)"),
                            String::from("1 Zwiebel"),
                            String::from("1 Knoblauchzehe"),
                            String::from("3 EL Olivenöl"),
                            String::from("400 ml Gemüsebrühe"),
                            String::from("400 ml Kokosmilch"),
                            String::from("2 TL gem. Paprikapulver"),
                            String::from("1 TL gem. Kurkuma"),
                            String::from("1 TL gem. Ingwer"),
                            String::from("2 EL gem. Currypulver"),
                            String::from("50 g Tomatenmark"),
                            String::from("1 TL Oregano"),
                            String::from("1 TL Thymian"),
                            String::from("1-2 TL Salz"),
                            String::from("1 TL schwarzen Pfeffer"),
                            String::from("1 Msp. Chilipulver"),
                            String::from("1 Msp. Kreuzkümmel"),
                            String::from("150 g TK Erbsen"),
                            String::from("Basmati Reis für 4 Personen"),
                        ],
                        description: String::from(
                            "So wird's gemacht\nZunächst Blumenkohl waschen, Röschen vom Strunk befreien und klein schneiden. Derweil Zwiebel und Knoblauchzehe fein hacken. Öl in einer Pfanne erhitzen und Zwiebel und Knoblauch darin anschwitzen, Paprikapulver hinzugeben und leicht mitrösten. Blumenkohl hinzufügen und anbraten. Mit Gemüsebrühe und Kokosmilch ablöschen. Die restlichen Gewürze und Kräuter sowie das Tomatenmark hinzugeben, gut verrühren und 20 Minuten köcheln lassen, dabei immer mal wieder umrühren. Derweil den Reis kochen. Zum Schluss die Erbsen in das Curry rühren, nochmals abschmecken und 10 Minuten ziehen lassen. Mit Reis servieren.",
                        ),
                        files: vec![
                            String::from(
                                "https://knusperstuebchen.net/wp-content/uploads/2019/03/Blumenkohl-Curry-mit-Reis-Cauliflower-Curry-with-Rice-11.jpg",
                            ),
                            String::from(
                                "https://knusperstuebchen.net/wp-content/uploads/2019/03/Blumenkohl-Curry-mit-Reis-Cauliflower-Curry-with-Rice-Rezept.pdf",
                            ),
                        ],
                    }],
                    ..Default::default()
                },
            },
            ExpectedGet {
                // pin to https://instakoch.de/quiche-lorraine/
                url: String::from("https://www.pinterest.de/pin/568227677997682303"),
                external_recipe: ExternalRecipe {
                    name: "Quiche Lorraine".to_string(),
                    ingredients: vec![
                        "250 g Mehl".to_string(),
                        "125 g Butter".to_string(),
                        "½ TL Salz".to_string(),
                        "1  Eigelb".to_string(),
                        "3-4 EL Wasser".to_string(),
                        "300 g gekochter Schinken".to_string(),
                        "1  Stange Lauch".to_string(),
                        "3  Zwiebeln".to_string(),
                        "200 ml Sahne".to_string(),
                        "1 Becher Creme Fraiche".to_string(),
                        "150 g Reibekäse".to_string(),
                        "3  Eier".to_string(),
                        "1 TL Salz".to_string(),
                        "½ TL Schwarzer Pfeffer".to_string(),
                        "1 Prise Muskatnuss".to_string(),
                        "½ Bund Petersilie".to_string(),
                        "Öl".to_string(),
                    ],
                    files: vec![
                        "https://instakoch.de/wp-content/uploads/2019/03/00000PORTRAIT_00000_BURST20190307160523971-01.jpeg".to_string(),
                        "https://instakoch.de/wp-content/uploads/2019/03/00000PORTRAIT_00000_BURST20190307160523971-01-500x500.jpeg".to_string(),
                        "https://instakoch.de/wp-content/uploads/2019/03/00000PORTRAIT_00000_BURST20190307160523971-01-500x375.jpeg".to_string(),
                        "https://instakoch.de/wp-content/uploads/2019/03/00000PORTRAIT_00000_BURST20190307160523971-01-480x270.jpeg".to_string(),
                    ],
                    steps: vec![
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Das Mehl sieben und mit der Butter in die Küchenmaschine geben. Kurz mixen. Ei und Wasser hinzufügen und kneten, bis ein glatter Teig entstanden ist. Herausnehmen und kurz von Hand kneten.".to_string(),
                            files: vec![
                                "https://instakoch.de/wp-content/uploads/2019/03/00100dPORTRAIT_00100_BURST20190307134048578_COVER-01-01.jpeg".to_string(),
                            ],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Teig in Frischhaltefolie wickeln und 30 Minuten lang kühl stellen.".to_string(),
                            files: vec![
                                "https://instakoch.de/wp-content/uploads/2019/03/IMG_20190307_130210.jpg".to_string(),
                            ],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Jetzt den Backofen auf 200 Grad Ober- und Unterhitze vorheizen.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Die Tarteform leicht einfetten.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Die Zwiebeln und den Lauch klein schneiden.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "In einer Pfanne, mit wenig Öl, den Schinken kurz anbraten.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Die Zwiebeln und den Lauch hinzufügen und anschwitzen.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "In einer Schüssel, die Eier mit Sahne und Crème fraîche miteinander vermischen.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Mit Salz, schwarzem Pfeffer und Muskatnuss abschmecken.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Die gehackte Petersilie hinzugeben und alles gut miteinander vermischen.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Den Teig auf bemehlter Fläche ausrollen, bis er 3 cm größer ist als die Tarteform.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Den Teig in die Tarteform geben, festdrücken und den Rand abschneiden.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Die Hälfte der Zwiebel-Lauch-Mischung in die Tarte Form geben.".to_string(),
                            files: vec![
                                "https://instakoch.de/wp-content/uploads/2019/03/00100dPORTRAIT_00100_BURST20190307144836120_COVER-01.jpeg".to_string(),
                            ],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Dann den Reibekäse und die Hälfte der Sahne-Ei-Mischung hinzugeben.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Den Rest der Zwiebel-Lauch-Mischung hinzufügen.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Etwas Reibekäse und den Rest der Sahne-Ei-Mischung hinzufügen und den übrigen Reibekäse obendrauf verteilen".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Dann etwa 45 Minuten lang im Backofen auf mittlerem Einschub backen, oder bis der Rand goldbraun ist.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Herausnehmen und 5 Minuten lang ruhen lassen. Dann aus der Form nehmen und portionsweise auf Tellern servieren.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Guten Appetit!".to_string(),
                            files: vec![],
                        },
                    ],
                },
            },
            /* ExpectedGet {
                // pin to https://www.madamecuisine.de/gemuese-lasagne-mit-spinat/
                url: String::from("https://www.pinterest.de/pin/568227677994153490"),
                external_recipe: ExternalRecipe {
                    ..Default::default()
                },
            }, */
        ]
    }

    #[tokio::test]
    async fn test_get() {
        crate::tests::run();
        let getter = ExternalRecipeGetter;
        assert_expected_gets(getter, expected_gets()).await;
    }
}
