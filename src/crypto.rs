use chacha20poly1305::{
    XChaCha20Poly1305, XNonce,
    aead::{Aead, KeyInit},
};
use rand::{rngs::OsRng, RngCore};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use zeroize::Zeroize;
use crate::error::CryptixError;

const KEY_LEN: usize = 32;
const NONCE_LEN: usize = 24;
const BUNDLE_LEN: usize = KEY_LEN + NONCE_LEN;
pub const ENCODED_KEY_LEN: usize = 75;

/// The Poly1305 authentication tag length in bytes.
pub const TAG_LEN: usize = 16;

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
        
        let encoded = URL_SAFE_NO_PAD.encode(bundle);
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

    /// Encrypts plaintext bytes using XChaCha20-Poly1305 (AEAD).
    ///
    /// Returns a tuple of `(ciphertext, tag)` where:
    /// - `ciphertext` has the same length as the input plaintext
    /// - `tag` is the 16-byte Poly1305 authentication seal
    pub fn encrypt(&self, plaintext: &[u8]) -> (Vec<u8>, [u8; TAG_LEN]) {
        let cipher = XChaCha20Poly1305::new(&self.key.into());
        let nonce = XNonce::from_slice(&self.nonce);

        // The AEAD crate returns ciphertext || tag (tag is the last 16 bytes)
        let combined = cipher.encrypt(nonce, plaintext)
            .expect("AEAD encryption failure: this should never happen with valid inputs");

        // Separate the ciphertext from the appended Poly1305 tag
        let tag_start = combined.len() - TAG_LEN;
        let mut tag = [0u8; TAG_LEN];
        tag.copy_from_slice(&combined[tag_start..]);

        let ciphertext = combined[..tag_start].to_vec();
        (ciphertext, tag)
    }

    /// Decrypts ciphertext using XChaCha20-Poly1305 (AEAD) with tag verification.
    ///
    /// If even a single byte of the ciphertext was tampered with, the Poly1305
    /// tag will mismatch and this function returns `CryptixError::TamperedData`.
    pub fn decrypt(&self, ciphertext: &[u8], tag: &[u8; TAG_LEN]) -> Result<Vec<u8>, CryptixError> {
        let cipher = XChaCha20Poly1305::new(&self.key.into());
        let nonce = XNonce::from_slice(&self.nonce);

        // Reconstruct the combined payload: ciphertext || tag
        let mut combined = Vec::with_capacity(ciphertext.len() + TAG_LEN);
        combined.extend_from_slice(ciphertext);
        combined.extend_from_slice(tag);

        cipher.decrypt(nonce, combined.as_ref())
            .map_err(|_| CryptixError::TamperedData)
    }
}
