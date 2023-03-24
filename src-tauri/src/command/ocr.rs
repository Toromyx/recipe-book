use tesseract::Tesseract;

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
    std::env::set_var("TESSDATA_PREFIX", tessdata);
    let mut tesseract = Tesseract::new(None, Some("Latin"))?;
    tesseract = tesseract.set_image(&file.to_string_lossy())?;
    let hocr_string = tesseract.get_hocr_text(1)?;
    Ok(hocr_string)
}
