use async_trait::async_trait;
use url::Url;

use crate::external_recipe::{error::ExternalRecipeError, ExternalRecipe};

mod yoast_schema_graph;

/// Implementors implement the getting itself.
#[async_trait]
pub trait GenericExternalRecipeGetterTrait: Send + Sync {
    /// Get the external recipe from the URL.
    async fn get(&self, url: Url) -> Result<Option<ExternalRecipe>, ExternalRecipeError>;
}

pub fn generic_external_recipe_getters() -> Vec<Box<dyn GenericExternalRecipeGetterTrait>> {
    vec![Box::new(yoast_schema_graph::ExternalRecipeGetter)]
}
