use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tisch {
    pub id: i64,
    pub seats: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Items {
    #[serde(default)]
    pub item_id: i64,
    pub item_name: String,
    pub item_price: f64,
    pub item_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    #[serde(default)]
    pub order_id: i64,
    pub order_timestamp: DateTime<Local>,
    pub total_amount: f64,
    #[serde(default)]
    pub order_paid: bool,
    #[serde(default)]
    pub pay_timestamp: Option<DateTime<Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderItem {
    #[serde(default)]
    pub orderitem_id: i64,
    pub table_id: i64,
    pub order_id: i64,
    pub item_id: i64,
    pub orderitem_quantity: i64,
}
