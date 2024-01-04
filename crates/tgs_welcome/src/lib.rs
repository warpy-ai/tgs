use sysinfo::System;
use tgs_colors::{blend_color, custom, display_color_text, display_gradient_text};

fn display_neon_art_with_gradient() {
    let ascii_art = r#"
************************
************************
************************
**   .--.             **
**   |__| .-------.   **
**   |=.| |.-----.|   **
**   |--| || TGS ||   **
**   |  | |'-----'|   **
**   |__|~')_____('   **
************************
************************
************************


========================
    "#;

    // Define gradient colors
    let start_color = custom::MAGENTA; // Magenta
    let end_color = custom::CYAN; // Cyan

    // Split the art into lines
    let lines: Vec<&str> = ascii_art.lines().collect();

    // Apply gradient to each line
    for (i, line) in lines.iter().enumerate() {
        let gradient_ratio = i as f32 / lines.len() as f32;
        let blended_color = blend_color(start_color, end_color, gradient_ratio);
        display_color_text(line, blended_color);
    }
}

pub fn display_welcome_message() {
    display_neon_art_with_gradient();

    let welcome_text = format!(
        "Welcome to Terminal Generative Shell\nHostname: {}\nOS: {}\nUptime: {:?}\n",
        System::name().unwrap_or_default(),
        System::os_version().unwrap_or_default(),
        System::uptime()
    );

    // Define gradient colors for the welcome text
    let start_color_welcome = custom::ORANGE;
    let end_color_welcome = custom::MAGENTA;

    display_gradient_text(&welcome_text, start_color_welcome, end_color_welcome);
}
