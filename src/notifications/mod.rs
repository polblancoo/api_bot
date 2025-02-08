pub mod queue;
pub mod websocket;
pub mod webhook;
pub mod models;

use crate::models::users::DbUser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    PriceAlert,
    MarketSentiment,
    StrategyUpdate,
    TradeExecution,
    SystemAlert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Option<i32>,
    pub user_id: i32,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub metadata: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub read_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Notification {
    pub fn new(
        user_id: i32,
        notification_type: NotificationType,
        title: String,
        message: String,
        metadata: serde_json::Value,
    ) -> Self {
        Self {
            id: None,
            user_id,
            notification_type,
            title,
            message,
            metadata,
            created_at: chrono::Utc::now(),
            read_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreference {
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
}
