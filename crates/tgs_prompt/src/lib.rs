use crossterm::style::{style, Stylize};
use dirs;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use tgs_colors::{custom, return_color_text};
use tgs_readline::prelude::*;
use tgs_services::{styled, styled_buf::StyledBuf};
pub struct TGSPrompt;

fn git_prompt() -> StyledBuf {
    let git_branch = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output();

    if let Ok(output) = git_branch {
        if output.status.success() {
            let branch_name = String::from_utf8_lossy(&output.stdout).trim().to_string();

            let git_label = style("git:").with(custom::DARK_PINK);
            let branch = style(branch_name).with(custom::ORANGE);
            let parentheses_open = style("(").with(custom::DARK_PINK);
            let parentheses_close = style(")").with(custom::DARK_PINK);

            styled! {git_label, parentheses_open, branch, parentheses_close}
        } else {
            styled! {style("").with(custom::DARK_PINK)}
        }
    } else {
        styled! {style("").with(custom::DARK_PINK)}
    }
}

impl Prompt for TGSPrompt {
    fn prompt_left(&self, line_ctx: &LineCtx) -> StyledBuf {
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

        let indicator = match line_ctx.mode() {
            LineMode::Insert => return_color_text("\u{27e3}", custom::PURPLE),
            LineMode::Normal => return_color_text(":", custom::ORANGE),
        };

        if !line_ctx.lines.is_empty() {
            return styled! {" ", indicator, " "};
        }

        let pre = return_color_text("\u{27e3}", custom::PURPLE);

        let action = return_color_text("\u{2023}", custom::CYAN);

        styled! {pre,"  ", dir," ", git_prompt()," ", action, " "}
    }
    fn prompt_right(&self, _line_ctx: &LineCtx) -> StyledBuf {
        styled!()
        // styled! {@(bold,blue)git_branch, " ", time_str, " ", lang, " "}
    }
}
