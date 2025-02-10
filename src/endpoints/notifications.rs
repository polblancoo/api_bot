use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
    routing::{get, put, post, delete},
};
use serde::Deserialize;
use tracing::error;
use uuid::Uuid;

use crate::{
    auth::jwt::Claims,
    db::notifications,
    models::notifications::{CreateWebhookRequest, NotificationPreference, WebhookConfig},
    app_state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct UpdatePreferencesRequest {
    pub email_enabled: Option<bool>,
    pub telegram_enabled: Option<bool>,
    pub whatsapp_enabled: Option<bool>,
    pub price_alerts_enabled: Option<bool>,
    pub system_alerts_enabled: Option<bool>,
}

pub async fn get_preferences(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<NotificationPreference>, (StatusCode, String)> {
    match notifications::get_notification_preferences(&state.pool, claims.user_id).await {
        Ok(prefs) => Ok(Json(prefs)),
        Err(sqlx::Error::RowNotFound) => {
            // Si no existen preferencias, crear unas por defecto
            let now = chrono::Utc::now();
            let prefs = sqlx::query_as::<_, NotificationPreference>(
                r#"
                INSERT INTO notification_preferences (
                    user_id,
                    email_enabled,
                    telegram_enabled,
                    whatsapp_enabled,
                    price_alerts_enabled,
                    system_alerts_enabled,
                    created_at,
                    updated_at
                )
                VALUES ($1, true, true, false, true, true, $2, $2)
                RETURNING user_id, email_enabled, telegram_enabled, whatsapp_enabled,
                         price_alerts_enabled, system_alerts_enabled, created_at, updated_at
                "#
            )
            .bind(claims.user_id)
            .bind(now)
            .fetch_one(&state.pool)
            .await
            .map_err(|e| {
                error!("Error al crear preferencias por defecto: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error al crear preferencias por defecto".to_string(),
                )
            })?;

            Ok(Json(prefs))
        }
        Err(e) => {
            error!("Error al obtener preferencias: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error al obtener preferencias".to_string(),
            ))
        }
    }
}

pub async fn update_preferences(
    State(state): State<AppState>,
    claims: Claims,
    Json(request): Json<UpdatePreferencesRequest>,
) -> Result<Json<NotificationPreference>, (StatusCode, String)> {
    let updated = sqlx::query_as::<_, NotificationPreference>(
        r#"
        UPDATE notification_preferences
        SET 
            email_enabled = COALESCE($1, email_enabled),
            telegram_enabled = COALESCE($2, telegram_enabled),
            whatsapp_enabled = COALESCE($3, whatsapp_enabled),
            price_alerts_enabled = COALESCE($4, price_alerts_enabled),
            system_alerts_enabled = COALESCE($5, system_alerts_enabled),
            updated_at = CURRENT_TIMESTAMP
        WHERE user_id = $6
        RETURNING user_id, email_enabled, telegram_enabled, whatsapp_enabled,
                 price_alerts_enabled, system_alerts_enabled, created_at, updated_at
        "#
    )
    .bind(request.email_enabled)
    .bind(request.telegram_enabled)
    .bind(request.whatsapp_enabled)
    .bind(request.price_alerts_enabled)
    .bind(request.system_alerts_enabled)
    .bind(claims.user_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        error!("Error al actualizar preferencias: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al actualizar preferencias".to_string(),
        )
    })?;

    Ok(Json(updated))
}

pub async fn create_webhook(
    State(state): State<AppState>,
    claims: Claims,
    Json(request): Json<CreateWebhookRequest>,
) -> Result<Json<WebhookConfig>, (StatusCode, String)> {
    match notifications::create_webhook(&state.pool, claims.user_id, &request).await {
        Ok(webhook) => Ok(Json(webhook)),
        Err(e) => {
            error!("Error al crear webhook: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error al crear webhook".to_string(),
            ))
        }
    }
}

pub async fn list_webhooks(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<WebhookConfig>>, (StatusCode, String)> {
    match notifications::list_webhooks(&state.pool, claims.user_id).await {
        Ok(webhooks) => Ok(Json(webhooks)),
        Err(e) => {
            error!("Error al listar webhooks: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error al listar webhooks".to_string(),
            ))
        }
    }
}

pub async fn delete_webhook(
    State(state): State<AppState>,
    claims: Claims,
    Path(webhook_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    match notifications::delete_webhook(&state.pool, claims.user_id, webhook_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(sqlx::Error::RowNotFound) => {
            Err((StatusCode::NOT_FOUND, "Webhook no encontrado".to_string()))
        }
        Err(e) => {
            error!("Error al eliminar webhook: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error interno del servidor".to_string(),
            ))
        }
    }
}

pub fn notifications_router() -> Router<AppState> {
    Router::new()
        .route("/preferences", get(get_preferences).put(update_preferences))
        .route("/webhooks", get(list_webhooks).post(create_webhook))
        .route("/webhooks/:id", delete(delete_webhook))
}
