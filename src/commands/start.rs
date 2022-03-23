use crate::errors::{Error, Result};
use crate::options::Start as Options;
use crate::tracefs;

use super::{
    clear_instance_trace, read_instance_file, write_instance_file, write_instance_tracing_on,
};

pub fn trace_start(opts: Options) -> Result<()> {
    if let Some(plugin) = opts.plugin {
        /* disable all tracing */
        tracecmd_disable_all_tracing()?;
        /* set funcs */
        set_funcs(tracefs::Instance::top(), &opts.funcs)?;
        /* check and enable plugin*/
        update_plugin(&plugin)?;
        /* enable tracing */
        enable_tracing()?;
    } else if let Some(_) = opts.event {
        return Err(Error::Unsupprted {
            name: "-e event".to_string(),
        });
    } else {
        return Err(Error::ArgError {
            msg: "no event or plugin was specified".to_string(),
        });
    }
    Ok(())
}

fn tracecmd_disable_all_tracing() -> Result<()> {
    write_instance_tracing_on(tracefs::Instance::top(), false)?; // close tracing_on

    // disable_func_stack_trace();
    // set_plugin("nop")
    // reset_events
    // Force close and reset of ftrace pid file

    clear_instance_trace(tracefs::Instance::top())?; // clear instance
    Ok(())
}

fn set_funcs(ins: &tracefs::Instance, funcs: &Vec<String>) -> Result<()> {
    tracefs::function_filter(
        ins,
        None,
        None,
        tracefs::TRACEFS_FL_RESET | tracefs::TRACEFS_FL_CONTINUE,
    )?;
    if funcs.is_empty() {
        tracefs::function_filter(
            ins,
            Some("*".to_string()),
            None,
            tracefs::TRACEFS_FL_CONTINUE,
        )?;
    } else {
        for func in funcs.iter() {
            tracefs::function_filter(
                ins,
                Some(func.to_string()),
                None,
                tracefs::TRACEFS_FL_CONTINUE,
            )?;
        }
    }

    Ok(())
}

fn update_plugin(plugin: &str) -> Result<()> {
    // check_plugin
    let plugins = read_instance_file(tracefs::Instance::top(), "available_tracers")?;
    if plugins.split_whitespace().any(|p| p == plugin) {
        eprintln!("  plugin {}", plugin);
    } else {
        return Err(Error::PluginNotExit {
            name: plugin.to_string(),
        });
    }
    // update current tracer
    write_instance_file(tracefs::Instance::top(), "current_tracer", plugin)?;
    // if function reset func_stack_trace
    if plugin == "function" {
        write_instance_file(tracefs::Instance::top(), "options/func_stack_trace", "0")?;
    }
    Ok(())
}

fn enable_tracing() -> Result<()> {
    // The debugfs file tracing_enabled needs to be deprecated. ignore
    // all instance
    write_instance_tracing_on(tracefs::Instance::top(), true)
}
