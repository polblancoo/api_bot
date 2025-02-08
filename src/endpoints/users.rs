use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use crate::{
    AppState,
    auth::{jwt, jwt::Claims},
    db::users,
    models::users::{CreateUserRequest, LoginRequest, User},
    config::Config,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde_json::json;
use tracing::{debug, error, warn};

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Hash de la contraseña
    let password_hash = hash(req.password.as_bytes(), DEFAULT_COST).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error hashing password: {}", e),
        )
    })?;

    // Crear usuario en la base de datos
    match users::create_user(&state.pool, &req, password_hash).await {
        Ok(user) => Ok(Json(json!({
            "status": "success",
            "message": "User created successfully",
            "data": User {
                id: user.id,
                username: user.username,
                created_at: user.created_at,
                is_active: user.is_active,
                last_login: user.last_login,
                telegram_id: user.telegram_id,
                is_admin: user.is_admin,
            }
        }))),
        Err(e) => {
            if e.to_string().contains("unique constraint") {
                Err((
                    StatusCode::CONFLICT,
                    "Username already registered".to_string(),
                ))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error creating user: {}", e),
                ))
            }
        }
    }
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Buscar usuario por username
    let user = match users::get_by_username(&state.pool, &req.username).await {
        Ok(user) => user,
        Err(e) => {
            if e.to_string().contains("no rows") {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    "Invalid username or password".to_string(),
                ));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error getting user: {}", e),
            ));
        }
    };

    // Verificar contraseña
    if !verify(req.password.as_bytes(), &user.password_hash).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error verifying password: {}", e),
        )
    })? {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Invalid username or password".to_string(),
        ));
    }

    // Crear token JWT
    let config = Config::from_env().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error loading config: {}", e),
        )
    })?;

    let token = jwt::create_token(user.id, &config).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error creating token: {}", e),
        )
    })?;

    Ok(Json(json!({
        "status": "success",
        "message": "Login successful",
        "data": {
            "token": token,
            "user": User {
                id: user.id,
                username: user.username,
                created_at: user.created_at,
                is_active: user.is_active,
                last_login: user.last_login,
                telegram_id: user.telegram_id,
                is_admin: user.is_admin,
            }
        }
    })))
}

pub async fn delete_user(
    claims: Claims,
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Verificar que el usuario tiene permisos
    if claims.user_id != user_id {
        return Err((StatusCode::FORBIDDEN, "No tienes permiso para eliminar este usuario".to_string()));
    }

    match users::deactivate_user(&state.pool, user_id.into()).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn verify_token(
    claims: Claims,
    State(_state): State<AppState>,
) -> impl IntoResponse {
    Json(json!({
        "message": "Token válido",
        "user_id": claims.user_id
    }))
}

pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    // Hash de la contraseña
    let password_hash = match hash(req.password.as_bytes(), DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Crear usuario en la base de datos
    match users::create_user(&state.pool, &req, password_hash).await {
        Ok(user) => Ok(Json(json!({
            "status": "success",
            "message": "User created successfully",
            "data": User {
                id: user.id,
                username: user.username,
                created_at: user.created_at,
                is_active: user.is_active,
                last_login: user.last_login,
                telegram_id: user.telegram_id,
                is_admin: user.is_admin,
            }
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}