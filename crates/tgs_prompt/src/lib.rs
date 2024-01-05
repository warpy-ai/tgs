use dirs;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use tgs_colors::{custom, return_color_text};
mod loading_indicator;

pub fn terminal_prompt() {
    let current_dir = env::current_dir().unwrap();
    let home_dir = dirs::home_dir().expect("Could not find home directory");

    let path = if current_dir.starts_with(&home_dir) {
        let relative_path = current_dir.strip_prefix(&home_dir).unwrap();
        PathBuf::from("~").join(relative_path)
    } else {
        current_dir.clone()
    };

    let current_dir_str = path.to_str().unwrap();

    let dir = return_color_text(&current_dir_str, custom::DARK_BLUE);

    let git_branch = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output();

    let git_prompt = if let Ok(output) = git_branch {
        if output.status.success() {
            let branch_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let branch = return_color_text(&branch_name, custom::ORANGE);
            let git_label = return_color_text("git:", custom::DARK_PINK);
            let parentheses = return_color_text("(", custom::DARK_PINK); // Color the opening parenthesis
            let closing_parentheses = return_color_text(")", custom::DARK_PINK); // Color the closing parenthesis
            format!(
                "{}{}{}{}",
                git_label, parentheses, branch, closing_parentheses
            )
        } else {
            String::new()
        }
    } else {
        String::new() // or handle error as needed
    };

    let pre = return_color_text("\u{27e3}", custom::PURPLE);
    let action = return_color_text("\u{2023}", custom::CYAN);

    print!("{}  {} {} {} ", pre, dir, git_prompt, action);
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately.
}

pub fn format_as_table(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();

    // Check if the output is in long format (e.g., starts with file permissions)
    let is_long_format = lines
        .get(0)
        .map_or(false, |line| line.starts_with("d") || line.starts_with("-"));

    let mut table = String::from("\u{2502}");
    if is_long_format {
        for line in lines {
            table.push_str("\t");
            table.push_str(line);
            table.push_str("\n\u{2502}");
        }
    } else {
        for (i, item) in lines
            .iter()
            .flat_map(|line| line.split_whitespace())
            .enumerate()
        {
            table.push_str("\t");
            table.push_str(item);
            if (i + 1) % 2 == 0 {
                table.push_str("\n\u{2502}");
            }
        }
    }

    if !table.ends_with('\n') {
        table.push('\n');
    }
    table.push_str("\u{2502}_");

    table
}
