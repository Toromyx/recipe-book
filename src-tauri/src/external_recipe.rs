//! This module handles getting data from external recipes in the world wide web.

use std::{str::FromStr, sync::OnceLock, time::Duration};

use rdf_types::{Id, Object};
use reqwest::Client;
use schema_org_constants::{
    HOW_TO_SECTION_IRI_HTTP, HOW_TO_SECTION_IRI_HTTPS, HOW_TO_STEP_IRI_HTTP, HOW_TO_STEP_IRI_HTTPS,
};
use schema_org_traits::{
    json_ld_0_15::JsonLdStore, FindRecipeIds, GetContentUrlProperty, GetImageProperty,
    GetItemListElementProperty, GetNameProperty, GetRecipeIngredientProperty,
    GetRecipeInstructionsProperty, GetTextProperty, GetVideoProperty,
};
use url::Url;

use crate::external_recipe::{
    error::ExternalRecipeError, generic::generic_external_recipe_getters,
    specific::specific_external_recipe_getters,
};

pub mod error;
mod generic;
mod specific;

static CLIENT_ONCE_LOCK: OnceLock<Client> = OnceLock::new();

fn client() -> &'static Client {
    CLIENT_ONCE_LOCK.get_or_init(|| {
        reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap()
    })
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ExternalRecipe {
    pub name: String,
    pub ingredients: Vec<String>,
    pub files: Vec<String>,
    pub steps: Vec<ExternalRecipeStep>,
}

impl ExternalRecipe {
    /// Try to create an [`ExternalRecipe`] via [`FindRecipeIds`] inside a [`JsonLdStore`].
    ///
    /// Returns [`None`] if no subject of type [`schema_org_constants::RECIPE_IRI_HTTP`] can be found.
    pub fn try_from_json_ld(json_ld_store: &JsonLdStore) -> Option<Self> {
        let recipe_id_option = json_ld_store.find_recipe_ids().first().copied();
        let recipe_id = recipe_id_option?;
        Some(Self::from_schema_org_recipe_json_ld(
            json_ld_store,
            recipe_id,
        ))
    }

    /// Create an [`ExternalRecipe`] from with a given [`schema_org_constants::RECIPE_IRI_HTTP`] id inside a [`JsonLdStore`].
    ///
    /// This roughly implements the logic outlined in <https://developers.google.com/search/docs/appearance/structured-data/recipe>,
    /// but also changes it where necessary.
    fn from_schema_org_recipe_json_ld(json_ld_store: &JsonLdStore, recipe_id: &Id) -> Self {
        let name = json_ld_store
            .get_name_property(recipe_id)
            .first()
            .copied()
            .and_then(|object| object.as_literal())
            .map(|literal| literal.as_str().to_string())
            .unwrap_or_default();
        let ingredients = json_ld_store
            .get_recipe_ingredient_property(recipe_id)
            .into_iter()
            .filter_map(|object| object.as_literal())
            .map(|literal| literal.as_str().to_string())
            .collect();
        let files = get_file_urls(json_ld_store, recipe_id);
        let steps = json_ld_store
            .get_recipe_instructions_property(recipe_id)
            .into_iter()
            .flat_map(
                |recipe_instructions_property| match recipe_instructions_property {
                    Object::Id(id) => {
                        let Some(object_type_str) =
                            (|| Some(json_ld_store.get_type(id)?.as_id()?.as_str()))()
                        else {
                            return vec![];
                        };
                        match object_type_str {
                            HOW_TO_STEP_IRI_HTTP | HOW_TO_STEP_IRI_HTTPS => {
                                vec![ExternalRecipeStep::from_schema_org_how_to_step_json_ld(
                                    json_ld_store,
                                    id,
                                )]
                            }
                            HOW_TO_SECTION_IRI_HTTP | HOW_TO_SECTION_IRI_HTTPS => json_ld_store
                                .get_item_list_element_property(id)
                                .into_iter()
                                .filter_map(|object| object.as_id())
                                .map(|id| {
                                    ExternalRecipeStep::from_schema_org_how_to_step_json_ld(
                                        json_ld_store,
                                        id,
                                    )
                                })
                                .collect(),
                            _ => vec![],
                        }
                    }
                    Object::Literal(literal) => vec![ExternalRecipeStep {
                        description: literal.as_str().to_string(),
                        ..Default::default()
                    }],
                },
            )
            .collect();

        Self {
            name,
            ingredients,
            files,
            steps,
        }
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ExternalRecipeStep {
    pub ingredients: Vec<String>,
    pub description: String,
    pub files: Vec<String>,
}

impl ExternalRecipeStep {
    /// Create an [`ExternalRecipeStep`] from with a given [`schema_org_constants::HOW_TO_STEP_IRI_HTTP`] id inside a [`JsonLdStore`].
    ///
    /// This roughly implements the logic outlined in <https://developers.google.com/search/docs/appearance/structured-data/recipe#how-to-step>,
    /// but also changes it where necessary.
    fn from_schema_org_how_to_step_json_ld(
        json_ld_store: &JsonLdStore,
        how_to_step_id: &Id,
    ) -> Self {
        let description = json_ld_store
            .get_text_property(how_to_step_id)
            .into_iter()
            .filter_map(|object| object.as_literal())
            .map(|literal| literal.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        let files = get_file_urls(json_ld_store, how_to_step_id);
        ExternalRecipeStep {
            description,
            files,
            ..Default::default()
        }
    }
}

/// Get images and videos from the [`schema_org_constants::IMAGE_PROPERTY_IRI_HTTP`] and [`schema_org_constants::VIDEO_PROPERTY_IRI_HTTP`] of a specified id in a [`JsonLdStore`].
///
/// This function tries to return the publicly available URLs of these objects:
/// - the string directly if a [`Object::Literal`]
/// - the [`schema_org_constants::CONTENT_URL_PROPERTY_IRI_HTTP`] if present in the [`JsonLdStore`]
/// - or the IRI directly as URL.
fn get_file_urls(json_ld_store: &JsonLdStore, id: &Id) -> Vec<String> {
    let get_content_url_of_object = |object: &Object| -> Option<String> {
        Some(
            match object {
                Object::Id(id) => {
                    if let Some(content_url_property) =
                        json_ld_store.get_content_url_property(id).first()
                    {
                        content_url_property.as_literal()?.as_str()
                    } else {
                        id.as_str()
                    }
                }
                Object::Literal(literal) => literal.as_str(),
            }
            .to_string(),
        )
    };

    let image_urls_iter = json_ld_store
        .get_image_property(id)
        .into_iter()
        .filter_map(get_content_url_of_object);
    let video_urls_iter = json_ld_store
        .get_video_property(id)
        .into_iter()
        .filter_map(get_content_url_of_object);
    image_urls_iter.chain(video_urls_iter).collect()
}

/// Get an external recipe from a URL.
pub async fn get(url_string: String) -> Result<ExternalRecipe, ExternalRecipeError> {
    let url = Url::from_str(&url_string).map_err(anyhow::Error::from)?;
    let external_recipe_getter_option = specific_external_recipe_getters()
        .into_iter()
        .find(|external_recipe_getter| external_recipe_getter.can_get(&url));
    let external_recipe_option = match external_recipe_getter_option {
        None => {
            let getters = generic_external_recipe_getters();
            let mut getter_iterator = getters.iter();
            let mut getter_option = getter_iterator.next();
            loop {
                match getter_option {
                    Some(getter) => {
                        if let Some(external_recipe) = getter.get(url.clone()).await? {
                            break Some(external_recipe);
                        }
                    }
                    None => {
                        break None;
                    }
                }
                getter_option = getter_iterator.next();
            }
        }
        Some(getter) => Some(getter.get(url).await?),
    };
    let Some(external_recipe) = external_recipe_option else {
        return Err(ExternalRecipeError::UrlNotSupported(url_string));
    };
    Ok(external_recipe)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use url::Url;

    use super::*;
    use crate::external_recipe::{
        generic::GenericExternalRecipeGetterTrait, specific::SpecificExternalRecipeGetterTrait,
    };

    #[async_trait::async_trait]
    impl<T> GenericExternalRecipeGetterTrait for T
    where
        T: SpecificExternalRecipeGetterTrait,
    {
        async fn get(&self, url: Url) -> Result<Option<ExternalRecipe>, ExternalRecipeError> {
            Ok(Some(self.get(url).await?))
        }
    }

    #[derive(Debug, Clone)]
    pub struct ExpectedGet {
        pub url: String,
        pub external_recipe: ExternalRecipe,
    }

    pub async fn assert_expected_gets(expected_gets: Vec<ExpectedGet>) {
        for expected_get in expected_gets {
            let actual = get(expected_get.url).await.unwrap();
            assert_eq!(actual, expected_get.external_recipe);
        }
    }
}
