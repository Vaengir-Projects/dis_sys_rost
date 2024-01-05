mod queries;
mod structs;

use crate::queries::*;
use axum::{
    body::Body,
    http::{Method, Request},
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Extension, Router,
};
use chrono::Local;
use dotenv::dotenv;
use sqlx::MySqlPool;
use std::{env, fs::OpenOptions, io::Write};

async fn logging_middleware(req: Request<Body>, next: Next<Body>) -> Response {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("/tmp/dis_sys_log.txt")
        .expect("Couldn't access Logfile");
    let message = match req.method() {
        &Method::GET => format!(
            "GET: Received a request to {} at: {}",
            req.uri(),
            Local::now()
        ),
        &Method::POST => format!(
            "POST: Received a request to {} at: {}",
            req.uri(),
            Local::now(),
        ),
        _ => format!("Received an invalid request"),
    };
    file.write_all(&message.as_bytes())
        .expect("Couldn't write to Logfile");
    println!("{}", &message);
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
        .route("/items", get(items).layer(Extension(pool.clone())))
        .route("/add/item", post(add_item).layer(Extension(pool.clone())))
        .route(
            "/order/:order_id",
            get(handle_order).layer(Extension(pool.clone())),
        )
        .layer(middleware::from_fn(logging_middleware));

    println!("Running on http://{}", &server_url_display);
    axum::Server::bind(&server_url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
