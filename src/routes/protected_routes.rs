use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: i32,
    exp: usize,
}

impl Claims {
    pub fn new(sub: i32, exp: usize) -> Self {
        Self { sub, exp }
    }
}

pub async fn require_auth(req: Request, next: Next) -> Result<Response, (StatusCode, String)> {
    let auth_header = req.headers().get("Authorization");

    if let Some(auth_header) = auth_header {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                let token_data = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret("your_secret_key".as_ref()),
                    &Validation::default(),
                );

                if token_data.is_ok() {
                    return Ok(next.run(req).await);
                }
            }
        }
    }

    Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()))
}
