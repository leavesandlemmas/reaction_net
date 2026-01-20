use std::io;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use crate::source::load_source_files;
use crate::lexer::Lexer;

pub struct Config {
    files: Vec<PathBuf>,
    print_usage: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let _callname = args.next();

        let mut files: Vec<PathBuf> = Vec::new();
        let mut print_usage = false;

        for arg in args {
            if !is_option(&arg) {
                let file = Path::new(&arg);
                // skip files without extensions
                if let Some(ext) = file.extension() {
                    if valid_extension(ext) {
                        files.push(file.to_path_buf());
                        continue;
                    }
                }
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

        Ok(Config {
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

    if config.files.len() == 0 {
        return Err("No files to parse...".into());
    }

    let source_files = load_source_files(&config.files)?;
    let tokens = source_files.iter().flat_map(|f| Lexer::new(f.content().chars()));    
    for token in tokens {
            println!("{token:?}");
    }
    //parse(source_files.get());

    Ok(())
}

fn is_option(arg: &str) -> bool {
    arg.starts_with("-")
}

fn is_help(arg: &str) -> bool {
    arg == "--help" || arg == "-h"
}

fn valid_extension(ext: &OsStr) -> bool {
    let valid_exts = vec![OsStr::new("txt"), OsStr::new("rxn"), OsStr::new("crn")];
    valid_exts.contains(&&ext)
}

static USAGE: &str = "reaction_net <filename.crn> [options]

Options:
    --help                    Print usage. 
     ";
