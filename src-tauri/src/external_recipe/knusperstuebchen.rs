//! This module implements [`ExternalRecipeGetterTrait`] for [Knusperstübchen](https://knusperstuebchen.net/).

use std::sync::OnceLock;

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
    /// The recipes on Knusperstuebchen can be only a pdf or a pdf and structured html.
    async fn get(&self, url: Url) -> anyhow::Result<ExternalRecipe, ExternalRecipeError> {
        let response = reqwest::get(url).await?;
        let text = response.text().await?;
        let dom = Dom::create(text).await?;
        let name_element = dom.select("h1").await?.unwrap();
        let recipe_element_option = dom.select(".easyrecipe").await?;
        let pdf_anchor_element = dom.select("a[href$=\".pdf\"]").await?.unwrap();
        match recipe_element_option {
            None => Ok(ExternalRecipe {
                name: name_element.text_content().await?,
                steps: vec![ExternalRecipeStep {
                    ingredients: vec![],
                    description: String::from(""),
                    files: vec![pdf_anchor_element.get_attribute("href").await?],
                }],
            }),
            Some(recipe_element) => {
                let name_element = recipe_element.select(".ERSName").await?.unwrap();
                let ingredients_element = recipe_element.select(".ERSIngredients").await?.unwrap();
                let mut ingredients = vec![];
                for ingredient_element in ingredients_element.select_all("li").await? {
                    let inner_text = ingredient_element.inner_text().await?;
                    let trimmed = inner_text.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    ingredients.push(String::from(trimmed));
                }
                let description_element = recipe_element.select(".ERSInstructions").await?.unwrap();
                let img_element = recipe_element.select("img").await?.unwrap();
                Ok(ExternalRecipe {
                    name: name_element.text_content().await?,
                    steps: vec![ExternalRecipeStep {
                        ingredients,
                        description: String::from(
                            description_element
                                .inner_text()
                                .await?
                                .replace("  ", "\n")
                                .trim(),
                        ),
                        files: vec![
                            img_element.get_attribute("src").await?,
                            pdf_anchor_element.get_attribute("href").await?,
                        ],
                    }],
                })
            }
        }
    }

    fn url_matches(&self) -> Vec<UrlMatch<'static>> {
        vec![UrlMatch {
            schemes: &["http", "https"],
            domains: &["knusperstuebchen.net"],
            path_regex: PATH_REGEX.get_or_init(|| Regex::new(r"^/\d{4}/\d{2}/\d{2}/.*$").unwrap()),
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::external_recipe::tests::{assert_expected_gets, ExpectedGet};

    fn expected_gets() -> Vec<ExpectedGet> {
        vec![
            ExpectedGet {
                url: String::from(
                    "https://knusperstuebchen.net/2014/09/26/apfelkuchenzeit-schoenste-zeit-apfel-mazarin-kuchen/",
                ),
                external_recipe: ExternalRecipe {
                    name: String::from("Apfel-Mazarin-Kuchen"),
                    steps: vec![ExternalRecipeStep {
                        ingredients: vec![],
                        description: String::from(""),
                        files: vec![String::from(
                            "https://knusperstuebchen.net/wp-content/uploads/2014/09/Apfel-Mazarin-Kuchen.pdf",
                        )],
                    }],
                },
            },
            ExpectedGet {
                url: String::from(
                    "https://knusperstuebchen.net/2013/11/08/freitag-ist-pizzatag-auf-ins-wochenende/",
                ),
                external_recipe: ExternalRecipe {
                    name: String::from("Pizza Grundrezept - knusprig"),
                    steps: vec![ExternalRecipeStep {
                        ingredients: vec![
                            String::from("25g frische Hefe oder 1Pck. Trockenhefe"),
                            String::from("50ml Olivenöl"),
                            String::from("450g Mehl (Typ 550)"),
                            String::from("1TL Honig"),
                            String::from("250ml lauwarmes Wasser"),
                            String::from("1TL Meersalz"),
                            String::from("75ml Olivenöl"),
                            String::from("Meersalz"),
                            String::from("2 Zwiebeln"),
                            String::from("6 Knoblauchzehen"),
                            String::from("200ml Passata"),
                            String::from("100g Tomatenmark"),
                            String::from(
                                "nach Belieben 200g frische Tomaten oder gehackte Tomaten",
                            ),
                            String::from("175ml Rotwein"),
                            String::from("Prise Zucker"),
                            String::from("Prise Zimt"),
                            String::from("etwas Pfeffer"),
                            String::from("Gemüsebrühe (1x Bouillon Pur)"),
                            String::from("250g Mozzarella"),
                            String::from("50g Parmesan"),
                            String::from("Getrocknete Tomaten"),
                            String::from("Chorizo"),
                            String::from("Parmaschinken"),
                            String::from("Basilikum"),
                        ],
                        description: String::from(
                            "So wird's gemacht Für den Teig:\nHonig mit der Hefe cremig rühren, lauwarmes Wasser hinzufügen, alles verrühren bis die Hefe vollständig aufgelöst ist, abdecken und 10 Minuten ruhen lassen. Nun zu dem Hefegemisch das Mehl geben, auf das Mehl die restlichen Zutaten geben und zu einem homogenen Teig verkneten. Hände mit 1 TL Olivenöl einreiben und den Teig noch einmal kurz mit den Händen kneten, sodass der gesamte Teig mit Olivenöl bedeckt ist. In der Schüssel abgedeckt für 50 Minuten ruhen lassen.\nFür die Tomatensauce\nDerweil Olivenöl in einem Topf erhitzen, Zwiebeln anschwitzen. Nach 5 Minuten Knoblauch hinzugeben. Nun mit Rotwein ablöschen und kurz köcheln lassen. Passata und Gemüsebrühe hinzugeben, weiter köcheln lassen. Restliche Zutaten hinzugeben, Platte herunterstellen, leicht köcheln lassen, gelegentlich umrühren. Zum Schluss mit Pürierstab auf gewünschte Konsistenz pürieren. Entweder gleich verwenden oder in Gläser füllen und einkochen.\nFertigstellung:\nOfen inkl. Blech auf 220°C vorheizen. Nun Teig in bis zu vier gleich große Teile teilen, kreisförmig dünn ausrollen. Teig auf Backpapier legen. 2-3 EL Tomatensauce verteilen, darauf geriebenen Mozzarella geben. Mit gewünschten Zutaten wie Chorizo, Parma und Tomaten belegen. Zum Schluss mit etwas Parmesan bestreuen. Pizzen auf das heiße Blech geben ca. 15-20 Minuten backen.",
                        ),
                        files: vec![
                            String::from(
                                "https://knusperstuebchen.net/wp-content/uploads/2013/11/Knusprige-Lieblingspizza-mit-Chorizo-Pizza-Raclette-477x300.jpg",
                            ),
                            String::from(
                                "https://knusperstuebchen.net/wp-content/uploads/2013/11/Knusper-Pizza-Rezept.pdf",
                            ),
                        ],
                    }],
                },
            },
            ExpectedGet {
                url: String::from(
                    "https://knusperstuebchen.net/2023/08/16/perlcouscous-salat-mit-ofenlachs-und-blaubeeren/",
                ),
                external_recipe: ExternalRecipe {
                    name: String::from("Perlcouscous-Salat mit Ofenlachs und Blaubeeren"),
                    steps: vec![ExternalRecipeStep {
                        ingredients: vec![
                            String::from("400 g Lachs (kann TK sein)"),
                            String::from("20 ml Olivenöl"),
                            String::from("1 TL Salz"),
                            String::from("1 TL Pfeffer"),
                            String::from("1 TL Paprikapulver"),
                            String::from("½ TL Chiliflocken"),
                            String::from("1 EL gehackte Petersilie"),
                            String::from("1 EL gehackter Basilikum"),
                            String::from("1 rote Zwiebel"),
                            String::from("150 g Perlcouscous"),
                            String::from("1 Gurke"),
                            String::from("200 g Tomaten"),
                            String::from("40 ml Olivenöl"),
                            String::from("2 Frühlingszwiebeln"),
                            String::from("1 Knoblauchzehe"),
                            String::from("1 handvoll Basilikum"),
                            String::from("1 handvoll Petersilie"),
                            String::from("Saft einer Zitrone"),
                            String::from("1 EL Tomatenmark"),
                            String::from("200 g Blaubeeren"),
                            String::from("100 g Rucola"),
                            String::from("1 TL Paprikapulver"),
                            String::from("Salz und Pfeffer nach Belieben"),
                            String::from("200 g Feta"),
                        ],
                        description: String::from(
                            "So wird's gemacht\nFür den Lachs Olivenöl, Salt, Pfeffer, Paprikapulver, Chiliflocken, Kräuter und Zwiebel (in Ringe geschnitten) in eine Schale geben und vermengen. Den (angetauten) Lachs mit der Masse marinieren und in eine Ofenform geben. Anstelle von Lachs kann zum Beispiel auch mehr Feta genommen werden und dieser im Ofen gebacken werden mit der gleichen Marinade. Lachs im Backofen bei 180°C (je nach Filetgröße) ca. 15–20 Minuten glasig backen Für den Salat: Den Perlocouscous (alternativ auch einfach Couscous) in doppelt so viel Wasser geben und aufkochen lassen, kurz köcheln lassen bis er aufquellt und weich wird. Etwa 5 Minuten quellen lassen und abgießen.20 ml Olivenöl und Tomatenmark in den Couscous geben und verrühren. Gerne etwas salzen und pfeffern. In eine große Salatschüssel gehackte Tomaten, gehackte Gutken, gehackte Frühlingszwiebel und fein gehackter Knoblauch geben. Basilikum, Petersilie, Zitrone 20 ml Olivenöl mischen und in die Masse geben.Perlcouscous hinzugeben und gut vermengen. Blaubeeren, Rucola hinzugeben und zum Schluss den Feta über den Salat bröseln. Salat in Schüsseln anrichten und den Lachs warm aus dem Ofen dazu servieren.",
                        ),
                        files: vec![
                            String::from(
                                "https://knusperstuebchen.net/wp-content/uploads/2023/08/Perlcouscous-Salat-mit-Ofenlachs-und-Blaubeeren-10.jpg",
                            ),
                            String::from(
                                "https://knusperstuebchen.net/wp-content/uploads/2023/08/Perlcouscous-Salat-mit-Ofenlachs.pdf",
                            ),
                        ],
                    }],
                },
            },
        ]
    }

    #[tokio::test]
    async fn test_get() {
        crate::tests::run();
        let getter = ExternalRecipeGetter;
        assert_expected_gets(getter, expected_gets()).await;
    }
}
