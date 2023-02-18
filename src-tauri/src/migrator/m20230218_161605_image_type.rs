use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(RecipeStep::Table)
                    .drop_column(RecipeStep::Image)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(RecipeStep::Table)
                    .add_column(ColumnDef::new(RecipeStep::Image).text().null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(RecipeStep::Table)
                    .drop_column(RecipeStep::Image)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(RecipeStep::Table)
                    .add_column(
                        ColumnDef::new(RecipeStep::Image)
                            .blob(BlobSize::Long)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum RecipeStep {
    Table,
    Image,
}
