//! `delta_lib` produces syntax-highlighed output for input from `git diff`
//! and other subcommands such as `log`, `blame` and `grep`.

extern crate bitflags;

#[macro_use]
extern crate error_chain;

mod align;
mod ansi;
mod color;
mod colors;
mod edits;
mod features;
mod format;
mod handlers;
mod minusplus;
mod options;
mod paint;
mod parse_style;
mod parse_styles;
mod style;
mod tests;
mod wrapping;

pub mod cli;
pub mod config;
pub mod delta;
pub mod env;
pub mod git_config;
pub mod mainfn;
pub mod subcommands;
pub mod utils;

fn fatal<T>(errmsg: T) -> !
where
    T: AsRef<str> + std::fmt::Display,
{
    #[cfg(not(test))]
    {
        use std::process;
        eprintln!("{}", errmsg);
        // As in Config::error_exit_code: use 2 for error
        // because diff uses 0 and 1 for non-error.
        process::exit(2);
    }
    #[cfg(test)]
    panic!("{}\n", errmsg);
}

mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            SyntectError(::syntect::LoadingError);
            ParseIntError(::std::num::ParseIntError);
        }
    }
}
