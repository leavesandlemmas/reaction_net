use std::io;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
// use crate::parse;
use crate::source::SourceIterator;
use crate::parser::Parser;
use crate::scanner::Scanner;
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

    let char_stream = SourceIterator::new(config.files);
    for ch in char_stream {
        let c = ch ?; 
        println!("{c}")
    }
    // let chars = maybe_chars?;

    // let tokens = Scanner::new(chars.into_iter());
     
    // println!("{cs:?}");
    // let char_stream = config.files
    //     .iter()
    //     .flat_map(
    //         |x| {
    //             let maybe_contents = fs::read_to_string(x).into_iter().map(|x| x.chars());

    //             maybe_contents
              
    //         }
    //     );
    // for maybe_token in tokens {
    //     let t = maybe_token?;
    //     println!("{t:?}")
    // }
    // for file in config.files {
    //     let contents = fs::read_to_string(file)?;
    //     println!("{contents}");
    //     let mtokens : Result<Vec<_>, _> = Scanner::scan(&contents).collect();
    //     let tokens = mtokens?;
    //     println!("{tokens:?}");
    //     // parser 
    // }

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
