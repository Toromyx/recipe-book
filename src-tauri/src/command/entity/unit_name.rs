use crate::{
    command::error::{CommandError, CommandError::NotFound},
    entity::unit_name::Model,
    entity_crud::{
        unit_name::{UnitNameCreate, UnitNameCrud, UnitNameFilter, UnitNameUpdate},
        EntityCrudTrait,
    },
};

#[tauri::command]
pub async fn entity_create_unit_name(create: UnitNameCreate) -> Result<i64, CommandError> {
    let id = UnitNameCrud::create(create).await?;
    Ok(id)
}

#[tauri::command]
pub async fn entity_read_unit_name(id: i64) -> Result<Model, CommandError> {
    let model_option = UnitNameCrud::read(id).await?;
    let model = model_option.ok_or(NotFound)?;
    Ok(model)
}

#[tauri::command]
pub async fn entity_update_unit_name(update: UnitNameUpdate) -> Result<(), CommandError> {
    UnitNameCrud::update(update).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_delete_unit_name(id: i64) -> Result<(), CommandError> {
    UnitNameCrud::delete(id).await?;
    Ok(())
}

#[tauri::command]
pub async fn entity_list_unit_name(filter: UnitNameFilter) -> Result<Vec<i64>, CommandError> {
    let list = UnitNameCrud::list(filter).await?;
    Ok(list)
}

#[tauri::command]
pub async fn entity_count_unit_name(filter: UnitNameFilter) -> Result<i64, CommandError> {
    let count = UnitNameCrud::count(filter).await?;
    Ok(count)
}
