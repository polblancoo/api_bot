use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Router,
};
use serde_json::{json, Value as JsonValue};

use crate::{
    app_state::AppState,
    auth::jwt::Claims,
    db::api_keys::{self, CreateApiKeyRequest, UpdateApiKeyRequest},
};

pub fn api_keys_router() -> Router<AppState> {
    Router::new()
        .route("/api-keys", post(create_api_key))
        .route("/api-keys", get(list_api_keys))
        .route("/api-keys/:id", get(get_api_key))
        .route("/api-keys/:id", put(update_api_key))
        .route("/api-keys/:id", delete(delete_api_key))
}

pub async fn create_api_key(
    State(app_state): State<AppState>,
    claims: Claims,
    Json(req): Json<CreateApiKeyRequest>,
) -> Result<Json<JsonValue>, (StatusCode, String)> {
    match api_keys::create_api_key(&app_state.pool, claims.user_id, req).await {
        Ok(api_key) => Ok(Json(json!({
            "status": "success",
            "message": "API key creada exitosamente",
            "data": {
                "id": api_key.id,
                "name": api_key.name,
                "exchange": api_key.exchange,
                "permissions": api_key.permissions,
                "created_at": api_key.created_at,
                "updated_at": api_key.updated_at
            }
        }))),
        Err(e) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al crear la API key: {}", e),
            ))
        }
    }
}

pub async fn get_api_key(
    State(app_state): State<AppState>,
    claims: Claims,
    Path(id): Path<i32>,
) -> Result<Json<JsonValue>, (StatusCode, String)> {
    match api_keys::get_api_key(&app_state.pool, claims.user_id, id).await {
        Ok(Some(api_key)) => Ok(Json(json!({
            "status": "success",
            "message": "API key encontrada",
            "data": {
                "id": api_key.id,
                "name": api_key.name,
                "exchange": api_key.exchange,
                "permissions": api_key.permissions,
                "created_at": api_key.created_at,
                "updated_at": api_key.updated_at
            }
        }))),
        Ok(None) => Err((StatusCode::NOT_FOUND, "API key no encontrada".to_string())),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al obtener la API key: {}", e),
        )),
    }
}

pub async fn list_api_keys(
    State(app_state): State<AppState>,
    claims: Claims,
) -> Result<Json<JsonValue>, (StatusCode, String)> {
    match api_keys::list_api_keys(&app_state.pool, claims.user_id).await {
        Ok(api_keys) => Ok(Json(json!({
            "status": "success",
            "message": "API keys encontradas",
            "data": api_keys.iter().map(|key| {
                json!({
                    "id": key.id,
                    "name": key.name,
                    "exchange": key.exchange,
                    "permissions": key.permissions,
                    "created_at": key.created_at,
                    "updated_at": key.updated_at
                })
            }).collect::<Vec<_>>()
        }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al listar las API keys: {}", e),
        )),
    }
}

pub async fn update_api_key(
    State(app_state): State<AppState>,
    claims: Claims,
    Path(id): Path<i32>,
    Json(req): Json<UpdateApiKeyRequest>,
) -> Result<Json<JsonValue>, (StatusCode, String)> {
    match api_keys::update_api_key(&app_state.pool, claims.user_id, id, &req).await {
        Ok(api_key) => Ok(Json(json!({
            "status": "success",
            "message": "API key actualizada exitosamente",
            "data": {
                "id": api_key.id,
                "name": api_key.name,
                "exchange": api_key.exchange,
                "permissions": api_key.permissions,
                "created_at": api_key.created_at,
                "updated_at": api_key.updated_at
            }
        }))),
        Err(e) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al actualizar la API key: {}", e),
            ))
        }
    }
}

pub async fn delete_api_key(
    State(app_state): State<AppState>,
    claims: Claims,
    Path(id): Path<i32>,
) -> Result<Json<JsonValue>, (StatusCode, String)> {
    match api_keys::delete_api_key(&app_state.pool, id, claims.user_id).await {
        Ok(true) => Ok(Json(json!({
            "status": "success",
            "message": "API key eliminada exitosamente"
        }))),
        Ok(false) => Err((StatusCode::NOT_FOUND, "API key no encontrada".to_string())),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al eliminar la API key: {}", e),
        )),
    }
}
