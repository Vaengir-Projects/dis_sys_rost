use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tisch {
  pub id: i64,
  pub seats: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Food {
  #[serde(default)]
  pub food_id: i64,
  pub food_name: String,
  pub food_price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Drinks {
  #[serde(default)]
  pub drink_id: i64,
  pub drink_name: String,
  pub drink_price: f64,
  pub drink_size: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Items {
  #[serde(default)]
  item_id: String,
  item_type: String,
  item_name: String,
  item_price: f64,
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
  pub order_id: i64,
  pub item_id: String,
  pub orderitem_quantity: i64,
}
