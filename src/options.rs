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

#[derive(Args)]
pub struct Start {}

#[derive(Args)]
pub struct Stop {}

#[derive(Args)]
pub struct Show {}

#[derive(Args)]
pub struct List {
    /// -e list available events
    #[clap(short, long)]
    pub events: bool,
    /// -t list available tracers
    #[clap(short, long)]
    pub tracer: bool,
    /// -f [regex] list available functions to filter on
    #[clap(short, long)]
    pub funcs: bool,
}