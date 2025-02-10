use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use rand::{rngs::OsRng, RngCore};
use sqlx::PgPool;
use tracing::error;

const NONCE_LENGTH: usize = 12;
const KEY_LENGTH: usize = 32;

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Error de encriptación: {0}")]
    Encryption(String),
    #[error("Error de desencriptación: {0}")]
    Decryption(String),
    #[error("Clave de usuario no encontrada")]
    KeyNotFound,
    #[error("Error decodificando la clave maestra: {0}")]
    MasterKeyDecode(String),
}

/// Genera una nueva clave de encriptación para un usuario
pub async fn generate_user_key(pool: &PgPool, user_id: i32) -> Result<(), CryptoError> {
    // Generar una clave aleatoria para el usuario
    let mut user_key = vec![0u8; KEY_LENGTH];
    OsRng.fill_bytes(&mut user_key);

    // Generar IV para encriptar la clave del usuario
    let mut key_iv = vec![0u8; NONCE_LENGTH];
    OsRng.fill_bytes(&mut key_iv);

    // Obtener y decodificar la clave maestra
    let master_key_b64 = std::env::var("MASTER_KEY")
        .expect("MASTER_KEY debe estar configurada");
    
    let master_key = BASE64.decode(master_key_b64)
        .map_err(|e| CryptoError::MasterKeyDecode(e.to_string()))?;
    
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&master_key));
    let nonce = Nonce::from_slice(&key_iv);
    
    let encrypted_key = cipher
        .encrypt(nonce, user_key.as_ref())
        .map_err(|e| CryptoError::Encryption(e.to_string()))?;

    // Guardar la clave encriptada en la base de datos
    sqlx::query!(
        r#"
        INSERT INTO user_encryption_keys (user_id, key_hash, created_at, updated_at)
        VALUES ($1, $2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        "#,
        user_id,
        encrypted_key,
    )
    .execute(pool)
    .await
    .map_err(CryptoError::Database)?;

    Ok(())
}

/// Obtiene la clave de encriptación de un usuario
pub async fn get_user_key(pool: &PgPool, user_id: i32) -> Result<Vec<u8>, CryptoError> {
    let key_data = sqlx::query!(
        r#"
        SELECT key_hash
        FROM user_encryption_keys
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(CryptoError::Database)?
    .ok_or(CryptoError::KeyNotFound)?;

    // Obtener y decodificar la clave maestra
    let master_key_b64 = std::env::var("MASTER_KEY")
        .expect("MASTER_KEY debe estar configurada");
    
    let master_key = BASE64.decode(master_key_b64)
        .map_err(|e| CryptoError::MasterKeyDecode(e.to_string()))?;

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&master_key));
    let nonce = Nonce::from_slice(&key_data.key_hash[..NONCE_LENGTH]);
    let encrypted_key = &key_data.key_hash[NONCE_LENGTH..];

    cipher
        .decrypt(nonce, encrypted_key)
        .map_err(|e| CryptoError::Decryption(e.to_string()))
}

/// Encripta datos usando la clave del usuario
pub async fn encrypt_with_user_key(
    pool: &PgPool,
    user_id: i32,
    data: &[u8],
) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
    let user_key = get_user_key(pool, user_id).await?;

    let mut iv = vec![0u8; NONCE_LENGTH];
    OsRng.fill_bytes(&mut iv);

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&user_key));
    let nonce = Nonce::from_slice(&iv);

    let encrypted_data = cipher
        .encrypt(nonce, data)
        .map_err(|e| CryptoError::Encryption(e.to_string()))?;

    Ok((encrypted_data, iv))
}

/// Desencripta datos usando la clave del usuario
pub async fn decrypt_with_user_key(
    pool: &PgPool,
    user_id: i32,
    encrypted_data: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let user_key = get_user_key(pool, user_id).await?;

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&user_key));
    let nonce = Nonce::from_slice(iv);

    cipher
        .decrypt(nonce, encrypted_data)
        .map_err(|e| CryptoError::Decryption(e.to_string()))
}
