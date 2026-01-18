// standard imports
use std::error::Error;
use std::fmt;
use super::LineNum;

// Errors
#[derive(Debug)]
pub enum ParseError {
    Lex(LexError),
    Syntax(SyntaxError),
    //UnexpectedEOF,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Lex(e) => write!(f, "Scanning Error: {}", e),
            ParseError::Syntax(e) => write!(f, "Syntax Error: {}", e),
      //      ParseError::UnexpectedEOF => write!(f, "Unexpected end of input"),
        }
    }
}

impl Error for ParseError {}

impl From<LexError> for ParseError {
    fn from(e: LexError) -> Self {
        ParseError::Lex(e)
    }
}

impl From<SyntaxError> for ParseError {
    fn from(e: SyntaxError) -> Self {
        ParseError::Syntax(e)
    }
}

// Errors for syntax analysis
#[derive(Debug)]
pub struct SyntaxError {
    message: String,
    line: LineNum,
}

impl SyntaxError {
    pub fn new<S>(message: S, line : LineNum) -> Self
    where
        S: Into<String> + AsRef<str>,
    {
        SyntaxError {
            message: message.into(),
            line,
        }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Syntax Error on Line {}: {}", self.line, self.message)
    }
}

impl Error for SyntaxError {}


// Errors for lexical analysis
#[derive(Debug)]
pub struct LexError {
    message: String,
    line: LineNum,
}

impl LexError {
    pub fn new(message: String, line: LineNum) -> Self {
        LexError { message, line }
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error on Line {}: {}", self.line, self.message)
    }
}

impl Error for LexError {}


