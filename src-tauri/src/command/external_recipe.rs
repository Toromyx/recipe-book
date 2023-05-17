use crate::{command::error::CommandError, external_recipe::ExternalRecipeData};

#[tauri::command]
pub async fn external_recipe(url: String) -> Result<ExternalRecipeData, CommandError> {
    let data = crate::external_recipe::get(url).await?;
    Ok(data)
}
