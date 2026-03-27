use image::{ImageBuffer, RgbaImage};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use std::time::Duration;
use crate::crypto::CryptoKey;
use crate::error::CryptixError;

pub fn process_image(
    input: &str,
    output: &str,
    key: &CryptoKey,
    action: &str,
) -> Result<(), CryptixError> {
    if !output.to_lowercase().ends_with(".png") {
        return Err(CryptixError::LossyFormat);
    }

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", "✓"])
            .template("  {spinner:.magenta} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(80));

    spinner.set_message(format!("{} Loading image matrix...", action));
    let img = image::open(Path::new(input))?.to_rgba8();
    let (width, height) = img.dimensions();
    
    let mut raw_pixels = img.into_raw();

    spinner.set_message(format!("{} Splicing pixels via XChaCha20...", action));
    key.apply_keystream(&mut raw_pixels);

    spinner.set_message(format!("{} Recompiling visual data...", action));
    let new_img: RgbaImage = ImageBuffer::from_raw(width, height, raw_pixels)
        .expect("Critical engine failure: byte count mismatch.");

    spinner.set_message(format!("{} Writing binary to disk...", action));
    new_img.save(Path::new(output))?;

    spinner.finish_with_message(format!("{} Operation complete.", action));
    Ok(())
}
