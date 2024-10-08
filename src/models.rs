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
