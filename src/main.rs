use std::env;
use std::process;

use reaction_net::Config;
use reaction_net::run;

fn main() {
    let args = env::args();
    // for arg in args.try_into() {
    //     match arg {
    //         "help" => println!("reaction_net *filenames*");
    //         otherwise => println!("Arg {n} = {arg}");
    //     }
    // }
    let config = Config::build(args).unwrap_or_else( |err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    } );

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }; 


}
