mod commands;
pub mod options;
pub mod tracefs;
pub mod tracefs_sys;
pub mod errors;

use errors::{Error, Result};
use commands::{trace_list};
use options::{Command, Options};

pub fn run(options: Options) ->Result<()> {
    match options.cmd {
        Command::List(arg) => trace_list(arg),
        _ => Err(Error::Unsupprted { name: options.cmd.to_string() }),
    }
}

#[macro_export]
macro_rules! die {
    ($code:expr,$($arg:tt)*) => ({
        eprintln!($($arg)*);
        std::process::exit($code);
    })
}
