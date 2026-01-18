use std::env;
use std::process;

// use reaction_net::cli;

use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::str::from_utf8;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    args.next();
    let file = args.next().expect("Pass an argument");
    let f = File::open(file)?;
    let mut reader = BufReader::new(f);
    let mut buffer : [u8; 4] = [0; 4];

    // read a line into buffer
    reader.read(&mut buffer)?;
    let source = from_utf8(&buffer)?;
    println!("{source}");
    Ok(())

    // let config = cli::Config::build(args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    // if let Err(e) = cli::run(config) {
    //     eprintln!("Application error: {e}");
    //     process::exit(1);
    // };
}
