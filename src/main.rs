use std::env;
use std::process;

// use reaction_net::Config;
// use reaction_net::run;

mod reactions;
use reactions::RxNet;



fn main() {

    let crn = RxNet::build_example();
    println!("{crn:?}");

    // let t = crn.translate();
    // println!("{t}");
    // let args = env::args();

    // let config = Config::build(args).unwrap_or_else( |err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // } );

    // if let Err(e) = run(config) {
    //     eprintln!("Application error: {e}");
    //     process::exit(1);
    // }; 


}
