//! PhenoCrypto - Cryptographic Utilities

use aes_gcm::{aead::Aead, Aes256Gcm, Nonce, KeyInit};
use sha2::Sha256;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("encryption failed")]
    EncryptionFailed,
    #[error("decryption failed")]
    DecryptionFailed,
    #[error("invalid key")]
    InvalidKey,
}

/// AES-256-GCM encryption
pub struct AesEncryptor {
    cipher: Aes256Gcm,
}

impl AesEncryptor {
    pub fn new(key: &[u8; 32]) -> Self {
        let cipher = Aes256Gcm::new(key.into());
        Self { cipher }
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let binding: [u8; 12] = rand::random();
        let nonce = Nonce::from_slice(&binding);
        let ciphertext = self.cipher
            .encrypt(nonce, plaintext)
            .map_err(|_| CryptoError::EncryptionFailed)?;

        let mut result = binding.to_vec();
        result.extend(ciphertext);
        Ok(result)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if data.len() < 12 {
            return Err(CryptoError::DecryptionFailed);
        }

        let nonce = Nonce::from_slice(&data[..12]);
        let ciphertext = &data[12..];

        self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| CryptoError::DecryptionFailed)
    }
}

/// HMAC-SHA256 using hmac 0.12 API
pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    use hmac::{Hmac, Mac};
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = <HmacSha256 as Mac>::new(key.into());
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

/// Secure random bytes using rand 0.8
pub fn random_bytes(len: usize) -> Vec<u8> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut bytes = vec![0u8; len];
    rng.fill(bytes.as_mut_slice());
    bytes
}

/// Base64 encode
pub fn base64_encode(data: &[u8]) -> String {
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, data)
}

pub fn base64_decode(data: &str) -> Result<Vec<u8>, CryptoError> {
    base64::Engine::decode(&base64::engine::general_purpose::STANDARD, data)
        .map_err(|_| CryptoError::InvalidKey)
}
