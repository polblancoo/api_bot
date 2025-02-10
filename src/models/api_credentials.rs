use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiCredential {
    pub id: i32,
    pub user_id: i32,
    pub exchange: String,
    pub api_key: String,
    pub api_secret: String,
    pub permissions: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateApiCredentialRequest {
    pub exchange: String,
    pub api_key: String,
    pub api_secret: String,
    pub permissions: Vec<String>,
}
