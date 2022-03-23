use crate::tracefs::Instance;
use crate::{options::Clear as Options};
use crate::{
    errors::{Result},
};

use super::{clear_instance_trace};

pub fn trace_clear(_opts: Options) -> Result<()> {
    clear_instance_trace(Instance::top())
}
