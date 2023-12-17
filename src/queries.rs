use crate::structs::*;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::MySqlPool;

pub async fn food(Extension(pool): Extension<MySqlPool>) -> impl IntoResponse {
  let query = "Select food_id, food_name, food_price from food".to_string();
  let food: Vec<Food> = match sqlx::query_as(&query).fetch_all(&pool).await {
    Ok(food) => food,
    Err(e) => {
      return (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal server error: {}", e),
      )
        .into_response()
    }
  };

  (StatusCode::OK, Json(food)).into_response()
}

pub async fn add_food(
  Extension(pool): Extension<MySqlPool>,
  Json(data): Json<Food>,
) -> impl IntoResponse {
  println!("{:?}", data);
  let query = format!(
    "Insert into food (food_name, food_price) values ('{}', {})",
    data.food_name, data.food_price
  );
  let rows_affected = match sqlx::query(&query).execute(&pool).await {
    Ok(rows_affected) => rows_affected,
    Err(e) => {
      return (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal server error: {}", e),
      )
        .into_response()
    }
  };

  (
    StatusCode::OK,
    format!("Number of rows affected: {:?}", rows_affected),
  )
    .into_response()
}

pub async fn drinks(Extension(pool): Extension<MySqlPool>) -> impl IntoResponse {
  let query = "Select drink_id, drink_name, drink_price, drink_size from drinks".to_string();
  let drinks: Vec<Drinks> = match sqlx::query_as(&query).fetch_all(&pool).await {
    Ok(drinks) => drinks,
    Err(e) => {
      return (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal server error: {}", e),
      )
        .into_response()
    }
  };

  (StatusCode::OK, Json(drinks)).into_response()
}

pub async fn add_drink(
  Extension(pool): Extension<MySqlPool>,
  Json(data): Json<Drinks>,
) -> impl IntoResponse {
  println!("{:?}", data);
  let query = format!(
    "Insert into drinks (drink_name, drink_price, drink_size) values ('{}', {}, {})",
    data.drink_name, data.drink_price, data.drink_size
  );
  let rows_affected = match sqlx::query(&query).execute(&pool).await {
    Ok(rows_affected) => rows_affected,
    Err(e) => {
      return (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal server error: {}", e),
      )
        .into_response()
    }
  };

  (
    StatusCode::OK,
    format!("Number of rows affected: {:?}", rows_affected),
  )
    .into_response()
}
