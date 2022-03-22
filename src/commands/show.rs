use crate::errors::{Error, Result};
use crate::options::Show as Options;
use crate::tracefs::{get_tracing_file, Instance};

use super::{show_file, show_instance_file};

pub fn trace_show(opts: Options) -> Result<()> {
    if opts.tracing_on{
        show_instance_file(Instance::top(), "tracing_on")?;
        return Ok(());
    }
    if opts.current_tracer{
        show_instance_file(Instance::top(), "current_tracer")?;
        return Ok(());
    }
    if opts.buffer_size_kb{
        show_instance_file(Instance::top(), "buffer_size_kb")?;
        return Ok(());
    }
    if opts.buffer_total_size_kb{
        show_instance_file(Instance::top(), "buffer_total_size_kb")?;
        return Ok(());
    }
    if opts.set_ftrace_filter{
        show_instance_file(Instance::top(), "set_ftrace_filter")?;
        return Ok(());
    }
    if opts.set_ftrace_notrace{
        show_instance_file(Instance::top(), "set_ftrace_notrace")?;
        return Ok(());
    }
    if opts.set_ftrace_pid{
        show_instance_file(Instance::top(), "set_ftrace_pid")?;
        return Ok(());
    }
    if opts.set_graph_function{
        show_instance_file(Instance::top(), "set_graph_function")?;
        return Ok(());
    }
    if opts.set_graph_notrace{
        show_instance_file(Instance::top(), "set_graph_notrace")?;
        return Ok(());
    }
    if opts.tracing_cpumask{
        show_instance_file(Instance::top(), "tracing_cpumask")?;
        return Ok(());
    }
    if opts.pipe && opts.snap {
        return Err(Error::ArgError {
            msg: "Can not have -s and -p together".to_string(),
        });
    }
    
    let mut file = "trace";

    if opts.pipe {
        file = "trace_pipe";
    }
    if opts.snap {
        file = "snapshot";
    }
    if opts.show_name {
        println!("{}", get_tracing_file(file)?);
    }
    show_file(file)?;
    Ok(())
}
