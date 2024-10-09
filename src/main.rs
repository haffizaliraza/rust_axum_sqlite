// main.rs

use axum::{
    routing::{delete, get, post, put, Router},
    Extension,
};
use std::net::SocketAddr;
use dotenv::dotenv;

mod db;
mod models;
mod api;

use tower_http::cors::{CorsLayer, Any};



#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = db::connect_db().await;



    
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);


    let app = Router::new()
    .route("/api/signup", post(api::signup))
    .route("/api/login", post(api::login))
    .route("/api/items", post(api::create_item))
    .route("/api/items/:id", put(api::update_item))
    .route("/api/items", get(api::get_items))
    .route("/api/products", get(api::get_products))
    .route("/api/products/:id", get(api::get_product))
    .route("/api/products/:id", delete(api::delete_product))
    .layer(cors)
    .layer(Extension(pool));


    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
