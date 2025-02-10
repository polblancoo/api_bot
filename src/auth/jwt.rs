use axum::http::StatusCode;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tracing::debug;

const JWT_SECRET: &[u8] = b"secret";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
}

pub fn create_token(user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims { user_id };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
}

pub fn verify_token(token: &str) -> Result<Claims, StatusCode> {
    debug!("Verifying token: {}", token);
    let mut validation = Validation::default();
    validation.validate_exp = false;
    validation.required_spec_claims.remove("exp");

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(|err| {
        debug!("Token verification error: {}", err);
        StatusCode::UNAUTHORIZED
    })
}