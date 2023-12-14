use crate::structs::*;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::MySqlPool;

pub async fn get_all_food(Extension(pool): Extension<MySqlPool>) -> impl IntoResponse {
  let query = "Select id, name, price, vegetarian, vegan from food".to_string();
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
    "Insert into food (name, price, vegetarian, vegan) values ('{}', {}, {}, {})",
    data.name, data.price, data.vegetarian, data.vegan
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

pub async fn get_all_drinks(Extension(pool): Extension<MySqlPool>) -> impl IntoResponse {
  let query = "Select id, name, price, size from drinks".to_string();
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
    "Insert into drinks (name, price, size) values ('{}', {}, {})",
    data.name, data.price, data.size
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
