use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DbNotification {
    pub id: i32,
    pub user_id: i32,
    pub notification_type: String,
    pub title: String,
    pub message: String,
    pub metadata: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub read_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DbNotificationPreference {
    pub user_id: i32,
    pub email_enabled: bool,
    pub telegram_enabled: bool,
    pub whatsapp_enabled: bool,
    pub web_push_enabled: bool,
    pub price_alerts_enabled: bool,
    pub market_sentiment_enabled: bool,
    pub strategy_updates_enabled: bool,
    pub trade_execution_enabled: bool,
    pub system_alerts_enabled: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
