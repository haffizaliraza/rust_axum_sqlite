// main.rs

use axum::{
    routing::{get, post, put, Router},
    Extension,
};
use std::net::SocketAddr;
use dotenv::dotenv;

mod db;
mod models;
mod api;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = db::connect_db().await;

    let app = Router::new()
        .route("/items", post(api::create_item))
        .route("/items/:id", put(api::update_item))
        .route("/items", get(api::get_items))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
