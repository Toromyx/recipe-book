use sea_orm::{
    sea_query::{Expr, Query, UnionType},
    ConnectionTrait, DeriveIden, EntityName, EnumIter, TryGetableMany,
};

use crate::{
    command::error::CommandError,
    database,
    entity::{recipe_ingredient, unit_name},
};

#[derive(EnumIter, DeriveIden)]
enum ResultColumn {
    Unit,
}

/// Get the unit names currently in use.
///
/// This includes values inside [`recipe_ingredient::Column::Unit`] and [`unit_name::Column::Name`].
#[tauri::command]
pub async fn unit_list_get() -> Result<Vec<String>, CommandError> {
    let db = database::connect().await;
    let query = Query::select()
        .column(recipe_ingredient::Column::Unit)
        .distinct()
        .from(recipe_ingredient::Entity.table_ref())
        .and_where(Expr::col(recipe_ingredient::Column::Unit).is_not_null())
        .union(
            UnionType::Distinct,
            Query::select()
                .column(unit_name::Column::Name)
                .from(unit_name::Entity.table_ref())
                .to_owned(),
        )
        .to_owned();
    let query = db.get_database_backend().build(&query);
    let result = <String>::find_by_statement::<ResultColumn>(query)
        .all(db)
        .await?;
    Ok(result)
}
