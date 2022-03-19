mod commands;
pub mod options;

use options::{Options, Command};
use commands::{trace_show, trace_list};

pub fn run(options: Options) {
    match options.cmd {
        Command::Show(arg) => trace_show(arg),
        Command::List(arg) => trace_list(arg),

        _ => println!("Unsupprted command now!")
    }
}
