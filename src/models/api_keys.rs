use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ApiKey {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub key: String,
    pub exchange: String,
    pub permissions: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub exchange: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateApiKeyRequest {
    pub name: Option<String>,
    pub exchange: Option<String>,
    pub permissions: Option<Vec<String>>,
    pub is_active: Option<bool>,
}

// Validaciones
pub fn is_valid_exchange(exchange: &str) -> bool {
    matches!(exchange.to_lowercase().as_str(), "binance" | "kucoin" | "bybit")
}

pub fn validate_permissions(permissions: &[String]) -> bool {
    let valid_permissions = vec!["read", "trade", "withdraw"];
    permissions.iter().all(|p| valid_permissions.contains(&p.as_str()))
}
