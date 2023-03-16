use sea_orm::{EntityTrait, QuerySelect};

use crate::{command::error::CommandError, database, entity::recipe_ingredient};

#[tauri::command]
pub async fn unit_list_get() -> Result<Vec<String>, CommandError> {
    let db = database::connect().await;
    let result = recipe_ingredient::Entity::find()
        .select_only()
        .distinct()
        .column(recipe_ingredient::Column::Unit)
        .into_tuple::<String>()
        .all(db)
        .await?;
    Ok(result)
}
