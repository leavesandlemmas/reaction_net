use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

mod language;
use language::parser::{Parser, SyntaxError};
use language::scanner::{Scanner, LexError};
use language::grammar::Terminal;

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
        println!("{contents}");

        let scanner = Scanner::scan(&contents);
        let tokens : Result<Vec<Terminal>, LexError> = scanner.collect();
        let tokens = tokens? ;
        for token in tokens.iter() {
            println!("{token:?}")
        }       
        let mut parser = Parser::new(tokens.into_iter());
        let _ = parser.parse()?;
    }

    Ok(())
}

fn is_option(arg: &str) -> bool {
    arg.starts_with("-")
}

fn is_help(arg: &str) -> bool {
    arg == "--help" || arg == "-h"
}

fn valid_extension(ext : &OsStr) -> bool {
    let valid_exts = vec![OsStr::new("txt"),OsStr::new("rxn"), OsStr::new("crn")];  
    valid_exts.contains(&&ext)
}

static USAGE: &str = "reaction_net <filename.crn> [options]

Options:
    --help                    Print usage. 
     ";
