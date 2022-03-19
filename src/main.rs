use litetrace_rs::{options, run};

fn main() {
    let opts = options::parse();
    run(opts);
}
