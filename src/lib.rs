mod commands;
pub mod errors;
pub mod options;
pub mod tracefs;
pub mod tracefs_sys;

use commands::{trace_list, trace_show, trace_start, trace_stop};
use errors::{Error, Result};
use options::{Command, Options};

pub fn run(opts: Options) -> Result<()> {
    match opts.cmd {
        Command::List(arg) => trace_list(arg),
        Command::Start(arg) => trace_start(arg),
        Command::Show(arg) => trace_show(arg),
        Command::Stop(arg) => trace_stop(arg),
        _ => Err(Error::Unsupprted {
            name: opts.cmd.to_string(),
        }),
    }
}

#[macro_export]
macro_rules! die {
    ($code:expr,$($arg:tt)*) => ({
        eprintln!($($arg)*);
        std::process::exit($code);
    })
}
