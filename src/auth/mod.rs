use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use crate::{
    app_state::AppState,
    db::users::{self, LoginRequest},
    models::users::CreateUserRequest,
};

pub mod api_key;
pub mod jwt;
pub mod middleware;

pub async fn register(
    State(state): State<AppState>,
    api_key::ApiKey(_): api_key::ApiKey,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<()>, (StatusCode, String)> {
    match users::create_user(&state.pool, &req).await {
        Ok(_) => Ok(Json(())),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create user".to_string(),
        )),
    }
}

pub async fn login(
    State(state): State<AppState>,
    api_key::ApiKey(_): api_key::ApiKey,
    Json(req): Json<LoginRequest>,
) -> Result<Json<String>, (StatusCode, String)> {
    match users::authenticate_user(&state.pool, &req.username, &req.password).await {
        Ok(user) => {
            let token = jwt::create_token(user.id).map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create token".to_string(),
                )
            })?;
            Ok(Json(token))
        }
        Err(_) => Err((
            StatusCode::UNAUTHORIZED,
            "Invalid username or password".to_string(),
        )),
    }
}