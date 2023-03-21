use crate::{
    command::error::CommandError, entity_crud, entity_crud::EntityCrudTrait, get_app_handle,
    recipe_file_storage,
};

#[tauri::command]
pub async fn ocr(recipe_file_id: i64) -> Result<String, CommandError> {
    let model_option = entity_crud::recipe_file::RecipeFileCrud::read(recipe_file_id).await?;
    let Some(model) = model_option else {
        return Err(CommandError::NotFound);
    };
    let file = recipe_file_storage::file(&model).await?;
    let tessdata = get_app_handle()
        .path_resolver()
        .resolve_resource("tessdata")
        .unwrap()
        .to_string_lossy()
        .to_string();
    std::env::set_var("TESSDATA_PREFIX", &tessdata);
    let ocr_result = tesseract::ocr(&file.to_string_lossy(), "Latin")?;
    Ok(ocr_result)
}
