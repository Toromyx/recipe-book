//! This module implements the creation of [`crate::entity::unit_name`].

use sea_orm_migration::prelude::*;

use crate::migrator::index_name;

pub async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(UnitName::Table)
                .col(
                    ColumnDef::new(UnitName::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(UnitName::Name).string().not_null())
                .col(ColumnDef::new(UnitName::Unit).text().not_null())
                .index(Index::create().col(UnitName::Name).unique())
                .to_owned(),
        )
        .await?;
    manager
        .create_index(
            Index::create()
                .name(&index_name(&UnitName::Table, &UnitName::Unit))
                .table(UnitName::Table)
                .col(UnitName::Unit)
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
    Id,
    Name,
    Unit,
}
