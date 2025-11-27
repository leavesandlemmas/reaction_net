// standard imports
use std::error::Error;
use std::fmt;
use std::iter::Peekable;
// import grammar symbols
use crate::language::grammar;
use crate::language::grammar::Terminal;
use crate::language::scanner::{LexError, LineNum, Scanner};

// import reaction network
use crate::language::crn::{Complex, Reaction, RxNet, SpeciesRegistry};

// Errors for syntax analysis
#[derive(Debug)]
pub struct SyntaxError {
    message: String,
    line: LineNum,
}

impl SyntaxError {
    pub fn new(message: String) -> Self {
        SyntaxError { message, line: 0 }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Syntax Error on Line {}: {}", self.line, self.message)
    }
}

impl Error for SyntaxError {}

// Parser struct contains syntax analysis logic
pub struct Parser<T: Iterator<Item = Terminal>> {
    terminals: Peekable<T>,
}

impl<T: Iterator<Item = Terminal>> Parser<T> {
    //    pub fn new(scanner : Scanner<'a>) -> Self {
    //        Self {terminals : scanner}
    //    }
    pub fn new(terminals: T) -> Self {
        Self {
            terminals: terminals.peekable(),
        }
    }

    //advance to next character
    fn pop(&mut self) -> Option<Terminal> {
        self.terminals.next()
    }

    fn peek(&mut self) -> Option<&Terminal> {
        self.terminals.peek()
    }

    fn next_if(&mut self, func: impl FnOnce(&Terminal) -> bool) -> Option<Terminal> {
        self.terminals.next_if(func)
    }

    fn matches_next_if(&mut self, func: impl FnOnce(&Terminal) -> bool) -> bool {
        self.terminals.next_if(func).is_some()
    }

    fn match_peek(&mut self, symbol: Terminal) -> bool {
        self.peek() == Some(&symbol)
    }

    fn match_next(&mut self, symbol: Terminal) -> bool {
        self.matches_next_if(|x| *x == symbol)
    }

    pub fn parse(&mut self) -> Result<RxNet, SyntaxError> {
        let mut registry = SpeciesRegistry::new();
        let mut reactions: Vec<Reaction> = Vec::new();
        self.reaction_list()?;
        Ok(RxNet::make(registry, reactions))
    }
    // grammar productions for recursive descent
    fn reaction_list(&mut self) -> Result<(), SyntaxError> {
        println!("Deriving reaction_list");
        self.reaction()?;
        self.reaction_r()?;
        Ok(())
    }

    fn reaction(&mut self) -> Result<(), SyntaxError> {
        println!("Deriving reaction");
        self.complex()?;
        self.yield_symbol()?;
        self.complex()?;
        Ok(())
    }

    fn reaction_r(&mut self) -> Result<(), SyntaxError> {
        println!("Deriving reaction_r");
        if self.match_next(Terminal::SemiColon) {
            if self.peek().is_some() {
                self.reaction()?;
                self.reaction_r()?;
            }
            Ok(())
        } else {
            if let Some(s) = self.pop() {
                Err(SyntaxError::new(format!(
                    "Expected newline or ';' but found unexpected '{s:?}'"
                )))
            } else {
                Err(SyntaxError::new(
                    "Expected newline or ';' but no further input found".to_string(),
                ))
            }
        }
    }

    fn yield_symbol(&mut self) -> Result<Terminal, SyntaxError> {
        println!("Deriving yield");
        if let Some(s) = self.next_if(grammar::is_yield_symbol) {
            Ok(s)
        } else {
            Err(SyntaxError::new(
                "Expected yield symbol '->', '<-', '<->' or '='".to_string(),
            ))
        }
    }

    fn complex(&mut self) -> Result<(), SyntaxError> {
        println!("Deriving complex");
        self.monomial()?;
        self.complex_r()?;
        Ok(())
    }

    fn complex_r(&mut self) -> Result<(), SyntaxError> {
        println!("Deriving complex_r");
        if self.match_next(Terminal::Plus) {
            self.monomial()?;
            self.complex_r()?;
        }
        Ok(())
    }

    fn monomial(&mut self) -> Result<(), SyntaxError> {
        println!("Deriving monomial");
        if self.match_peek(Terminal::Number) {
            self.match_next(Terminal::Star);
        }
        self.factor()?;
        Ok(())
    }

    fn factor(&mut self) -> Result<(), SyntaxError> {
        println!("Deriving factor");
        if self.match_next(Terminal::Identifier) {
            Ok(())
        } else if self.match_next(Terminal::LeftParen) {
            self.complex()?;
            if !self.match_next(Terminal::RightParen) {
                return Err(SyntaxError::new(
                    "Unmatched parentheses. Expected ')' but found 's'".to_string(),
                ));
            }
            Ok(())
        } else {
            Err(SyntaxError::new("Factor Error.".to_string()))
        }
    }
}
