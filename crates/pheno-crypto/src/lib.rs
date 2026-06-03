//! PhenoCrypto - Cryptographic Utilities

use aes_gcm::{aead::Aead, KeyInit as AesKeyInit, Aes256Gcm, Nonce};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};
use hmac::{Hmac, Mac, KeyInit};
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

/// HMAC-SHA256
pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    type HmacSha256 = Hmac<Sha256>;
    let mac = HmacSha256::new_from_slice(key)
        .expect("HMAC key size must be valid");
    mac.chain_update(data).finalize().into_bytes().to_vec()
}

/// Secure random bytes
pub fn random_bytes(len: usize) -> Vec<u8> {
    use rand::RngCore;
    let mut bytes = vec![0u8; len];
    rand::rng().fill_bytes(&mut bytes);
    bytes
}

/// Base64 encode
pub fn base64_encode(data: &[u8]) -> String {
    BASE64_STANDARD.encode(data)
}

pub fn base64_decode(data: &str) -> Result<Vec<u8>, CryptoError> {
    BASE64_STANDARD.decode(data)
        .map_err(|_| CryptoError::InvalidKey)
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: [u8; 32] = [7u8; 32];

    #[test]
    fn aes_encrypt_decrypt_roundtrip() {
        let enc = AesEncryptor::new(&KEY);
        let plaintext = b"phenotype substrate secret";
        let ct = enc.encrypt(plaintext).expect("encrypt");
        // ciphertext must differ from plaintext and carry the 12-byte nonce prefix
        assert_ne!(&ct[12..], plaintext);
        assert!(ct.len() > plaintext.len() + 12);
        let pt = enc.decrypt(&ct).expect("decrypt");
        assert_eq!(pt, plaintext);
    }

    #[test]
    fn aes_nonce_is_random_per_call() {
        let enc = AesEncryptor::new(&KEY);
        let a = enc.encrypt(b"same").unwrap();
        let b = enc.encrypt(b"same").unwrap();
        // distinct nonces => distinct ciphertexts for identical plaintext
        assert_ne!(a, b);
    }

    #[test]
    fn aes_decrypt_rejects_tampered_ciphertext() {
        let enc = AesEncryptor::new(&KEY);
        let mut ct = enc.encrypt(b"integrity").unwrap();
        let last = ct.len() - 1;
        ct[last] ^= 0xff; // flip a bit in the GCM tag/body
        assert!(matches!(enc.decrypt(&ct), Err(CryptoError::DecryptionFailed)));
    }

    #[test]
    fn aes_decrypt_rejects_too_short_input() {
        let enc = AesEncryptor::new(&KEY);
        assert!(matches!(enc.decrypt(&[0u8; 4]), Err(CryptoError::DecryptionFailed)));
    }

    #[test]
    fn aes_wrong_key_fails() {
        let ct = AesEncryptor::new(&KEY).encrypt(b"secret").unwrap();
        let other = AesEncryptor::new(&[9u8; 32]);
        assert!(other.decrypt(&ct).is_err());
    }

    #[test]
    fn hmac_is_deterministic_and_key_sensitive() {
        let a = hmac_sha256(b"key1", b"msg");
        let b = hmac_sha256(b"key1", b"msg");
        let c = hmac_sha256(b"key2", b"msg");
        assert_eq!(a, b, "same key+data must be stable");
        assert_ne!(a, c, "different key must change the mac");
        assert_eq!(a.len(), 32, "sha256 mac is 32 bytes");
    }

    #[test]
    fn hmac_known_vector() {
        // RFC 4231 test case 2: key="Jefe", data="what do ya want for nothing?"
        let mac = hmac_sha256(b"Jefe", b"what do ya want for nothing?");
        let hex: String = mac.iter().map(|b| format!("{b:02x}")).collect();
        assert_eq!(
            hex,
            "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
        );
    }

    #[test]
    fn random_bytes_length_and_variation() {
        assert_eq!(random_bytes(0).len(), 0);
        assert_eq!(random_bytes(64).len(), 64);
        // two draws of a reasonable size should not collide
        assert_ne!(random_bytes(32), random_bytes(32));
    }

    #[test]
    fn base64_roundtrip_and_decode_error() {
        let data = b"\x00\x01\x02\xfe\xff binary";
        let encoded = base64_encode(data);
        assert_eq!(base64_decode(&encoded).unwrap(), data);
        assert!(matches!(base64_decode("not valid base64!!!"), Err(CryptoError::InvalidKey)));
    }
}
