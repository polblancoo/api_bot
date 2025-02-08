use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use sqlx::types::BigDecimal;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AssetPair {
    pub id: i32,
    pub user_id: i32,
    pub base_asset: String,
    pub quote_asset: String,
    #[serde(with = "bigdecimal_string")]
    pub slip_percentage: BigDecimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAssetPairRequest {
    pub base_asset: String,
    pub quote_asset: String,
    #[serde(with = "bigdecimal_string")]
    pub slip_percentage: BigDecimal,
}

mod bigdecimal_string {
    use serde::{de, Deserialize, Deserializer, Serializer};
    use sqlx::types::BigDecimal;
    use std::str::FromStr;

    pub fn serialize<S>(decimal: &BigDecimal, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&decimal.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BigDecimal, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        BigDecimal::from_str(&s).map_err(de::Error::custom)
    }
}
