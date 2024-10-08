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




#[derive(Deserialize, Serialize, Debug, FromRow)]

pub struct Product{
    pub id: i32,
    pub title: String,
    pub price: f64,
    pub image_url: String,
    pub brandname: String,
}