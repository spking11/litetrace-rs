use litetrace_rs::{options, run};

fn main() {
    let opts = options::parse();
    std::process::exit(match run(opts) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    })
}
