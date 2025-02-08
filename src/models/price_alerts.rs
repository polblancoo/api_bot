use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use sqlx::types::BigDecimal;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PriceAlert {
    pub id: i32,
    pub user_id: i32,
    pub asset: String,
    #[serde(with = "bigdecimal_string")]
    pub target_price: BigDecimal,
    pub condition: String,
    #[serde(with = "bigdecimal_string_option")]
    pub trigger_price: Option<BigDecimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePriceAlertRequest {
    pub asset: String,
    #[serde(with = "bigdecimal_string")]
    pub target_price: BigDecimal,
    pub condition: String,
    #[serde(with = "bigdecimal_string_option")]
    pub trigger_price: Option<BigDecimal>,
}

mod bigdecimal_string {
    use serde::{de, Deserialize, Deserializer, Serializer};
    use sqlx::types::BigDecimal;

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
        s.parse().map_err(de::Error::custom)
    }
}

mod bigdecimal_string_option {
    use serde::{Deserialize, Deserializer, Serializer};
    use sqlx::types::BigDecimal;

    pub fn serialize<S>(decimal: &Option<BigDecimal>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match decimal {
            Some(d) => serializer.serialize_str(&d.to_string()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<BigDecimal>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(deserializer)?;
        match opt {
            Some(s) => Ok(Some(s.parse().map_err(serde::de::Error::custom)?)),
            None => Ok(None),
        }
    }
}
