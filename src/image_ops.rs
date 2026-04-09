use image::{ImageBuffer, RgbaImage};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::Duration;
use crate::crypto::{CryptoKey, TAG_LEN};
use crate::error::CryptixError;

/// Creates and configures the signature Cryptix progress spinner.
fn make_spinner() -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", "✓"])
            .template("  {spinner:.magenta} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(80));
    spinner
}

/// Encrypts an image using XChaCha20-Poly1305 (AEAD).
///
/// The 16-byte Poly1305 authentication tag is appended after the PNG IEND chunk
/// on disk, making it invisible to standard image viewers but recoverable by
/// Cryptix during decryption for tamper verification.
pub fn encrypt_image(
    input: &str,
    output: &str,
    key: &CryptoKey,
) -> Result<(), CryptixError> {
    if !output.to_lowercase().ends_with(".png") {
        return Err(CryptixError::LossyFormat);
    }

    let spinner = make_spinner();

    // --- Phase 1: Load ---
    spinner.set_message("Encrypting ║ Loading image matrix...");
    let img = image::open(Path::new(input))?.to_rgba8();
    let (width, height) = img.dimensions();
    let raw_pixels = img.into_raw();

    // --- Phase 2: AEAD Encrypt ---
    spinner.set_message("Encrypting ║ Sealing pixels via XChaCha20-Poly1305...");
    let (ciphertext, tag) = key.encrypt(&raw_pixels);

    // --- Phase 3: Recompile into PNG ---
    spinner.set_message("Encrypting ║ Recompiling visual data...");
    let new_img: RgbaImage = ImageBuffer::from_raw(width, height, ciphertext)
        .expect("Critical engine failure: byte count mismatch.");

    // --- Phase 4: Write PNG to disk ---
    spinner.set_message("Encrypting ║ Writing binary to disk...");
    new_img.save(Path::new(output))?;

    // --- Phase 5: Append Poly1305 Tag (Post-IEND Strategy) ---
    spinner.set_message("Encrypting ║ Embedding authentication seal...");
    let mut file = fs::OpenOptions::new().append(true).open(output)?;
    file.write_all(&tag)?;
    file.flush()?;

    spinner.finish_with_message("Encrypting ║ Operation complete. Vault sealed.");
    Ok(())
}

/// Decrypts an image using XChaCha20-Poly1305 (AEAD) with tamper verification.
///
/// Reads the 16-byte Poly1305 tag from the end of the file (after IEND),
/// verifies data integrity, and only then produces the decrypted output.
/// If even a single byte was altered, returns `CryptixError::TamperedData`.
pub fn decrypt_image(
    input: &str,
    output: &str,
    key: &CryptoKey,
) -> Result<(), CryptixError> {
    if !output.to_lowercase().ends_with(".png") {
        return Err(CryptixError::LossyFormat);
    }

    let spinner = make_spinner();

    // --- Phase 1: Read raw file bytes ---
    spinner.set_message("Decrypting ║ Reading encrypted vault...");
    let file_bytes = fs::read(input)?;

    if file_bytes.len() <= TAG_LEN {
        return Err(CryptixError::TamperedData);
    }

    // --- Phase 2: Split off the Poly1305 Tag ---
    spinner.set_message("Decrypting ║ Extracting authentication seal...");
    let split_point = file_bytes.len() - TAG_LEN;
    let png_bytes = &file_bytes[..split_point];
    let mut tag = [0u8; TAG_LEN];
    tag.copy_from_slice(&file_bytes[split_point..]);

    // --- Phase 3: Decode the PNG from memory ---
    spinner.set_message("Decrypting ║ Decoding image matrix...");
    let img = image::load_from_memory(png_bytes)
        .map_err(CryptixError::Image)?
        .to_rgba8();
    let (width, height) = img.dimensions();
    let encrypted_pixels = img.into_raw();

    // --- Phase 4: AEAD Decrypt with Tag Verification ---
    spinner.set_message("Decrypting ║ Verifying integrity & decrypting via XChaCha20-Poly1305...");
    let plaintext = key.decrypt(&encrypted_pixels, &tag)?;

    // --- Phase 5: Recompile and Save ---
    spinner.set_message("Decrypting ║ Recompiling visual data...");
    let restored_img: RgbaImage = ImageBuffer::from_raw(width, height, plaintext)
        .expect("Critical engine failure: byte count mismatch.");

    spinner.set_message("Decrypting ║ Writing restored image to disk...");
    restored_img.save(Path::new(output))?;

    spinner.finish_with_message("Decrypting ║ Operation complete. Vault unsealed.");
    Ok(())
}
