// standard imports
use std::fmt;
use std::error::Error;
use std::iter::Peekable;
// import grammar symbols
use crate::language::grammar;
use crate::language::grammar::{Terminal, StoichCoef};
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

// Parser struct contains syntax analysis logic
pub struct Parser<'a> {
    terminals : Scanner<'a>,
}


impl<'a> Parser<'a> {

    pub fn new(scanner : Scanner<'a>) -> Self {
        Self {terminals : scanner}
    }
    
    
    //advance to next character
//    fn pop(&mut self) -> Option<> {
//        self.terminals.next()
//    }
//
//    fn take_next_if(&mut self, func: impl FnOnce(&Terminal) -> bool) -> Option<&Terminal> {
//        self.terminals.next_if(|x| func( x.unwrap() )
//    }
//    
//    fn match_next(&mut self, func: impl FnOnce(&Terminal) -> bool) -> bool {
//        self.terminals.next_if(func).is_some()
//    }

    // grammar productions for recursive descent
//    fn reaction(&self) {
//        self.complex();
//        self.yield();
//        self.complex();
//    }
//    
//    fn yield(&self) {
//        if let Some(s) = self.take_next_if(grammar::is_yield_symbol) {
//        } else {
//        }
//    }
//
//    fn complex(&self){
//        self.monomial()
//        self.complex_r()
//    }
//    
//    fn complex_r(&self){
//        if let Some(s) = terminals.pop() {
//               
//        } 
//    }
//    fn monomial(&self) {
//        
//    }        
}
    

