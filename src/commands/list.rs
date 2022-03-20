use crate::{options::List as Options, tracefs::use_tracing_file};
use std::{fs::File, io};
use crate::{
    errors::{Result},
};

pub fn trace_list(opts: Options) -> Result<()> {
    if opts.funcs {
        show_file("available_filter_functions")?;
    }
    Ok(())
}
fn show_file(name: &str) -> Result<()> {
    use_tracing_file(name, dump_file_content)
}

fn dump_file_content(path: &str) -> Result<()>{
    let mut file = File::options().read(true).open(path)?;
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    io::copy(&mut file, &mut stdout)?;
    Ok(())
}
