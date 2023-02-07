use sea_orm::{
    entity::prelude::*,
    prelude::async_trait::async_trait,
    ActiveValue::{NotSet, Set, Unchanged},
};
use serde::Serialize;

use crate::{
    event::channel::{
        ENTITY_ACTION_CREATED_INGREDIENT, ENTITY_ACTION_DELETED_INGREDIENT,
        ENTITY_ACTION_UPDATED_INGREDIENT,
    },
    window::get_window,
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "ingredient")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::recipe_ingredient::Entity")]
    RecipeIngredient,
}

impl Related<super::recipe_ingredient::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecipeIngredient.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn after_save<C>(model: Model, _db: &C, insert: bool) -> Result<Model, DbErr> {
        if insert {
            get_window()
                .emit(ENTITY_ACTION_CREATED_INGREDIENT, ())
                .map_err(|err| DbErr::Custom(err.to_string()))?;
        } else {
            get_window()
                .emit(ENTITY_ACTION_UPDATED_INGREDIENT, model.id)
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
            .emit(ENTITY_ACTION_DELETED_INGREDIENT, id)
            .map_err(|err| DbErr::Custom(err.to_string()))?;
        Ok(self)
    }
}
