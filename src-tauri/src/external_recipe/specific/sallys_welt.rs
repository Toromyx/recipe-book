//! This module implements [`SpecificExternalRecipeGetterTrait`] for the domain [Sallys Blog](https://sallys-blog.de/).

use std::sync::OnceLock;

use async_trait::async_trait;
use indexmap::IndexSet;
use regex::Regex;
use url::Url;

use crate::{
    external_recipe::{
        client,
        error::ExternalRecipeError,
        specific::{SpecificExternalRecipeGetterTrait, UrlMatch},
        ExternalRecipe, ExternalRecipeStep,
    },
    scraper::{Dom, ParentNode},
};

static PATH_REGEX: OnceLock<Regex> = OnceLock::new();

static IMG_SRC_BLACKLIST: [&str; 1] =
    ["data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7"];

pub struct ExternalRecipeGetter;

#[async_trait]
impl SpecificExternalRecipeGetterTrait for ExternalRecipeGetter {
    async fn get(&self, url: Url) -> Result<ExternalRecipe, ExternalRecipeError> {
        let response = client().get(url.clone()).send().await?;
        let text = response.text().await?;
        let dom = Dom::create(text).await?;
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
            let images_wrap_element = dom.select(".images-wrap").await?.unwrap();
            let mut files = IndexSet::new();
            for img_element in images_wrap_element.select_all("img").await? {
                let src = img_element.get_attribute("src").await?;
                if IMG_SRC_BLACKLIST.contains(&src.as_str()) {
                    continue;
                }
                let src_url_result = Url::options().base_url(Some(&url)).parse(&src);
                let src_url = match src_url_result {
                    Ok(src_url) => src_url,
                    Err(err) => {
                        log::warn!(
                            "Could not parse src attribute \"{}\" as URL for \"{}\": {}",
                            &src,
                            &url,
                            err
                        );
                        continue;
                    }
                };
                files.insert(src_url.to_string());
            }
            steps.push(ExternalRecipeStep {
                ingredients,
                description: step_element.text_content().await?,
                files: files.into_iter().collect(),
            });
        }
        Ok(ExternalRecipe {
            name: dom.select("h1").await?.unwrap().text_content().await?,
            steps,
            ..Default::default()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::external_recipe::tests::{assert_expected_gets, ExpectedGet};

    fn expected_gets() -> Vec<ExpectedGet> {
        vec![ExpectedGet {
            url: "https://sallys-blog.de/rezepte/ofenkebab-mit-kartoffeln".to_string(),
            external_recipe: ExternalRecipe {
                name: "Ofenkebab mit Kartoffeln und Paprika / INTERSPAR #27".to_string(),
                ingredients: vec![],
                files: vec![],
                steps: vec![ExternalRecipeStep {
                    ingredients: vec![
                        "750 g Hackfleisch (Rind)".to_string(),
                        "1  Zwiebel".to_string(),
                        "1  Knoblauchzehe".to_string(),
                        "0,5  Petersilie".to_string(),
                        "1,5 TL Salz".to_string(),
                        "0,5 TL Pfeffer".to_string(),
                        "0,5 TL Chili".to_string(),
                        "1 TL Kreuzkümmel (gemahlen)".to_string(),
                        "150 g Kartoffeln".to_string(),
                        "800 g Kartoffeln".to_string(),
                        "1  Paprika (rot)".to_string(),
                        "1  Paprika (gelb)".to_string(),
                        "1  Paprika (grün)".to_string(),
                        "1  Zwiebel".to_string(),
                        "2  Knoblauchzehen".to_string(),
                        "1 EL Olivenöl".to_string(),
                        "1 EL Tomatenmark".to_string(),
                        "400 g Tomaten (Dose)".to_string(),
                        "100 g Wasser".to_string(),
                        "0,5 TL Paprikapulver  (edelsüß)".to_string(),
                        "1 TL Salz".to_string(),
                        "0,5 TL Pfeffer".to_string(),
                    ],
                    description: "Ofen vorheizenHeize den Ofen auf 200 °C O/U vor.KöfteSchäle die Zwiebeln, Kartoffeln und Knoblauchzehe und reibe sie fein. Hacke die Petersilie klein. Verknete anschließend alle Zutaten miteinander. Teile die Masse mit einem mittleren Eisportionierer ein und forme sie zu Kugeln.Füllung vorbereitenSchäle die Kartoffeln und schneide sie in etwa 5 mm dicke Scheiben. Entkerne die Paprika und schneide sie in Streifen.FertigstellenSchneide die Zwiebeln und die Knoblauchzehen in feine Würfel. Gebe sie zusammen mit den restlichen Zutaten in ein 32 cm Emailleblech und verrühre alles. Gebe die Kartoffelscheiben hinzu und ummantele sie mit der Soße. Nehme etwa ein Drittel der Scheiben wieder heraus und lege sie zur Seite. Lege die Kartoffeln in der Form gleichmäßig auf den Boden. Verteile die Fleischbällchen in der Form und stecke jeweils die zur Seite gelegten Kartoffelscheiben dazwischen. Verteile die Paprikastreifen darüber. Backe das Kebab im vorgeheizten Ofen bei 200 °C O/U für etwa 45 Minuten, bis die Kartoffeln gar sind. Serviere es noch heiß.Viel Spaß beim Nachmachen, eure Sally!- ANZEIGE -Einkaufsliste erstellen?Du möchtest die Zutaten dieses Rezepts als Einkaufsliste abspeichern? Diese und weitere nützlichen Funktionen findest du in meiner App fur Android-und iOS-Geräte:Rezept Drucken?Drucken".to_string(),
                    files: vec![
                "https://sallys-blog.de/_next/image?url=https%3A%2F%2Fimg2.storyblok.com%2F950x650%2Ff%2F130848%2F799x533%2Fe92e306f72%2F1292_19676_ofenkebab_kartoffel_1-jpg.jpg&w=3840&q=75".to_string(),
                "https://sallys-blog.de/_next/image?url=https%3A%2F%2Fimg2.storyblok.com%2F950x650%2Ff%2F130848%2F799x533%2F64416cbc8b%2F1292_19672_ofenkebab_kartoffel_6-jpg.jpg&w=3840&q=75".to_string(),
                "https://sallys-blog.de/_next/image?url=https%3A%2F%2Fimg2.storyblok.com%2F950x650%2Ff%2F130848%2F799x533%2F111ca3bada%2F1292_19677_ofenkebab_kartoffel_5-jpg.jpg&w=3840&q=75".to_string(),
                "https://sallys-blog.de/_next/image?url=https%3A%2F%2Fimg2.storyblok.com%2F950x650%2Ff%2F130848%2F799x533%2F9a9f7c1d76%2F1292_19673_ofenkebab_kartoffel_4-jpg.jpg&w=3840&q=75".to_string(),
                "https://sallys-blog.de/_next/image?url=https%3A%2F%2Fimg2.storyblok.com%2F950x650%2Ff%2F130848%2F799x533%2F9f6ed05c7c%2F1292_19674_ofenkebab_kartoffel_3-jpg.jpg&w=3840&q=75".to_string(),
                "https://sallys-blog.de/_next/image?url=https%3A%2F%2Fimg2.storyblok.com%2F950x650%2Ff%2F130848%2F799x533%2F2db64f46e1%2F1292_19675_ofenkebab_kartoffel_2-jpg.jpg&w=3840&q=75".to_string(),
                    ],
                }],
            },
        }]
    }

    #[tokio::test]
    async fn test_get() {
        crate::tests::run();
        assert_expected_gets(expected_gets()).await;
    }
}
