use sea_orm::{
    entity::prelude::*,
    ActiveValue::{Set, Unchanged},
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

impl ActiveModelBehavior for ActiveModel {
    fn after_save(model: Model, insert: bool) -> Result<Model, DbErr> {
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

    fn after_delete(self) -> Result<Self, DbErr> {
        let (Set(id) | Unchanged(id)) = self.id else {
            return Ok(self);
        };
        get_window()
            .emit(ENTITY_ACTION_DELETED_RECIPE, id)
            .map_err(|err| DbErr::Custom(err.to_string()))?;
        Ok(self)
    }
}
