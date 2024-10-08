// api.rs


use core::panic::PanicMessage;

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    routing::{get, post, put},
    Json,
};
use sqlx::{Execute, Pool, Sqlite};
use crate::models::{Item, NewItem, UpdateItem, Product, SignupInput, User};

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use bcrypt::{hash, verify};

const SECRET: &str = "ERSDTYUIJKNVDQR";

pub async fn signup(
    Extension(pool): Extension<Pool<Sqlite>>,
    Json(user): Json<SignupInput>,
) -> Result<(StatusCode, Json<String>), (StatusCode, String)> {
   
    let password_hash = hash(&user.password, 4).map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Hash error: {}", e))
    })?;

    let result = sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
        .bind(&user.username)
        .bind(&password_hash)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok((StatusCode::CREATED, Json("User created successfully".to_string()))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error creating user: {}", e))),
    }
}



pub async fn get_products(
    Extension(pool): Extension<Pool<sqlx::Sqlite>>) -> Json<Vec<Product>> {
    let products = sqlx::query_as::<_, Product>("SELECT id, title, price, image_url, brandname FROM products")
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch products");
    
    Json(products)
}

pub async fn delete_product(
    Path(id): Path<i32>,
    Extension(pool): Extension<Pool<Sqlite>>,
) -> Result<(StatusCode, Json<String>), (StatusCode, String)> {
    let rows_affected = sqlx::query("DELETE FROM products WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error deleting product: {}", e)))?
        .rows_affected();

    if rows_affected == 0 {
        Err((StatusCode::NOT_FOUND, "Product not found".to_string()))
    } else {
        Ok((StatusCode::OK, Json("Product deleted successfully".to_string())))
    }
}



pub async fn get_product(
    Path(id): Path<i32>,
    Extension(pool): Extension<Pool<Sqlite>>,
) -> Result<Json<Product>, (StatusCode, String)> {
    let product = sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = ?")
        .bind(id)
        .fetch_one(&pool) 
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "Product not found".to_string()))?;

    Ok(Json(product))
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
