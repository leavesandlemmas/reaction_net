use std::error::Error;
use std::fs;
use std::path::PathBuf;

mod args;

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

// symbols
#[derive(Debug)]
enum Symbol {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Plus,
    Star,
    SemiColon,
    Colon,
    Equal,
    Greater,
    Less,
    Minus,
    Slash,
    RightArrow,
    LeftArrow,
    LeftRightArrow,
    Tick,
    Quote,
    Comma,
    Species(String),
    Number(u32),
}

struct Tokens {
    tokens: Vec<Symbol>,
}

impl Tokens {
    fn parse(characters: impl Iterator<Item = char>) -> Result<Tokens, String> {
        let mut tokens: Vec<Symbol> = Vec::new();
        let mut iterator = characters.peekable();
        let mut line: u64 = 1;
        while let Some(c) = iterator.next() {
            if c.is_whitespace() {
                if c == '\n' {
                    line += 1
                }
            } else {
                match c {
                    '(' => tokens.push(Symbol::LeftParen),
                    ')' => tokens.push(Symbol::RightParen),
                    '{' => tokens.push(Symbol::LeftBrace),
                    '}' => tokens.push(Symbol::RightBrace),
                    '[' => tokens.push(Symbol::LeftBracket),
                    ']' => tokens.push(Symbol::RightBracket),
                    '+' => tokens.push(Symbol::Plus),
                    '*' => tokens.push(Symbol::Star),
                    ';' => tokens.push(Symbol::SemiColon),
                    ':' => tokens.push(Symbol::Colon),
                    '=' => tokens.push(Symbol::Equal),
                    '"' => tokens.push(Symbol::Quote),
                    '\'' => tokens.push(Symbol::Tick),
                    ',' => tokens.push(Symbol::Comma),     
                    '/' => {
                        if let Some('/') = iterator.peek() {
                            iterator.next();
                            while let Some(c) = iterator.next() {
                                if c == '\n' {
                                    line += 1;
                                    break ;
                                }
                            }
                        } else if let Some('*') = iterator.peek(){
                            iterator.next();
                            while let Some(c) = iterator.next() {
                                if c == '*' {
                                    if  let Some('/') = iterator.peek() {break;}

                                }
                            }
                        } else {tokens.push(Symbol::Slash);}
                    }               
                    '-' => {
                        if let Some('>') = iterator.peek() {
                            iterator.next();
                            tokens.push(Symbol::RightArrow);
                        } else {
                            tokens.push(Symbol::Minus);
                        }
                    }
                    '>' => tokens.push(Symbol::Greater),
                    '<' => {
                        if let Some('-') = iterator.peek() {
                            iterator.next();
                            if let Some('>') = iterator.peek() {
                                tokens.push(Symbol::LeftRightArrow);
                            } else {
                                tokens.push(Symbol::LeftArrow);
                            }
                        } else {
                            tokens.push(Symbol::Less);
                        }
                    }
                    _ => {
                        if c.is_alphanumeric() {
                            let mut lexeme = String::new();
                            lexeme.push(c);
                            let mut number = c.is_ascii_digit();
                            while let Some(c) = iterator.next() {
                                if c.is_alphanumeric() {
                                    number &= c.is_ascii_digit();
                                    lexeme.push(c);
                                } else {
                                    break;
                                }
                            }
                            if number {let n :u32= lexeme.parse().expect("Couldn't parse integer."); tokens.push(Symbol::Number(n));} else {
tokens.push(Symbol::Species(lexeme));
                            }
                            
                        } else {
                            return Err(format!("Line {line}: Character not recognized {c}."));
                        }
                    }
                };
            }
        }
        Ok(Tokens { tokens })
    }

    fn print(&self) {
        for token in &self.tokens {
            println!("{token:?}");
        }
    }
}
