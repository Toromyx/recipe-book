use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(RecipeIngredient::Table)
                    .rename_column(RecipeIngredient::Quantity, RecipeIngredient::QuantityOld)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(RecipeIngredient::Table)
                    .add_column(ColumnDef::new(RecipeIngredient::Quantity).double().null())
                    .to_owned(),
            )
            .await?;
        manager
            .exec_stmt(
                UpdateStatement::new()
                    .table(RecipeIngredient::Table)
                    .value(
                        RecipeIngredient::Quantity,
                        Expr::col(RecipeIngredient::QuantityOld),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(RecipeIngredient::Table)
                    .drop_column(RecipeIngredient::QuantityOld)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(RecipeIngredient::Table)
                    .rename_column(RecipeIngredient::Unit, RecipeIngredient::UnitOld)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(RecipeIngredient::Table)
                    .add_column(ColumnDef::new(RecipeIngredient::Unit).string().null())
                    .to_owned(),
            )
            .await?;
        manager
            .exec_stmt(
                UpdateStatement::new()
                    .table(RecipeIngredient::Table)
                    .value(RecipeIngredient::Unit, Expr::col(RecipeIngredient::UnitOld))
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(RecipeIngredient::Table)
                    .drop_column(RecipeIngredient::UnitOld)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum RecipeIngredient {
    Table,
    Quantity,
    QuantityOld,
    Unit,
    UnitOld,
}
