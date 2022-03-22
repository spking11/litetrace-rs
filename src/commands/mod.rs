mod show;
mod list;
mod start;

pub use show::trace_show;
pub use list::trace_list;
pub use start::trace_start;

use std::{fs::File, io};
use crate::{
    errors::{Result},
};
use crate::tracefs;

fn dump_file_content(path: &str) -> Result<()>{
    let mut file = File::options().read(true).open(path)?;
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    io::copy(&mut file, &mut stdout)?;
    Ok(())
}

fn show_file(name: &str) -> Result<()> {
    dump_file_content(&tracefs::get_tracing_file(name)?)
}

fn show_instance_file(ins: &tracefs::Instance, name: &str) -> Result<()>
{
    let path = &tracefs::instance_get_file(ins, name)?;
	dump_file_content(path)
}
