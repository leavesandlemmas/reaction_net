// std lib imports
use std::error::Error;

// module declarations
mod parser;
mod scanner;
mod grammar;

// crate imports
use crate::ast;

pub fn parse(contents : &str) -> Result<ast::Network, Box<dyn Error>> {
    let scanner = scanner::Scanner::scan(&contents);
    let mut parser = parser::Parser::new(scanner);
    let out = parser.parse()?;
    Ok(out)
}
    