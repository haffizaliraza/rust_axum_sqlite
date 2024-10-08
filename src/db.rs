// db.rs

use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::env;

pub async fn connect_db() -> Pool<Sqlite> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create pool.")
}
