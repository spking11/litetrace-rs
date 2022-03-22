mod list;
mod show;
mod start;
mod stop;

pub use list::trace_list;
pub use show::trace_show;
pub use start::trace_start;
pub use stop::trace_stop;

use crate::errors::Result;
use crate::tracefs;
use std::{
    fs::File,
    io::{self, Write},
};

fn dump_file_content(path: &str) -> Result<()> {
    let mut file = File::options().read(true).open(path)?;
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    io::copy(&mut file, &mut stdout)?;
    Ok(())
}

fn show_file(name: &str) -> Result<()> {
    dump_file_content(&tracefs::get_tracing_file(name)?)
}

fn show_instance_file(ins: &tracefs::Instance, name: &str) -> Result<()> {
    let path = &tracefs::instance_get_file(ins, name)?;
    dump_file_content(path)
}

fn write_tracing_on(ins: &tracefs::Instance, on: bool) -> Result<()> {
    let path = tracefs::instance_get_file(ins, "tracing_on")?;
    let mut file = File::options().write(true).open(path)?;
    file.write(if on { b"1" } else { b"0" })?;
    Ok(())
}
