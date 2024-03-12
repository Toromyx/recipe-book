use tauri::Manager;

use crate::{
    command::error::{CommandError, CommandError::NotFound},
    entity::file::Model,
    entity_crud::{
        file::{FileCondition, FileCreate, FileCrud, FileFilter, FileUpdate},
        EntityCrudTrait,
    },
};

#[tauri::command]
pub async fn entity_create_file(create: FileCreate) -> Result<i64, CommandError> {
    let id = FileCrud::create(create).await?;
    Ok(id)
}

#[tauri::command]
pub async fn entity_read_file(id: i64, window: tauri::Window) -> Result<Model, CommandError> {
    let model_option = FileCrud::read(id).await?;
    let model = model_option.ok_or(NotFound)?;
    window.asset_protocol_scope().allow_file(&model.path)?;
    Ok(model)
}

#[tauri::command]
pub async fn entity_update_file(update: FileUpdate) -> Result<(), CommandError> {
    FileCrud::update(update).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_delete_file(id: i64) -> Result<(), CommandError> {
    FileCrud::delete(id).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_list_file(filter: FileFilter) -> Result<Vec<i64>, CommandError> {
    let list = FileCrud::list(filter).await?;
    Ok(list)
}

#[tauri::command]
pub async fn entity_count_file(condition: Option<FileCondition>) -> Result<i64, CommandError> {
    let count = FileCrud::count(condition).await?;
    Ok(count)
}
