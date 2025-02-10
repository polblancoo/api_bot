use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::{
    db::{
        users::get_all_users,
        asset_pairs::get_all_asset_pairs_admin,
        price_alerts::list_all_price_alerts,
    },
    endpoints::AppState,
};

pub async fn get_users(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match get_all_users(&state.pool).await {
        Ok(users) => Ok(Json(json!({
            "status": "success",
            "message": "Users retrieved successfully",
            "data": users
        }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error retrieving users: {}", e),
        )),
    }
}

pub async fn get_all_asset_pairs(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match get_all_asset_pairs_admin(&state.pool).await {
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

pub async fn get_all_alerts(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match list_all_price_alerts(&state.pool).await {
        Ok(alerts) => Ok(Json(json!({
            "status": "success",
            "message": "Price alerts retrieved successfully",
            "data": alerts
        }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error retrieving price alerts: {}", e),
        )),
    }
}
