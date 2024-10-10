// models.rs

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)] // Add FromRow here
pub struct Item {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize)]
pub struct NewItem {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateItem {
    pub name: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Product{
    pub id: i32,
    pub title: String,
    pub price: f64,
    pub image_url: String,
    pub brandname: String,
    pub quantity: i64,
}

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct NewProduct {
    pub title: String,
    pub price: f64,
    pub image_url: String,
    pub brandname: String, 
    pub quantity: i64,
}

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

#[derive(Deserialize)]
pub struct SignupInput {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct JwtResponse {
    pub token: String,
}
