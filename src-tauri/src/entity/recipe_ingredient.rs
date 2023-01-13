use sea_orm::{
    entity::prelude::*,
    ActiveValue::{Set, Unchanged},
};
use serde::Serialize;

use crate::{
    event::channel::{
        ENTITY_ACTION_CREATED_RECIPE_INGREDIENT, ENTITY_ACTION_DELETED_RECIPE_INGREDIENT,
        ENTITY_ACTION_UPDATED_RECIPE_INGREDIENT,
    },
    window::get_window,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "recipe_ingredient")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub order: i64,
    pub quantity: f64,
    pub unit: String,
    pub recipe_step_id: i64,
    pub ingredient_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ingredient::Entity",
        from = "Column::IngredientId",
        to = "super::ingredient::Column::Id",
        on_update = "NoAction",
        on_delete = "Restrict"
    )]
    Ingredient,
    #[sea_orm(
        belongs_to = "super::recipe_step::Entity",
        from = "Column::RecipeStepId",
        to = "super::recipe_step::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    RecipeStep,
}

impl Related<super::ingredient::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ingredient.def()
    }
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
                .emit(ENTITY_ACTION_CREATED_RECIPE_INGREDIENT, ())
                .map_err(|err| DbErr::Custom(err.to_string()))?;
        } else {
            get_window()
                .emit(ENTITY_ACTION_UPDATED_RECIPE_INGREDIENT, model.id)
                .map_err(|err| DbErr::Custom(err.to_string()))?;
        }
        Ok(model)
    }

    fn after_delete(self) -> Result<Self, DbErr> {
        let (Set(id) | Unchanged(id)) = self.id else {
            return Ok(self);
        };
        get_window()
            .emit(ENTITY_ACTION_DELETED_RECIPE_INGREDIENT, id)
            .map_err(|err| DbErr::Custom(err.to_string()))?;
        Ok(self)
    }
}
