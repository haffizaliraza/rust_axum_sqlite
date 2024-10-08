use axum::{
    extract::Path, http::StatusCode, routing::{get, post, put, Router}, Extension, Json
};
use serde::{Deserialize, Serialize};
use sqlx::{pool, sqlite::SqlitePoolOptions, FromRow, Pool, Sqlite};
use std::net::SocketAddr;
use dotenv::dotenv;
use std::env;

#[derive(Debug, Serialize, Deserialize, FromRow)] // Add FromRow here
struct Item {
    id: i32,
    name: String,
}


#[derive(Deserialize)] 
struct NewItem {
    name: String,
}

#[derive(Deserialize, Serialize, Debug)] 
struct UpdateItem {
    name: String,
}


async fn connect_db() -> Pool<Sqlite> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create pool.")
}

async fn get_items(Extension(pool): Extension<Pool<Sqlite>>) -> Json<Vec<Item>> {
    let items = sqlx::query_as::<_, Item>("SELECT id, name FROM items")
        .fetch_all(&pool) // Pass a reference to the pool
        .await
        .expect("Failed to fetch items.");

    Json(items)
}

async fn create_item(
    Extension(pool): Extension<Pool<Sqlite>>,
     Json(new_item): Json<NewItem>
) -> Result<(StatusCode, Json<Item>), (StatusCode, String)> {
    let result = sqlx::query("INSERT INTO items (name) VALUES (?)")
    .bind(&new_item.name)
    .execute(&pool)
    .await;

    match result {
        Ok(inserted) => {

            let id  = inserted.last_insert_rowid();
            
            let created_item = Item {
                id: id as i32,
                name: new_item.name.clone(),
            };
            
            Ok((StatusCode::CREATED, Json(created_item)))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error inserting item: {}", e))),
    }

}


async fn update_item(
    Path(id): Path<i32>,
    Extension(pool): Extension<Pool<Sqlite>>,
    Json(updated_item): Json<UpdateItem>,
) -> Result<(StatusCode, Json<Item>), (StatusCode, String)> {

    let rows_affected = sqlx::query("UPDATE items SET name =? WHERE id =?")
    .bind(&updated_item.name)
    .bind(id)
    .execute(&pool)
    .await
    .expect("Failed to execute update")
    .rows_affected();

    if rows_affected > 0 {
        let item = Item { id, name: updated_item.name };
        Ok((StatusCode::OK, Json(item) ))
    } else {
        Err((StatusCode::NOT_FOUND, "Item not found".to_string()))
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = connect_db().await;

    let app = Router::new()
        .route("/items", post(create_item))
        .route("/items/:id", put(update_item))
        .route("/items", get(get_items))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
