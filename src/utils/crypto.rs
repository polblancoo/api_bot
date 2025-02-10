use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rand::Rng;
use sqlx::PgPool;
use std::env;

const NONCE_LENGTH: usize = 12;
const KEY_LENGTH: usize = 32;

#[derive(Debug)]
pub struct UserKey {
    pub user_id: i32,
    pub encrypted_key: Vec<u8>,
    pub iv: Vec<u8>,
}

fn get_master_key() -> Vec<u8> {
    let key = env::var("MASTER_KEY")
        .expect("MASTER_KEY must be set");
    
    // La clave debe ser de 32 bytes para AES-256
    let mut key_bytes = [0u8; KEY_LENGTH];
    let key_slice = key.as_bytes();
    let len = std::cmp::min(key_slice.len(), KEY_LENGTH);
    key_bytes[..len].copy_from_slice(&key_slice[..len]);
    key_bytes.to_vec()
}

pub fn generate_user_key() -> Vec<u8> {
    let mut key = vec![0u8; KEY_LENGTH];
    rand::thread_rng().fill(&mut key[..]);
    key
}

pub async fn create_user_key(pool: &PgPool, user_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    // Generar una nueva clave para el usuario
    let user_key = generate_user_key();
    
    // Generar IV aleatorio
    let mut iv = vec![0u8; NONCE_LENGTH];
    rand::thread_rng().fill(&mut iv[..]);
    
    // Encriptar la clave del usuario con la clave maestra
    let master_key = get_master_key();
    let cipher = Aes256Gcm::new_from_slice(&master_key)?;
    let nonce = Nonce::from_slice(&iv);
    let encrypted_key = cipher.encrypt(nonce, user_key.as_slice())
        .map_err(|e| format!("Encryption error: {}", e))?;

    // Guardar en la base de datos
    sqlx::query!(
        r#"
        INSERT INTO user_encryption_keys (user_id, encrypted_key, iv)
        VALUES ($1, $2, $3)
        ON CONFLICT (user_id) DO UPDATE
        SET encrypted_key = EXCLUDED.encrypted_key,
            iv = EXCLUDED.iv
        "#,
        user_id,
        encrypted_key,
        iv
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_user_key(pool: &PgPool, user_id: i32) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let user_key = sqlx::query_as!(
        UserKey,
        r#"
        SELECT user_id, encrypted_key, iv
        FROM user_encryption_keys
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or("User key not found")?;

    // Desencriptar la clave del usuario usando la clave maestra
    let master_key = get_master_key();
    let cipher = Aes256Gcm::new_from_slice(&master_key)?;
    let nonce = Nonce::from_slice(&user_key.iv);
    
    let decrypted_key = cipher.decrypt(nonce, user_key.encrypted_key.as_slice())
        .map_err(|e| format!("Decryption error: {}", e))?;

    Ok(decrypted_key)
}

pub async fn encrypt_with_user_key(pool: &PgPool, user_id: i32, data: &str) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
    let user_key = get_user_key(pool, user_id).await?;
    
    // Generar IV aleatorio
    let mut iv = vec![0u8; NONCE_LENGTH];
    rand::thread_rng().fill(&mut iv[..]);
    
    // Encriptar los datos con la clave del usuario
    let cipher = Aes256Gcm::new_from_slice(&user_key)?;
    let nonce = Nonce::from_slice(&iv);
    let encrypted_data = cipher.encrypt(nonce, data.as_bytes())
        .map_err(|e| format!("Encryption error: {}", e))?;

    Ok((encrypted_data, iv))
}

pub async fn decrypt_with_user_key(pool: &PgPool, user_id: i32, encrypted_data: &[u8], iv: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let user_key = get_user_key(pool, user_id).await?;
    
    // Desencriptar los datos con la clave del usuario
    let cipher = Aes256Gcm::new_from_slice(&user_key)?;
    let nonce = Nonce::from_slice(iv);
    let decrypted_data = cipher.decrypt(nonce, encrypted_data)
        .map_err(|e| format!("Decryption error: {}", e))?;

    String::from_utf8(decrypted_data)
        .map_err(|e| format!("UTF-8 decode error: {}", e).into())
}
