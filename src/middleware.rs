use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::models::Claims;

const SECRET: &str = "ERSDTYUIJKNVDQR";

pub async fn validate_jwt<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_header = req.headers().get("Authorization");

    if let Some(auth_header) = auth_header {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];

                let token_data = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(SECRET.as_ref()),
                    &Validation::default(),
                );

                if token_data.is_ok() {
                    // Token is valid, proceed to the next handler
                    return Ok(next.run(req).await);
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
    
}
