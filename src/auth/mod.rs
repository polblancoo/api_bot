use axum::{
    async_trait,
    extract::{FromRequestParts, State, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;
use bcrypt::verify;
use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    db::users::get_by_username,
    endpoints::AppState,
    models::users::LoginRequest,
};

pub mod jwt;
pub mod admin;
pub mod middleware;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub exp: usize,
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> impl IntoResponse {
    match get_by_username(&state.pool, &req.username).await {
        Ok(user) => {
            match verify(req.password.as_bytes(), &user.password_hash) {
                Ok(valid) => {
                    if valid {
                        let config = Config::from_env().unwrap();
                        let token = jwt::create_token(user.id, &config).unwrap();
                        (
                            StatusCode::OK,
                            Json(json!({
                                "status": "success",
                                "message": "Login successful",
                                "data": {
                                    "token": token,
                                    "user": {
                                        "id": user.id,
                                        "username": user.username,
                                        "created_at": user.created_at,
                                        "is_active": user.is_active,
                                        "last_login": user.last_login,
                                        "telegram_id": user.telegram_id,
                                    }
                                }
                            }))
                        )
                    } else {
                        (
                            StatusCode::UNAUTHORIZED,
                            Json(json!({
                                "status": "error",
                                "message": "Invalid username or password"
                            }))
                        )
                    }
                }
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": "Error verifying password"
                    }))
                )
            }
        }
        Err(e) => {
            if e.to_string().contains("no rows") {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "status": "error",
                        "message": "Invalid username or password"
                    }))
                )
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": format!("Error getting user: {}", e)
                    }))
                )
            }
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, _state)
                .await
                .map_err(|_| {
                    (
                        StatusCode::UNAUTHORIZED,
                        "Invalid or missing authorization header".to_string(),
                    )
                })?;

        let config = Config::from_env().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error loading config".to_string(),
            )
        })?;

        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Token verification failed".to_string(),
            )
        })?;

        Ok(token_data.claims)
    }
}