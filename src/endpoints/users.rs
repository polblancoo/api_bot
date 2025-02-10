use crate::{
    app_state::AppState,
    auth::jwt::Claims,
    db::{personal_data, users},
    models::personal_data::UpdatePersonalDataRequest,
};
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value as JsonValue};
use tracing::error;

pub fn users_router() -> Router<AppState> {
    Router::new()
        .route("/users/me", get(get_current_user))
        .route("/users/me/personal-data", get(get_personal_data))
        .route("/users/me/personal-data", post(update_personal_data))
}

async fn get_current_user(
    State(app_state): State<AppState>,
    claims: Claims,
) -> Result<Json<JsonValue>, (axum::http::StatusCode, String)> {
    let user = users::get_user(&app_state.pool, claims.user_id)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al obtener el usuario: {}", e),
            )
        })?;

    match user {
        Some(user) => Ok(Json(json!({
            "status": "success",
            "message": "Usuario encontrado",
            "data": {
                "id": user.id,
                "username": user.username,
                "email": user.email,
                "created_at": user.created_at,
                "updated_at": user.updated_at
            }
        }))),
        None => Err((axum::http::StatusCode::NOT_FOUND, "Usuario no encontrado".to_string())),
    }
}

async fn get_personal_data(
    State(app_state): State<AppState>,
    claims: Claims,
) -> Result<Json<JsonValue>, (axum::http::StatusCode, String)> {
    match personal_data::get_personal_data(&app_state.pool, claims.user_id).await {
        Ok(data) => Ok(Json(json!({
            "status": "success",
            "message": "Datos personales encontrados",
            "data": data
        }))),
        Err(e) => {
            error!("Error getting personal data: {}", e);
            Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Error interno del servidor".to_string(),
            ))
        }
    }
}

async fn update_personal_data(
    State(app_state): State<AppState>,
    claims: Claims,
    Json(req): Json<UpdatePersonalDataRequest>,
) -> Result<Json<JsonValue>, (axum::http::StatusCode, String)> {
    match personal_data::update_personal_data(&app_state.pool, claims.user_id, &req).await {
        Ok(data) => Ok(Json(json!({
            "status": "success",
            "message": "Datos personales actualizados",
            "data": data
        }))),
        Err(e) => {
            error!("Error updating personal data: {}", e);
            Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Error interno del servidor".to_string(),
            ))
        }
    }
}