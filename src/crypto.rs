use chacha20::{XChaCha20, cipher::{KeyIvInit, StreamCipher}};
use rand::{rngs::OsRng, RngCore};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use zeroize::Zeroize;
use crate::error::CryptixError;

const KEY_LEN: usize = 32;
const NONCE_LEN: usize = 24;
const BUNDLE_LEN: usize = KEY_LEN + NONCE_LEN;

pub struct CryptoKey {
    key: [u8; KEY_LEN],
    nonce: [u8; NONCE_LEN],
}

impl Drop for CryptoKey {
    fn drop(&mut self) {
        self.key.zeroize();
        self.nonce.zeroize();
    }
}

impl CryptoKey {
    pub fn generate() -> (Self, String) {
        let mut key = [0u8; KEY_LEN];
        let mut nonce =[0u8; NONCE_LEN];
        
        OsRng.fill_bytes(&mut key);
        OsRng.fill_bytes(&mut nonce);
        
        let mut bundle =[0u8; BUNDLE_LEN];
        bundle[..KEY_LEN].copy_from_slice(&key);
        bundle[KEY_LEN..].copy_from_slice(&nonce);
        
        let encoded = URL_SAFE_NO_PAD.encode(&bundle);
        bundle.zeroize(); 
        
        (Self { key, nonce }, encoded)
    }

    pub fn from_string(encoded: &str) -> Result<Self, CryptixError> {
        // Automatically strips formatting, spaces, and newlines from copied text!
        let clean_encoded = encoded.replace(['\n', '\r', ' ', '\t'], "");
        
        let decoded = URL_SAFE_NO_PAD.decode(&clean_encoded)
            .map_err(|_| CryptixError::InvalidKey("Improper Base64 format".into()))?;
        
        if decoded.len() != BUNDLE_LEN {
            return Err(CryptixError::InvalidKey("Incorrect key length. Ensure you copied all lines.".into()));
        }
        
        let mut key =[0u8; KEY_LEN];
        let mut nonce = [0u8; NONCE_LEN];
        
        key.copy_from_slice(&decoded[..KEY_LEN]);
        nonce.copy_from_slice(&decoded[KEY_LEN..]);
        
        Ok(Self { key, nonce })
    }

    pub fn apply_keystream(&self, data: &mut [u8]) {
        let mut cipher = XChaCha20::new(&self.key.into(), &self.nonce.into());
        cipher.apply_keystream(data);
    }
}
