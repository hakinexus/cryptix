mod crypto;
mod error;
mod image_ops;
mod menu;

use colored::*;

use std::io::{self, Write};

fn center_print(text: &str, text_len: usize, term_width: usize) {
    let padding = term_width.saturating_sub(text_len) / 2;
    println!("{}{}", " ".repeat(padding), text);
}

fn main() {
    // Clear terminal and scrollback buffer for a true premium experience
    print!("\x1B[H\x1B[2J\x1B[3J");
    let _ = io::stdout().flush();
    
    let term_width = crossterm::terminal::size().map(|(w, _)| w as usize).unwrap_or(80);
    
    // God-Level Responsive Banners based on terminal width
    let large_banner = r#"
      ██████╗██████╗ ██╗   ██╗██████╗ ████████╗██╗██╗  ██╗
     ██╔════╝██╔══██╗╚██╗ ██╔╝██╔══██╗╚══██╔══╝██║╚██╗██╔╝
     ██║     ██████╔╝ ╚████╔╝ ██████╔╝   ██║   ██║ ╚███╔╝ 
     ██║     ██╔══██╗  ╚██╔╝  ██╔═══╝    ██║   ██║ ██╔██╗ 
     ╚██████╗██║  ██║   ██║   ██║        ██║   ██║██╔╝ ██╗
      ╚═════╝╚═╝  ╚═╝   ╚═╝   ╚═╝        ╚═╝   ╚═╝╚═╝  ╚═╝"#;

    let small_banner = r#"
   ___ ______   ______ _____ _______  __
  / __|  _ \ \ / /  _ \_   _|_   _\ \/ /
 | (__| |_) \ V /| |_) || |   | |  \  / 
  \___|  _ < | | |  __/ | |   | |  /  \ 
      |_| \_\|_| |_|    |_|   |_| /_/\_\"#;

    println!();
    
    if term_width >= 62 {
        for line in large_banner.lines() {
            if !line.is_empty() {
                center_print(&line.truecolor(200, 0, 255).bold().to_string(), 62, term_width);
            }
        }
    } else if term_width >= 40 {
        for line in small_banner.lines() {
            if !line.is_empty() {
                center_print(&line.truecolor(200, 0, 255).bold().to_string(), 42, term_width);
            }
        }
    } else {
        center_print(&"C R Y P T I X".truecolor(200, 0, 255).bold().to_string(), 13, term_width);
        println!();
    }
    
    // High-contrast neon cyan UI core metrics
    println!();
    let title = " ADVANCED VISUAL ENCRYPTION SUITE ".on_truecolor(0, 210, 255).black().bold().to_string();
    center_print(&title, 34, term_width);
    
    let version = env!("CARGO_PKG_VERSION");
    // Replace the dots with spaced dots instead of spacing every character
    let version_spaced = version.replace(".", " . ");
    
    let display_version = format!("V E R S I O N   {}", version_spaced);
    let v_len = display_version.chars().count();
    center_print(&display_version.truecolor(100, 100, 100).bold().to_string(), v_len, term_width);
    println!();
    
    // Launch the God-Level Terminal UI
    menu::start_interactive_menu();
}
