use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::types::BigDecimal;
use std::str::FromStr;

pub fn serialize_bigdecimal<S>(decimal: &BigDecimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    decimal.to_string().serialize(serializer)
}

pub fn deserialize_bigdecimal<'de, D>(deserializer: D) -> Result<BigDecimal, D::Error>
where
    D: Deserializer<'de>,
{
    let decimal_str = String::deserialize(deserializer)?;
    BigDecimal::from_str(&decimal_str).map_err(serde::de::Error::custom)
}
