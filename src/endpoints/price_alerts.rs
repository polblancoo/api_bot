use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::{
    auth::jwt::Claims,
    models::price_alerts::{PriceAlert, CreatePriceAlertRequest},
    endpoints::AppState,
    models::ApiResponse,
    notifications::webhook,
    db::price_alerts,
    utils::serde::serialize_bigdecimal,
};

pub async fn create_price_alert(
    claims: Claims,
    State(state): State<AppState>,
    Json(request): Json<CreatePriceAlertRequest>,
) -> Result<Json<ApiResponse<PriceAlert>>, (StatusCode, String)> {
    let alert = price_alerts::create_price_alert(&state.pool, claims.user_id, &request)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error creating price alert: {}", e),
            )
        })?;

    // Enviar notificación a través de webhooks
    let notification_data = json!({
        "alert_id": alert.id,
        "target_price": serialize_bigdecimal(&alert.target_price, serde_json::value::Serializer).unwrap(),
        "condition": alert.condition,
    });

    // No esperamos a que se envíen los webhooks para responder
    tokio::spawn(async move {
        if let Err(e) = webhook::send_event_to_webhooks(&state.pool, "price_alert.created", notification_data).await {
            eprintln!("Error sending webhooks: {:?}", e);
        }
    });

    Ok(Json(ApiResponse::success(alert)))
}

pub async fn get_price_alerts(
    claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<PriceAlert>>>, (StatusCode, String)> {
    let alerts = price_alerts::list_price_alerts(&state.pool, claims.user_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error fetching price alerts: {}", e),
            )
        })?;

    Ok(Json(ApiResponse::success(alerts)))
}

pub async fn get_price_alert(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<PriceAlert>>, (StatusCode, String)> {
    let alert = price_alerts::get_price_alert(&state.pool, id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error fetching price alert: {}", e),
            )
        })?
        .ok_or((StatusCode::NOT_FOUND, "Price alert not found".to_string()))?;

    if alert.user_id != claims.user_id {
        return Err((
            StatusCode::FORBIDDEN,
            "Not authorized to access this price alert".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(alert)))
}

pub async fn update_price_alert(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(request): Json<CreatePriceAlertRequest>,
) -> Result<Json<ApiResponse<PriceAlert>>, (StatusCode, String)> {
    let alert = price_alerts::update_price_alert(&state.pool, id, claims.user_id, &request)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error updating price alert: {}", e),
            )
        })?
        .ok_or((StatusCode::NOT_FOUND, "Price alert not found".to_string()))?;

    Ok(Json(ApiResponse::success(alert)))
}

pub async fn delete_price_alert(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, String)> {
    let deleted = price_alerts::delete_price_alert(&state.pool, id, claims.user_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error deleting price alert: {}", e),
            )
        })?;

    if !deleted {
        return Err((StatusCode::NOT_FOUND, "Price alert not found".to_string()));
    }

    Ok(Json(ApiResponse::success(())))
}