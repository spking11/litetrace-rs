use crate::{options::List as Options};
use crate::{
    errors::{Result},
};

use super::show_file;

pub fn trace_list(opts: Options) -> Result<()> {
    if opts.funcs {
        show_file("available_filter_functions")?;
    }
    if opts.tracer{
        show_file("available_tracers")?;
    }
    Ok(())
}
