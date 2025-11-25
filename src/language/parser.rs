// standard imports
use std::fmt;
use std::error::Error;
use std::iter::Peekable;
// import grammar symbols
use crate::language::grammar::{Terminal,Nonterminal, StoichCoef};
use crate::language::scanner::{Scanner, LineNum};

// Errors for syntax analysis
#[derive(Debug)]
pub struct SyntaxError {
    message: String,
    line: LineNum,
}

impl SyntaxError {
    pub fn new(message: String, line: LineNum) -> Self {
        SyntaxError { message, line }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Syntax Error on Line {}: {}", self.line, self.message)
    }
}

impl Error for SyntaxError {}

// Parser struct contains lexical analysis logic
pub struct Parser<T : Iterator<Item = char>> {
    terminals : Scanner<T>,
}


impl<T: Iterator<Item = char>> Parser<T> {
    
}
    

