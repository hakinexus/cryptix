use inquire::{Confirm, Select, Text, ui::{RenderConfig, Color, Styled}};
use colored::*;
use std::path::Path;
use crate::crypto::CryptoKey;
use crate::image_ops;

pub fn setup_theme() {
    let mut config = RenderConfig::default();
    config.prompt_prefix = Styled::new("◈").with_fg(Color::LightMagenta);
    config.answered_prompt_prefix = Styled::new("✓").with_fg(Color::DarkGrey);
    config.highlighted_option_prefix = Styled::new(" ❯").with_fg(Color::LightCyan);
    inquire::set_global_render_config(config);
}

fn get_local_images() -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(".") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if["png", "jpg", "jpeg", "webp", "bmp"].contains(&ext.to_lowercase().as_str()) {
                        files.push(path.file_name().unwrap().to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    files.sort();
    files
}

pub fn start_interactive_menu() {
    setup_theme();
    loop {
        println!("\n");
        let options = vec!["◈ Target Image File", "◈ Manual & Help", "✕ Terminate Session"];
        let choice = Select::new(":: SYSTEM COMMAND ::", options).prompt();

        match choice {
            Ok("◈ Target Image File") => handle_image_selection(),
            Ok("◈ Manual & Help") => print_help(),
            Ok("✕ Terminate Session") | Err(_) => {
                println!("\n  {}\n", " Session Terminated. Stay Secure. ".on_black().white().bold());
                break;
            }
            _ => {}
        }
    }
}

fn handle_image_selection() {
    let images = get_local_images();
    
    if images.is_empty() {
        println!("\n  {} {}", "⚠".bright_yellow(), "No image files found in the current directory.".bright_yellow());
        return;
    }

    let selected_file = match Select::new("Select Image Target:", images).prompt() {
        Ok(file) => file,
        Err(_) => return, 
    };

    println!("\n  {} {}\n", "TARGET ACQUIRED:".bright_black().bold(), selected_file.bright_white().bold());

    let actions = vec!["🔒 Encrypt (Lock Data)", "🔓 Decrypt (Unlock Data)", "↩ Return to Menu"];
    match Select::new("Select Processing Mode:", actions).prompt() {
        Ok("🔒 Encrypt (Lock Data)") => encrypt_flow(&selected_file),
        Ok("🔓 Decrypt (Unlock Data)") => decrypt_flow(&selected_file),
        _ => {}
    }
}

fn generate_output_name(input: &str, suffix: &str) -> String {
    let path = Path::new(input);
    let stem = path.file_stem().unwrap().to_string_lossy();
    let mut clean_stem = stem.as_ref();
    
    // Iteratively strip trailing status suffixes to prevent accumulation and avoid the greedy replace bug
    loop {
        if let Some(s) = clean_stem.strip_suffix("_locked") {
            clean_stem = s;
        } else if let Some(s) = clean_stem.strip_suffix("_decrypted") {
            clean_stem = s;
        } else {
            break;
        }
    }
    
    format!("{}{}.png", clean_stem, suffix)
}

fn check_overwrite(output_file: &str) -> bool {
    if Path::new(output_file).exists() {
        let msg = format!("File '{}' already exists. Overwrite?", output_file);
        matches!(Confirm::new(&msg).with_default(false).prompt(), Ok(true))
    } else {
        true
    }
}

fn encrypt_flow(input_file: &str) {
    let output_file = generate_output_name(input_file, "_locked");

    if !check_overwrite(&output_file) {
        println!("\n  {}", "⚠ Operation Aborted.".bright_yellow());
        return;
    }

    println!("\n  {}", " INITIALIZING ENCRYPTION MATRIX ".on_truecolor(180, 0, 255).white().bold());
    let (crypto_key, key_string) = CryptoKey::generate();

    match image_ops::process_image(input_file, &output_file, &crypto_key, "Encrypting") {
        Ok(_) => {
            println!("  {} Image secured: {}", "✔".bright_green(), output_file.bright_cyan().bold());
            print_secret_key(&key_string);
        }
        Err(e) => println!("\n  {} {}", "❌ Error:".bright_red().bold(), e),
    }
}

// THE GOD-LEVEL CONTINUOUS BLOCK: Auto-copied on Termux and flawlessly double-tap-selectable layout.
fn print_secret_key(key: &str) {
    let mut copied = false;
    
    // Attempt auto-copy for Termux natively without relying on heavy external crates
    if let Ok(mut child) = std::process::Command::new("termux-clipboard-set")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            if stdin.write_all(key.as_bytes()).is_ok() {
                copied = true;
            }
        }
        let _ = child.wait();
    }

    if copied {
        println!("\n  {} {}", "✔".bright_green().bold(), "Key automatically copied to clipboard!".bright_green().bold());
    }

    println!();
    println!("  {}", "╭────── S E C R E T   K E Y ──────╮".truecolor(180, 0, 255).bold());
    println!();
    // Continuous block highlighted natively without side borders for perfect "double-tap" word selection.
    println!("  {}", key.on_truecolor(40, 40, 40).white().bold());
    println!();
    println!("  {}", "╰─────────────────────────────────╯".truecolor(180, 0, 255).bold());
    println!("\n  {} {}", "⚠ CRITICAL:".on_red().white().bold(), "Save this key block immediately.".bright_red().bold());
    println!("  {}", "Recovery is mathematically impossible without it.\n".bright_red().bold());
}

fn decrypt_flow(input_file: &str) {
    let output_file = generate_output_name(input_file, "_decrypted");

    if !check_overwrite(&output_file) {
        println!("\n  {}", "⚠ Operation Aborted.".bright_yellow());
        return;
    }

    println!("\n  {}", " INITIALIZING DECRYPTION SEQUENCE ".on_truecolor(0, 200, 255).black().bold());
    println!("  {}\n", "Paste your key below. (If it spans multiple lines, paste it all and press Enter)".bright_black());

    let mut full_key = String::new();
    
    // SMART MULTI-LINE PASTE READER: Keeps reading until it has the full valid payload length.
    while full_key.replace(['\n', '\r', ' ', '\t'], "").len() < crate::crypto::ENCODED_KEY_LEN {
        let input = Text::new(if full_key.is_empty() { "◈" } else { "│" }).prompt();
        match input {
            Ok(line) => full_key.push_str(&line),
            Err(_) => return,
        }
    }

    let crypto_key = match CryptoKey::from_string(&full_key) {
        Ok(k) => k,
        Err(e) => {
            println!("\n  {} {}", "❌ Key Authorization Failed:".bright_red().bold(), e);
            return;
        }
    };

    println!();
    match image_ops::process_image(input_file, &output_file, &crypto_key, "Decrypting") {
        Ok(_) => {
            println!("  {} Authorization successful. Restored to: {}", "✔".bright_green(), output_file.bright_cyan().bold());
        }
        Err(e) => println!("\n  {} {}", "❌ Error:".bright_red().bold(), e),
    }
}

fn print_help() {
    println!("\n  {}", "=== CRYPTIX MANUAL ===".bright_cyan().bold());
    println!("  ◈ Select {} to scan for files in this folder.", "'Target Image File'".bright_white());
    println!("  ◈ Select your image, and explicitly choose to Encrypt or Decrypt.");
    println!("  ◈ Output files will append {} or {} safely.", "_locked.png".bright_magenta(), "_decrypted.png".bright_cyan());
    println!("  ◈ Keys are output in multi-line blocks for mobile safety. Copy the whole block.");
}
