use sea_orm::{
    entity::prelude::*,
    prelude::async_trait::async_trait,
    ActiveValue::{NotSet, Set, Unchanged},
};
use serde::Serialize;

use crate::{
    event::channel::{
        ENTITY_ACTION_CREATED_RECIPE, ENTITY_ACTION_DELETED_RECIPE, ENTITY_ACTION_UPDATED_RECIPE,
    },
    window::get_window,
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "recipe")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::recipe_step::Entity")]
    RecipeStep,
}

impl Related<super::recipe_step::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecipeStep.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn after_save<C>(model: Model, _db: &C, insert: bool) -> Result<Model, DbErr> {
        if insert {
            get_window()
                .emit(ENTITY_ACTION_CREATED_RECIPE, ())
                .map_err(|err| DbErr::Custom(err.to_string()))?;
        } else {
            get_window()
                .emit(ENTITY_ACTION_UPDATED_RECIPE, model.id)
                .map_err(|err| DbErr::Custom(err.to_string()))?;
        }
        Ok(model)
    }

    async fn after_delete<C>(self, _db: &C) -> Result<Self, DbErr> {
        let id = match self.id {
            Set(id) | Unchanged(id) => id,
            NotSet => return Ok(self),
        };
        get_window()
            .emit(ENTITY_ACTION_DELETED_RECIPE, id)
            .map_err(|err| DbErr::Custom(err.to_string()))?;
        Ok(self)
    }
}
