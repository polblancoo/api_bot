use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::{
    auth::jwt::Claims,
    db::asset_pairs::{
        create_asset_pair as db_create_asset_pair,
        get_asset_pair as db_get_asset_pair,
        list_asset_pairs as db_list_asset_pairs,
        update_asset_pair as db_update_asset_pair,
        delete_asset_pair as db_delete_asset_pair,
    },
    endpoints::AppState,
    models::asset_pairs::{AssetPair, CreateAssetPairRequest},
};

pub async fn create_asset_pair(
    claims: Claims,
    State(state): State<AppState>,
    Json(req): Json<CreateAssetPairRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match db_create_asset_pair(&state.pool, claims.user_id, &req).await {
        Ok(asset_pair) => Ok(Json(json!({
            "status": "success",
            "message": "Asset pair created successfully",
            "data": asset_pair
        }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error creating asset pair: {}", e),
        )),
    }
}

pub async fn list_asset_pairs(
    claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match db_list_asset_pairs(&state.pool, claims.user_id).await {
        Ok(asset_pairs) => Ok(Json(json!({
            "status": "success",
            "message": "Asset pairs retrieved successfully",
            "data": asset_pairs
        }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error retrieving asset pairs: {}", e),
        )),
    }
}

pub async fn get_asset_pair(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match db_get_asset_pair(&state.pool, id, claims.user_id).await {
        Ok(asset_pair) => Ok(Json(json!({
            "status": "success",
            "message": "Asset pair retrieved successfully",
            "data": asset_pair
        }))),
        Err(e) => {
            if e.to_string().contains("no rows") {
                Err((StatusCode::NOT_FOUND, "Asset pair not found".to_string()))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error retrieving asset pair: {}", e),
                ))
            }
        }
    }
}

pub async fn update_asset_pair(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<CreateAssetPairRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match db_update_asset_pair(&state.pool, id, claims.user_id, &req).await {
        Ok(asset_pair) => Ok(Json(json!({
            "status": "success",
            "message": "Asset pair updated successfully",
            "data": asset_pair
        }))),
        Err(e) => {
            if e.to_string().contains("no rows") {
                Err((StatusCode::NOT_FOUND, "Asset pair not found".to_string()))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error updating asset pair: {}", e),
                ))
            }
        }
    }
}

pub async fn delete_asset_pair(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match db_delete_asset_pair(&state.pool, id, claims.user_id).await {
        Ok(_) => Ok(Json(json!({
            "status": "success",
            "message": "Asset pair deleted successfully",
        }))),
        Err(e) => {
            if e.to_string().contains("no rows") {
                Err((StatusCode::NOT_FOUND, "Asset pair not found".to_string()))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error deleting asset pair: {}", e),
                ))
            }
        }
    }
}