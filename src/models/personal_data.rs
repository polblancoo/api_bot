use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PersonalData {
    pub id: i32,
    pub user_id: i32,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePersonalDataRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePersonalDataRequest {
    pub email: String,
}
