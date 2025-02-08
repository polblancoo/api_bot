use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;
use std::sync::Arc;
use crate::{
    models::user::Claims,
    db::personal_data::{PersonalData, CreatePersonalDataRequest},
};

// Crear datos personales
pub async fn create_personal_data(
    claims: Claims,
    State(pool): State<Arc<SqlitePool>>,
    Json(data): Json<CreatePersonalDataRequest>,
) -> Result<Json<PersonalData>, (StatusCode, String)> {
    let personal_data = crate::db::personal_data::create(&pool, claims.user_id, &data)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(personal_data))
}

// Obtener datos personales del usuario
pub async fn get_personal_data(
    claims: Claims,
    State(pool): State<Arc<SqlitePool>>,
) -> Result<Json<PersonalData>, (StatusCode, String)> {
    let data = crate::db::personal_data::get_by_user_id(&pool, claims.user_id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;
    
    Ok(Json(data))
}

// Actualizar datos personales
pub async fn update_personal_data(
    claims: Claims,
    State(pool): State<Arc<SqlitePool>>,
    Json(data): Json<CreatePersonalDataRequest>,
) -> Result<Json<PersonalData>, (StatusCode, String)> {
    let updated = crate::db::personal_data::update(&pool, claims.user_id, &data)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(updated))
}

// Eliminar datos personales
pub async fn delete_personal_data(
    claims: Claims,
    State(pool): State<Arc<SqlitePool>>,
) -> Result<StatusCode, (StatusCode, String)> {
    crate::db::personal_data::delete(&pool, claims.user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(StatusCode::NO_CONTENT)
} 