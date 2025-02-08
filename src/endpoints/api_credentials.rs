use axum::{
    extract::{State, Path},
    Json,
    http::StatusCode,
};
use crate::{
    db::api_credentials::{self, ApiCredential, CreateApiCredentialRequest},
    auth::jwt::Claims,
    endpoints::AppState,
};

pub async fn create(
    claims: Claims,
    State(state): State<AppState>,
    Json(req): Json<CreateApiCredentialRequest>,
) -> Result<(StatusCode, Json<ApiCredential>), StatusCode> {
    api_credentials::create(&state.pool, claims.user_id, &req)
        .await
        .map(|cred| (StatusCode::CREATED, Json(cred)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn list(
    claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<Vec<ApiCredential>>, (StatusCode, String)> {
    let user_id: i32 = claims.user_id;  // Ya es i32

    api_credentials::list_by_user(&state.pool, user_id)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn get_one(
    _claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiCredential>, (StatusCode, String)> {
    let id: i32 = id.try_into()
        .map_err(|_| (StatusCode::BAD_REQUEST, "ID inválido".to_string()))?;

    api_credentials::get_by_id(&state.pool, id)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))
}

pub async fn update(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<CreateApiCredentialRequest>,
) -> Result<Json<ApiCredential>, StatusCode> {
    let id: i32 = id.try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let user_id: i32 = claims.user_id;

    api_credentials::update(&state.pool, id, user_id, &req)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn delete(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let id: i32 = id.try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let user_id: i32 = claims.user_id;

    api_credentials::delete(&state.pool, id, user_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// ... otros endpoints similares ... 