use sea_orm::{sea_query, FromQueryResult};
use serde::Deserialize;

pub mod error;
pub mod ingredient;
pub mod recipe;
pub mod recipe_file;
pub mod recipe_ingredient;
pub mod recipe_step;

#[derive(FromQueryResult)]
pub struct IdColumn {
    id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Order {
    Asc,
    Desc,
}

impl Default for Order {
    fn default() -> Self {
        Self::Asc
    }
}

impl From<Order> for sea_query::Order {
    fn from(value: Order) -> Self {
        match value {
            Order::Asc => sea_query::Order::Asc,
            Order::Desc => sea_query::Order::Desc,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderByItem<OrderByColumn> {
    column: OrderByColumn,
    #[serde(default)]
    order: Order,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter<Condition, OrderBy> {
    pub condition: Option<Condition>,
    pub order_by: Option<Vec<OrderByItem<OrderBy>>>,
}

fn get_order_by<OrderBy, Column: From<OrderBy>>(
    order_by: Option<Vec<OrderByItem<OrderBy>>>,
) -> Vec<(Column, sea_query::Order)> {
    let Some(order_by) = order_by else {
        return vec![];
    };
    order_by
        .into_iter()
        .map(|item| {
            (
                Column::from(item.column),
                sea_query::Order::from(item.order),
            )
        })
        .collect()
}
