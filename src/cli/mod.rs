pub mod args;
pub mod shell;
#[cfg(feature = "cli-colors")]
pub mod colors;
#[cfg(feature = "cli-json")]
pub mod json;

pub use args::Args;
pub use shell::ShellScript;

#[cfg(feature = "cli-colors")]
pub use colors::*;

#[cfg(feature = "cli-json")]
pub use json::{JsonError, JsonResult};