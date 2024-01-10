//! **tgs** is a rust shell with ai
//!
//! # Example
//! The most basic shell can be created very easily:
//! ```no_run
//! use tgs_shell::prelude::*;
//!
//! fn main() {
//!     let myshell = ShellBuilder::default()
//!         .build()
//!         .unwrap();
//!
//!     myshell.run().unwrap();
//! }
//! ```

#[macro_use]
extern crate derive_builder;

pub use tgs_core::*;

pub mod lang {
    //! Shell command language

    pub use tgs_lexer::*;
}

pub mod line {
    //! Readline implementation

    pub use tgs_readline::*;
}

mod shell;
pub use shell::*;

pub mod plugin;

pub mod crossterm {
    //! Re-export of crossterm types

    pub use crossterm::{
        style::{Print, Stylize},
        QueueableCommand,
    };
}

pub mod anyhow {
    //! Re-export of anyhow crate for error handling
    pub use anyhow::{anyhow, Error, Result, *};
}

pub mod prelude {
    //! `use tgs_shell::prelude::*` to import most commonly used structs and functions

    pub use tgs_core::prelude::*;
    pub use tgs_lexer::PosixLang;
    pub use tgs_readline::prelude::*;
    pub use tgs_services::*;

    pub use crate::{anyhow, crossterm, crossterm::*, plugin::*, shell::*};
}
