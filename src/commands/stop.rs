use crate::tracefs::Instance;
use crate::{options::Stop as Options};
use crate::{
    errors::{Result},
};

use super::{write_instance_tracing_on};

pub fn trace_stop(opts: Options) -> Result<()> {
    if opts.top {}
    write_instance_tracing_on(Instance::top(), false)?;
    Ok(())
}
