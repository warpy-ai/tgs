#[macro_use]
extern crate derive_builder;

pub mod buffer_history;
pub mod completion;
pub mod cursor;
pub mod highlight;
pub mod hooks;
pub mod line;
pub mod menu;
pub mod painter;
pub mod prompt;
pub mod vi;

// TODO kinda ugly rexporting shrs_core here
pub use shrs_core as _core;

pub mod prelude {
    //! Imports the commonly used structs and types

    // Macros
    pub use crate::{
        buffer_history::{BufferHistory, DefaultBufferHistory},
        completion::*,
        cursor::CursorStyle,
        highlight::{DefaultHighlighter, Highlighter, SyntaxHighlighter, SyntaxTheme},
        hooks::*,
        line::{Line, LineBuilder, LineBuilderError, LineCtx, LineMode, Readline},
        menu::{DefaultMenu, Menu},
        prompt::{DefaultPrompt, Prompt, *},
        vi::*,
    };
}

#[cfg(test)]
mod tests {}
