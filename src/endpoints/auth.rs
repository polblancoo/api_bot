use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use serde_json::{json, Value as JsonValue};
use tracing::error;

use crate::{
    app_state::AppState,
    auth::jwt,
    db::users::{self, LoginRequest},
    models::users::CreateUserRequest,
};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}

pub async fn login(
    State(app_state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<JsonValue>, (axum::http::StatusCode, String)> {
    match users::authenticate_user(&app_state.pool, &req.username, &req.password).await {
        Ok(user) => {
            // Generar token JWT
            let token = jwt::create_token(user.id).map_err(|e| {
                error!("Error creating token: {}", e);
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Error interno del servidor".to_string(),
                )
            })?;

            Ok(Json(json!({
                "status": "success",
                "message": "Login exitoso",
                "data": {
                    "id": user.id,
                    "username": user.username,
                    "email": user.email,
                    "token": token
                }
            })))
        }
        Err(e) => {
            error!("Error authenticating user: {}", e);
            Err((
                axum::http::StatusCode::UNAUTHORIZED,
                "Credenciales inválidas".to_string(),
            ))
        }
    }
}

pub async fn register(
    State(app_state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<JsonValue>, (axum::http::StatusCode, String)> {
    let _password_hash = bcrypt::hash(req.password.as_bytes(), bcrypt::DEFAULT_COST)
        .map_err(|e| {
            error!("Error hashing password: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Error interno del servidor".to_string(),
            )
        })?;

    match users::create_user(&app_state.pool, &req).await {
        Ok(user) => Ok(Json(json!({
            "status": "success",
            "message": "Usuario creado exitosamente",
            "data": {
                "id": user.id,
                "username": user.username,
                "email": user.email
            }
        }))),
        Err(e) => {
            error!("Error creating user: {}", e);
            Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Error interno del servidor".to_string(),
            ))
        }
    }
}
