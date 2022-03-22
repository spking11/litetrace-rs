use std::fmt;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Options {
    #[clap(subcommand)]
    pub cmd: Command,
}

pub fn parse() -> Options {
    Options::parse()
}

#[derive(Debug)]
#[derive(Subcommand)]
pub enum Command {
    /// start tracing without recording into a file
    Start(Start),
    /// stop the kernel from recording trace data
    Stop(Stop),
    /// show the contents of the kernel tracing buffer
    Show(Show),
    /// list the available events, plugins or options
    List(List),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug)]
#[derive(Args)]
pub struct Start {}

/// Stops the tracer from recording more data.
#[derive(Debug)]
#[derive(Args)]
pub struct Stop {
    // /// -B stop a given buffer (more than one may be specified)
    // #[clap(short='B')]
    // pub buffer: String,
    // ///  -a stop all buffers (except top one)
    // #[clap(short='f')]
    // pub all: bool,
    /// -t stop the top level buffer (useful with -B or -a)
    #[clap(short)]
    pub top: bool,
}

/// trace-cmd show [-p|-s][-c cpu][-B buf][options]
/// Basically, this is a cat of the trace file.
#[derive(Debug)]
#[derive(Args)]
pub struct Show {
    /// -f display the file path that is being dumped
    #[clap(short='f')]
    pub show_name: bool,
    /// -p read the trace_pipe file instead
    #[clap(short='p')]
    pub pipe: bool,
    /// -s read the snapshot file instance
    #[clap(short='s')]
    pub snap: bool,
    #[clap(long)]
    pub tracing_on: bool,
    #[clap(long)]
    pub current_tracer: bool,
    #[clap(long)]
    pub buffer_size_kb:bool,
    #[clap(long)]
    pub buffer_total_size_kb:bool,
    #[clap(long)]
    pub set_ftrace_filter: bool,
    #[clap(long)]
    pub set_ftrace_notrace: bool,
    #[clap(long)]
    pub set_ftrace_pid: bool,
    #[clap(long)]
    pub set_graph_function: bool,
    #[clap(long)]
    pub set_graph_notrace: bool,
    #[clap(long)]
    pub tracing_cpumask: bool,
}

#[derive(Debug)]
#[derive(Args)]
pub struct List {
    /// -e list available events
    // #[clap(short, long)]
    // pub events: bool,
    /// -t list available tracers
    #[clap(short, long)]
    pub tracer: bool,
    /// -f [regex] list available functions to filter on
    #[clap(short, long)]
    pub funcs: bool,
}
