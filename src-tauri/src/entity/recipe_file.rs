use sea_orm::{entity::prelude::*, prelude::async_trait::async_trait};
use serde::Serialize;

use crate::recipe_file_storage;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "recipe_file")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub order: i64,
    pub mime: String,
    pub path: String,
    pub recipe_step_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::recipe_step::Entity",
        from = "Column::RecipeStepId",
        to = "super::recipe_step::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    RecipeStep,
}

impl Related<super::recipe_step::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecipeStep.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_delete<C>(self, _db: &C) -> Result<Self, DbErr> {
        let model = Model::try_from(self.clone())?;
        recipe_file_storage::delete(&model).await.map_err(|err| {
            DbErr::Custom(format!("Could not delete recipe file on disk: {}", err))
        })?;
        Ok(self)
    }
}
