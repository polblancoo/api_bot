use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use std::error::Error;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

pub fn encrypt_api_key(password: &str, api_key: &str) -> Result<String, Box<dyn Error>> {
    // Generar salt
    let salt = SaltString::generate(&mut OsRng);
    
    // Crear hasher
    let argon2 = Argon2::default();
    
    // Generar key desde password
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    let key = password_hash.to_string();

    // Crear cipher
    let cipher = Aes256Gcm::new_from_slice(key.as_bytes())?;
    
    // Generar nonce
    let nonce = Nonce::from_slice(b"unique nonce"); // En producción, usar un nonce aleatorio

    // Encriptar
    let ciphertext = cipher.encrypt(nonce, api_key.as_bytes().as_ref())?;

    Ok(BASE64.encode(&ciphertext))
}

pub fn decrypt_api_key(password: &str, encrypted_api_key: &str) -> Result<String, Box<dyn Error>> {
    // Similar al proceso de encrypt pero en reversa
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    let key = password_hash.to_string();

    let cipher = Aes256Gcm::new_from_slice(key.as_bytes())?;
    let nonce = Nonce::from_slice(b"unique nonce");

    let ciphertext = BASE64.decode(encrypted_api_key)?;
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())?;

    String::from_utf8(plaintext).map_err(|e| e.into())
} 