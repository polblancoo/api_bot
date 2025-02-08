use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::{
    auth::jwt::Claims,
    db::price_alerts::{
        create_price_alert as db_create_price_alert,
        get_price_alert as db_get_price_alert,
        list_price_alerts as db_list_price_alerts,
        update_price_alert as db_update_price_alert,
        delete_price_alert as db_delete_price_alert,
    },
    endpoints::AppState,
    models::price_alerts::CreatePriceAlertRequest,
};

pub async fn create_price_alert(
    State(state): State<AppState>,
    claims: Claims,
    Json(req): Json<CreatePriceAlertRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match db_create_price_alert(&state.pool, claims.user_id, &req).await {
        Ok(price_alert) => Ok(Json(json!({
            "status": "success",
            "message": "Price alert created successfully",
            "data": price_alert
        }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error creating price alert: {}", e),
        )),
    }
}

pub async fn get_price_alerts(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match db_list_price_alerts(&state.pool, claims.user_id).await {
        Ok(price_alerts) => Ok(Json(json!({
            "status": "success",
            "message": "Price alerts retrieved successfully",
            "data": price_alerts
        }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error retrieving price alerts: {}", e),
        )),
    }
}

pub async fn get_price_alert(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match db_get_price_alert(&state.pool, id).await {
        Ok(price_alert) => {
            if price_alert.user_id != claims.user_id {
                return Err((
                    StatusCode::FORBIDDEN,
                    "Not authorized to access this price alert".to_string(),
                ));
            }
            Ok(Json(json!({
                "status": "success",
                "message": "Price alert retrieved successfully",
                "data": price_alert
            })))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error retrieving price alert: {}", e),
        )),
    }
}

pub async fn update_price_alert(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<i32>,
    Json(req): Json<CreatePriceAlertRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match db_update_price_alert(&state.pool, id, claims.user_id, &req).await {
        Ok(price_alert) => Ok(Json(json!({
            "status": "success",
            "message": "Price alert updated successfully",
            "data": price_alert
        }))),
        Err(e) => {
            if e.to_string().contains("not found") {
                Err((StatusCode::NOT_FOUND, "Price alert not found".to_string()))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error updating price alert: {}", e),
                ))
            }
        }
    }
}

pub async fn delete_price_alert(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match db_delete_price_alert(&state.pool, id, claims.user_id).await {
        Ok(_) => Ok(Json(json!({
            "status": "success",
            "message": "Price alert deleted successfully",
        }))),
        Err(e) => {
            if e.to_string().contains("not found") {
                Err((StatusCode::NOT_FOUND, "Price alert not found".to_string()))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error deleting price alert: {}", e),
                ))
            }
        }
    }
}