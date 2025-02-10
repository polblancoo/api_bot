use chrono::{DateTime, Utc};
use sqlx::types::BigDecimal;
use serde::{Deserialize, Serialize};
use crate::utils::serde::{serialize_bigdecimal, deserialize_bigdecimal};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePriceAlertRequest {
    #[serde(serialize_with = "serialize_bigdecimal", deserialize_with = "deserialize_bigdecimal")]
    pub target_price: BigDecimal,
    pub condition: String,
    pub asset: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceAlert {
    pub id: i32,
    pub user_id: i32,
    #[serde(serialize_with = "serialize_bigdecimal", deserialize_with = "deserialize_bigdecimal")]
    pub target_price: BigDecimal,
    pub condition: String,
    pub asset: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
