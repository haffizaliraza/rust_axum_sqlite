// api.rs


use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    routing::{get, post, put},
    Json,
};
use sqlx::{Execute, Pool};
use crate::models::{Item, NewItem, UpdateItem, Product};

pub async  fn get_products(
    Extension(pool): Extension<Pool<sqlx::Sqlite>>) -> Json<Vec<Product>> {
    let products = sqlx::query_as::<_, Product>("SELECT id, title, price, image_url, brandname FROM products")
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch products");
    

    Json(products)

}

pub async fn get_items(Extension(pool): Extension<Pool<sqlx::Sqlite>>) -> Json<Vec<Item>> {
    let items = sqlx::query_as::<_, Item>("SELECT id, name FROM items")
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch items.");

    Json(items)
}

pub async fn create_item(
    Extension(pool): Extension<Pool<sqlx::Sqlite>>,
    Json(new_item): Json<NewItem>
) -> Result<(StatusCode, Json<Item>), (StatusCode, String)> {
    let result = sqlx::query("INSERT INTO items (name) VALUES (?)")
        .bind(&new_item.name)
        .execute(&pool)
        .await;

    match result {
        Ok(inserted) => {
            let id = inserted.last_insert_rowid();
            let created_item = Item {
                id: id as i32,
                name: new_item.name.clone(),
            };

            Ok((StatusCode::CREATED, Json(created_item)))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error inserting item: {}", e))),
    }
}

pub async fn update_item(
    Path(id): Path<i32>,
    Extension(pool): Extension<Pool<sqlx::Sqlite>>,
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
        Ok((StatusCode::OK, Json(item)))
    } else {
        Err((StatusCode::NOT_FOUND, "Item not found".to_string()))
    }
}
