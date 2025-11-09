use std::error::Error;
use std::fs;
use std::path::PathBuf;


mod scanner;

pub struct Config {
    callname: String,
    files: Vec<PathBuf>,
    print_usage: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let callname = args.next().expect("No callname found...");

        let mut files: Vec<PathBuf> = Vec::new();
        let mut print_usage = false;

        for arg in args {
            if !is_option(&arg) {
                files.push(arg.into());
                continue;
            }

            if is_help(&arg) {
                print_usage = true;
                continue;
            }

            // trim --
            let _arg_op = (&arg)
                .strip_prefix("--")
                .expect("Optional arguments should start with `--`");

            return Err("Unknown Argument");
        }

        if files.len() == 0 {
            return Err("No files to parse...");
        }

        Ok(Config {
            callname,
            files,
            print_usage,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.print_usage {
        println!("{USAGE}");
        return Ok(());
    }

    for file in config.files {
        let contents = fs::read_to_string(file)?;
        let tokens = Tokens::parse(contents.chars())?;

        tokens.print();
    }

    Ok(())
}

fn is_option(arg: &str) -> bool {
    arg.starts_with("-")
}

fn is_help(arg: &str) -> bool {
    arg == "--help" || arg == "-h"
}

static USAGE: &str = "reaction_net [options] <filename.crn>

Options:
    --help                    Print usage. 
     ";

