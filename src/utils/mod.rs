use rust_decimal::Decimal;
use sqlx::types::BigDecimal;
use std::str::FromStr;

pub mod serde;
pub mod user_crypto;

pub trait DecimalConversion {
    fn to_decimal(&self) -> Decimal;
}

impl DecimalConversion for BigDecimal {
    fn to_decimal(&self) -> Decimal {
        Decimal::from_str(&self.to_string()).unwrap()
    }
}

pub trait BigDecimalConversion {
    fn to_bigdecimal(&self) -> BigDecimal;
}

impl BigDecimalConversion for Decimal {
    fn to_bigdecimal(&self) -> BigDecimal {
        BigDecimal::from_str(&self.to_string()).unwrap()
    }
}
