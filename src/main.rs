mod queries;
mod structs;

use crate::queries::*;
use axum::{
  body::Body,
  http::Request,
  middleware::{self, Next},
  response::Response,
  routing::{get, post},
  Extension, Router,
};
use chrono::Local;
use dotenv::dotenv;
use sqlx::MySqlPool;
use std::env;

async fn logging_middleware(req: Request<Body>, next: Next<Body>) -> Response {
  println!("Received a request to {} at: {}", req.uri(), Local::now());
  next.run(req).await
}

#[tokio::main]
async fn main() {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("Can't find database url");
  let pool = MySqlPool::connect(&database_url)
    .await
    .expect("Couldn't connect to the database");
  let server_url = env::var("SERVER_URL").expect("Can't find server url");
  let server_url_display =
    env::var("SERVER_URL_DISPLAY").expect("Can't find server url to display");

  let app = Router::new()
    .route("/", get(|| async { "Gastronomie 9000" }))
    .route("/food", get(food).layer(Extension(pool.clone())))
    .route("/add/food", post(add_food).layer(Extension(pool.clone())))
    .route("/drinks", get(drinks).layer(Extension(pool.clone())))
    .route(
      "/add/drinks",
      post(add_drink).layer(Extension(pool.clone())),
    )
    .route("/items", get(items).layer(Extension(pool.clone())))
    .layer(middleware::from_fn(logging_middleware));

  println!("Running on http://{}", &server_url_display);
  axum::Server::bind(&server_url.parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}
