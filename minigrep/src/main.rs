// bring the `std::env::args` into scope with a `use` statement
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // call `env::args` and immediately use `collect` to turn iterator into a vector containging all
    // the values produced by the iterator
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
