use sea_orm::{sea_query::Expr, EntityTrait, QueryFilter, QuerySelect};

use crate::{command::error::CommandError, database, entity::recipe_ingredient};

/// Get the unit names currently in use.
#[tauri::command]
pub async fn unit_list_get() -> Result<Vec<String>, CommandError> {
    let db = database::connect().await;
    let result = recipe_ingredient::Entity::find()
        .select_only()
        .distinct()
        .filter(Expr::col(recipe_ingredient::Column::Unit).is_not_null())
        .column(recipe_ingredient::Column::Unit)
        .into_tuple::<String>()
        .all(db)
        .await?;
    Ok(result)
}
