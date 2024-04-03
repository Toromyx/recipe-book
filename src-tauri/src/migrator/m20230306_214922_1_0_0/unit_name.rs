//! This module implements the creation of [`crate::entity::unit_name`].

use sea_orm_migration::prelude::*;

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(UnitName::Table)
                .col(
                    ColumnDef::new(UnitName::Name)
                        .string()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(UnitName::Unit).text().not_null())
                .to_owned(),
        )
        .await?;
    insert_data(manager).await?;
    Ok(())
}

/// Insert initial known unit names into the table.
async fn insert_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .exec_stmt(
            Query::insert()
                .into_table(UnitName::Table)
                .columns([UnitName::Name, UnitName::Unit])
                .values_panic(["kg".into(), "MassKilogram".into()])
                .values_panic(["kilogram".into(), "MassKilogram".into()])
                .values_panic(["g".into(), "MassGram".into()])
                .values_panic(["gram".into(), "MassGram".into()])
                .values_panic(["lb".into(), "MassPound".into()])
                .values_panic(["pound".into(), "MassPound".into()])
                .values_panic(["pounds".into(), "MassPound".into()])
                .values_panic(["l".into(), "VolumeLitre".into()])
                .values_panic(["litre".into(), "VolumeLitre".into()])
                .values_panic(["ml".into(), "VolumeMillilitre".into()])
                .values_panic(["millilitre".into(), "VolumeMillilitre".into()])
                .values_panic(["cup".into(), "VolumeUsCup".into()])
                .values_panic(["cups".into(), "VolumeUsCup".into()])
                .to_owned(),
        )
        .await?;
    Ok(())
}

#[derive(Iden)]
pub enum UnitName {
    Table,
    Name,
    Unit,
}

#[cfg(test)]
pub mod tests {
    use pretty_assertions::{assert_eq, assert_str_eq};
    use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};

    use crate::database::tests::{get_table_indices, get_table_schema};

    pub async fn assert_unit_name_schema(db: &DatabaseConnection) {
        let table_schema = get_table_schema("unit_name", db).await;
        assert_str_eq!(
            table_schema,
            "CREATE TABLE \"unit_name\" ( \"name\" text NOT NULL PRIMARY KEY, \"unit\" text NOT NULL )"
        );
    }

    pub async fn assert_unit_name_indices(db: &DatabaseConnection) {
        let indices = get_table_indices("unit_name", db).await;
        assert!(indices.is_empty())
    }

    pub async fn assert_unit_name_content(db: &DatabaseConnection) {
        let query_results = db
            .query_all(Statement::from_string(
                db.get_database_backend(),
                "SELECT `name`, `unit` FROM `unit_name` ORDER BY `name`",
            ))
            .await
            .unwrap();
        let data: Vec<(String, String)> = query_results
            .into_iter()
            .map(|query_result| query_result.try_get_many_by_index().unwrap())
            .collect();
        assert_eq!(
            data,
            vec![
                (String::from("cup"), String::from("VolumeUsCup")),
                (String::from("cups"), String::from("VolumeUsCup")),
                (String::from("g"), String::from("MassGram")),
                (String::from("gram"), String::from("MassGram")),
                (String::from("kg"), String::from("MassKilogram")),
                (String::from("kilogram"), String::from("MassKilogram")),
                (String::from("l"), String::from("VolumeLitre")),
                (String::from("lb"), String::from("MassPound")),
                (String::from("litre"), String::from("VolumeLitre")),
                (String::from("millilitre"), String::from("VolumeMillilitre")),
                (String::from("ml"), String::from("VolumeMillilitre")),
                (String::from("pound"), String::from("MassPound")),
                (String::from("pounds"), String::from("MassPound")),
            ]
        )
    }
}
