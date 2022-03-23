mod list;
mod show;
mod start;
mod stop;
mod clear;

pub use list::trace_list;
pub use show::trace_show;
pub use start::trace_start;
pub use stop::trace_stop;
pub use clear::trace_clear;

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
/*
 * Top
 */
fn show_file(name: &str) -> Result<()> {
    dump_file_content(&tracefs::get_tracing_file(name)?)
}

fn write_file(name: &str, content: &str) -> Result<()> {
    let path = tracefs::get_tracing_file(name)?;
    let mut file = File::options().write(true).open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/*
 * instance, can also top instance
 */
fn show_instance_file(ins: &tracefs::Instance, name: &str) -> Result<()> {
    let path = &tracefs::instance_get_file(ins, name)?;
    dump_file_content(path)
}

fn read_instance_file(ins: &tracefs::Instance, file: &str) -> Result<String> {
    let path = tracefs::instance_get_file(ins, file)?;
    Ok(std::fs::read_to_string(path)?)
}

fn write_instance_file(ins: &tracefs::Instance, name: &str, content: &str) -> Result<()> {
    let path = tracefs::instance_get_file(ins, name)?;
    let mut file = File::options().write(true).truncate(true).open(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn write_instance_tracing_on(ins: &tracefs::Instance, on: bool) -> Result<()> {
    write_instance_file(ins, "tracing_on", if on { "1" } else { "0" })
}

fn clear_instance_trace(ins: &tracefs::Instance) -> Result<()> {
    write_instance_file(ins, "trace", "0")
}
