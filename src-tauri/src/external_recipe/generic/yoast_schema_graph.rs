//! This module implements [`GenericExternalRecipeGetterTrait`] for a [Yoast schema graph](https://developer.yoast.com/features/schema/).

use async_trait::async_trait;
use json_ld::{
    syntax::{Parse, Value},
    RemoteDocument,
};
use schema_org_traits::json_ld_0_15::JsonLdStore;
use url::Url;

use crate::{
    external_recipe::{
        client, error::ExternalRecipeError, generic::GenericExternalRecipeGetterTrait,
        ExternalRecipe,
    },
    scraper::{Dom, ParentNode},
};

pub struct ExternalRecipeGetter;

async fn read_yoast(dom: &Dom) -> Result<Option<String>, ExternalRecipeError> {
    let yoast_schema_graph_element_option = dom.select("script.yoast-schema-graph").await?;
    let Some(yoast_schema_graph_element) = yoast_schema_graph_element_option else {
        return Ok(None);
    };
    Ok(Some(yoast_schema_graph_element.text_content().await?))
}

async fn parse_yoast(schema_graph_text: String) -> Option<ExternalRecipe> {
    let input = RemoteDocument::new(
        None,
        Some("application/ld+json".parse().unwrap()),
        Value::parse_str(&schema_graph_text, |_| ()).unwrap(),
    );
    let mut loader = json_ld::ReqwestLoader::new_with_metadata_map(|_, _, _| ());
    let json_ld_store = JsonLdStore::new(input, &mut loader, None).await;
    ExternalRecipe::try_from_json_ld(&json_ld_store)
}

#[async_trait]
impl GenericExternalRecipeGetterTrait for ExternalRecipeGetter {
    async fn get(&self, url: Url) -> Result<Option<ExternalRecipe>, ExternalRecipeError> {
        let response = client().get(url).send().await?;
        let text = response.text().await?;
        let dom = Dom::create(text).await?;
        let yoast_schema_graph_option = read_yoast(&dom).await?;
        let Some(yoast_schema_graph) = yoast_schema_graph_option else {
            return Ok(None);
        };
        let external_recipe_option = parse_yoast(yoast_schema_graph).await;
        let Some(external_recipe) = external_recipe_option else {
            return Ok(None);
        };
        Ok(Some(external_recipe))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::external_recipe::{
        tests::{assert_expected_gets, ExpectedGet},
        ExternalRecipeStep,
    };

    fn expected_gets() -> Vec<ExpectedGet> {
        vec![
            ExpectedGet {
                url: String::from("https://instakoch.de/gyros-mit-reis-und-salat/"),
                external_recipe: ExternalRecipe {
                    name: "Gyros mit Reis und Salat".to_string(),
                    ingredients: vec![
                        "500 g Putenbrust (Oder Hähnchenbrust)".to_string(),
                        "1 EL Gyrosgewürz".to_string(),
                        "Salz und Pfeffer (nach Geschmack)".to_string(),
                        "1 TL Zitronensaft".to_string(),
                        "1 TL Zucker".to_string(),
                        "4 EL Öl".to_string(),
                        "1 Becher Schmand (200 Gramm)".to_string(),
                        "100 ml Rinderbrühe".to_string(),
                        "200 g Reis".to_string(),
                        "1 TL Salz".to_string(),
                        "2 EL Weißwein Essig".to_string(),
                        "6 EL Öl".to_string(),
                        "1 TL Senf".to_string(),
                        "2 TL Honig".to_string(),
                        "1 TL Salz".to_string(),
                        "½ TL Schwarzer Pfeffer".to_string(),
                        "½ Kopf Eisberg Salat".to_string(),
                        "Etwas  Schnittlauch (Zum Garnieren.)".to_string(),
                    ],
                    files: vec![
                        "https://instakoch.de/wp-content/uploads/2019/01/IMG_20190121_185416_824-1-e1549394421544.jpg".to_string(),
                        "https://instakoch.de/wp-content/uploads/2019/01/IMG_20190121_185416_824-1-e1549394421544-500x500.jpg".to_string(),
                        "https://instakoch.de/wp-content/uploads/2019/01/IMG_20190121_185416_824-1-e1549394421544-500x375.jpg".to_string(),
                        "https://instakoch.de/wp-content/uploads/2019/01/IMG_20190121_185416_824-1-e1549394421544-480x270.jpg".to_string(),
                    ],
                    steps: vec![
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Das Fleisch in dünne Streifen schneiden.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "In einer Schüssel das Öl mit dem Gyros Gewürz, Salz und Pfeffer, sowie 1 TL Zucker und 1 TL Zitronensaft zu einer Marinade verrühren.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Das Fleisch hinzugeben, alles gut vermischen und abgedeckt mindestens 30 Minuten lang im Kühlschrank marinieren.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Tipp: Du kannst auch bereits mariniertes Gyros verwenden, dann entfällt dieser Schritt".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Zunächst den Reis kochen. Ich koche Reis gerne im Reiskocher. Das geht schnell, macht keinen Dreck und gelingt immer, ohne anzubrennen. Einfach 1 Teil Reis mit 1,5 Teilen Wasser und einer Prise Salz in den Reiskocher geben und das Reis-Programm starten.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Währenddessen das Fleisch in einer vorgeheizten Pfanne ohne zusätzliches Öl scharf anbraten.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Wenn es rundum schön braun ist, die Hitze reduzieren und den Schmand zusammen mit der Rinderbrühe zugeben.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "So lange auf geringer Hitze köcheln, bis eine sämige Konsistenz erreicht ist. Mit Salz und Pfeffer nochmal abschmecken.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "2 El Weißwein Essig mit 6 El Speiseöl vermischen. Den Senf, sowie Honig, Zitronensaft, Salz und Pfeffer hinzufügen und alles mit einem Schneebesen verrühren.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Nochmals mit Salz und Pfeffer abschmecken.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Den Salat klein schneiden, waschen und trocken schleudern.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Kurz vor dem Anrichten mit dem Salat-Dressing gut vermischen.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Tipp: Wenn du möchtest, kannst du auch noch Zwiebelringe, Gurkenscheiben und Tomaten mit in den Salat geben.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Gyros auf dem Reis anrichten, den Salat dazugeben. Mit Schnittlauch garnieren und servieren.".to_string(),
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
            ExpectedGet {
                url: "https://www.madamecuisine.de/jaegertorte-mit-preiselbeeren/".to_string(),
                external_recipe: ExternalRecipe {
                    name: "Jägertorte mit Preiselbeeren".to_string(),
                    ingredients: vec![
                        "100 g weiche Butter".to_string(),
                        "100 g Zucker".to_string(),
                        "3  Eier".to_string(),
                        "125 g gemahlene Haselnüsse oder Mandeln".to_string(),
                        "2 TL Backpulver".to_string(),
                        "2 EL dunkles Kakaopulver".to_string(),
                        "250 ml Sahne".to_string(),
                        "1 Päckchen Sahnesteif".to_string(),
                        "1 Glas Preiselbeeren (220 g)".to_string(),
                    ],
                    files: vec![
                        "https://www.madamecuisine.de/wp-content/uploads/2021/05/jaegertorte-featured.jpg".to_string(),
                        "https://www.madamecuisine.de/wp-content/uploads/2021/05/jaegertorte-featured-500x500.jpg".to_string(),
                        "https://www.madamecuisine.de/wp-content/uploads/2021/05/jaegertorte-featured-500x375.jpg".to_string(),
                        "https://www.madamecuisine.de/wp-content/uploads/2021/05/jaegertorte-featured-480x270.jpg".to_string(),
                    ],
                    steps: vec![
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Die Eier trennen, das Eiweiß sehr steif schlagen. Die Eigelbe mit der Butter und dem Zucker schaumig rühren.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Gemahlene Haselnüsse oder Mandeln mit dem Backpulver und dem Kakao vermengen und unter die Eigelb-Butter-Mischung rühren. Steif geschlagenes Eiweiß mit dem Schneebesen vorsichtig unterheben.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Teig in eine gefettete Springform füllen und im vorgeheizten Ofen bei 180 Grad Ober-/Unterhitze etwa 35 Minuten backen. Anschließend gut auskühlen lassen.".to_string(),
                            files: vec![],
                        },
                        ExternalRecipeStep {
                            ingredients: vec![],
                            description: "Die Sahne mit dem Sahnesteif schlagen bis sie fest ist. Preiselbeeren unterrühren und die Masse auf dem Tortenboden verteilen.".to_string(),
                            files: vec![],
                        },
                    ],
                },
            },
        ]
    }

    #[tokio::test]
    async fn test_get() {
        crate::tests::run();
        assert_expected_gets(expected_gets()).await;
    }
}
