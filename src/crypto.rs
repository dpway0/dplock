use anyhow::{anyhow, Result};
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString};
use base64::Engine;
use base64::engine::general_purpose::STANDARD_NO_PAD;
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use rand::{rngs::OsRng, RngCore};
use serde::{Serialize, de::DeserializeOwned};
use bincode::{serialize, deserialize};

pub fn encrypt<T: Serialize>(data: &T, password: &str) -> Result<Vec<u8>> {
    let serialized = serialize(data)?;
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    let key = derive_key(password, &salt)?;
    let cipher = ChaCha20Poly1305::new(&key);

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher.encrypt(nonce, serialized.as_ref())
        .map_err(|e| anyhow!("Encryption failed: {e}"))?;

    let mut result = vec![];
    result.extend(&salt);
    result.extend(&nonce_bytes);
    result.extend(ciphertext);
    Ok(result)
}

pub fn decrypt<T: DeserializeOwned>(bytes: &[u8], password: &str) -> Result<T> {
    if bytes.len() < 16 + 12 {
        return Err(anyhow!("Invalid data"));
    }
    let salt = &bytes[..16];
    let nonce = &bytes[16..28];
    let ciphertext = &bytes[28..];

    let key = derive_key(password, salt)?;
    let cipher = ChaCha20Poly1305::new(&key);
    let plaintext = cipher.decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|e| anyhow!("Decryption failed: {e}"))?;
    Ok(deserialize(&plaintext)?)
}

fn derive_key(password: &str, salt: &[u8]) -> Result<Key> {
    let salt_b64 = STANDARD_NO_PAD.encode(salt);
    let salt_str = SaltString::from_b64(&salt_b64)
        .map_err(|e| anyhow!("Invalid salt: {e}"))?;

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt_str)
        .map_err(|e| anyhow!("Hashing failed: {}", e))?
        .hash
        .ok_or_else(|| anyhow!("Hash missing"))?;

    let key_bytes = password_hash.as_bytes();
    let mut key = [0u8; 32];
    let copy_len = key_bytes.len().min(32);
    key[..copy_len].copy_from_slice(&key_bytes[..copy_len]);

    Ok(*Key::from_slice(&key))
}
