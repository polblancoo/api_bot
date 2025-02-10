use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use serde_json::Value as JsonValue;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NotificationPreference {
    pub user_id: i32,
    pub email_enabled: bool,
    pub telegram_enabled: bool,
    pub whatsapp_enabled: bool,
    pub price_alerts_enabled: bool,
    pub system_alerts_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct WebhookConfig {
    pub id: uuid::Uuid,
    pub user_id: i32,
    pub url: String,
    pub secret: String,
    pub enabled: bool,
    #[serde(rename = "notification_types")]
    pub notification_types: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWebhookRequest {
    pub url: String,
    pub secret: String,
    pub notification_types: Vec<String>,
}
