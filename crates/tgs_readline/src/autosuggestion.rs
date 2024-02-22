use crate::{
    completion::{Completer, CompletionCtx},
    line::LineCtx,
};
use crossterm::style::{Color, ContentStyle};
use tgs_services::styled_buf::StyledBuf;

pub trait Autosuggester {
    fn suggest(&self, input: &str) -> Option<String>;
}

pub struct DefaultAutosuggester;

impl Autosuggester for DefaultAutosuggester {
    fn suggest(&self, input: &str) -> Option<String> {
        // Example implementation
        // This should be replaced with actual suggestion logic
        Some(format!("{}-suggestion", input))
    }
}

pub struct Autosuggestion;

impl Autosuggestion {
    pub fn fetch_autosuggestion(
        completer: &dyn Completer, // Assuming Completer is the trait that provides completions
        ctx: &LineCtx,
    ) -> Option<String> {
        // Extract the user's current input up to the cursor position
        let input = ctx.cb.slice(..ctx.cb.cursor()).as_str().unwrap_or_default();

        // Create a context for completion based on the current input
        // This step depends on how CompletionCtx is structured and what information it needs
        // For example, it might need the current input split into arguments
        let args: Vec<String> = input.split_whitespace().map(String::from).collect();
        let comp_ctx = CompletionCtx::new(args);

        // Fetch completions based on the provided context
        let completions = completer.complete(&comp_ctx);

        // Return the first completion's accepted value as the suggestion
        completions.first().map(|comp| comp.accept())
    }

    pub fn render_with_autosuggestion(
        line_ctx: &LineCtx,
        autosuggestion: Option<String>,
        styled_buf: &mut StyledBuf,
    ) {
        let current_input = line_ctx.cb.as_str();
        let current_input_str = current_input.as_ref();

        // line_ctx.update_suggestion(autosuggestion);
        if let Some(suggestion) = autosuggestion {
            let trimmed_selection = &suggestion[current_input_str.len()..];
            let suggestion_extension =
                match Autosuggestion::complete_input(current_input_str, trimmed_selection) {
                    Some(s) => s,
                    None => suggestion.clone(),
                };

            styled_buf.push(
                &suggestion_extension,
                ContentStyle {
                    foreground_color: Some(Color::DarkGrey),
                    ..Default::default()
                },
            );
        }
    }

    pub fn complete_input(input: &str, completion_candidate: &str) -> Option<String> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let last_part = parts.last().unwrap_or(&"");

        if completion_candidate.starts_with(last_part) {
            // Calculate the start index for the substring we need to append
            let start_idx = last_part.len();
            // Extract the substring from the completion candidate that is not in the input
            Some(completion_candidate[start_idx..].to_string())
        } else {
            // Return None if the completion candidate does not start with the last part of the input
            None
        }
    }
}
