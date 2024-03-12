use std::path::PathBuf;

use entity_crud::file::FileCrud;
use tesseract::Tesseract;

use crate::{
    app_handle::get_app_handle, command::error::CommandError, entity_crud,
    entity_crud::EntityCrudTrait,
};

/// Get the optically recognized characters from the specified recipe step file.
#[tauri::command]
pub async fn ocr(file_id: i64) -> Result<String, CommandError> {
    let model_option = FileCrud::read(file_id).await?;
    let Some(model) = model_option else {
        return Err(CommandError::NotFound);
    };
    let file = PathBuf::from(&model.path);
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
