use crate::options::List as Options;

pub fn trace_list(opts: Options) {
    if opts.funcs {
        show_file("available_filter_functions")
    }
}
fn show_file(name: &str)
{
}
