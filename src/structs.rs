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
  pub id: i64,
  pub name: String,
  pub price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Drinks {
  #[serde(default)]
  pub id: i64,
  pub name: String,
  pub price: f64,
  pub size: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
  #[serde(default)]
  pub id: i64,
  pub order_timestamp: DateTime<Local>,
  pub total_amount: f64,
  #[serde(default)]
  pub paid: bool,
  #[serde(default)]
  pub pay_timestamp: Option<DateTime<Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderItem {
  #[serde(default)]
  pub id: i64,
  pub order_id: i64,
  pub item_id: String,
  pub quantity: i64,
}
