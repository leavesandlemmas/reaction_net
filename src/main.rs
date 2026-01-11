use std::env;
use std::process;

use reaction_net::cli;

fn main() {
    let args = env::args();

    let config = cli::Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = cli::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
