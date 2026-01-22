// standard imports
use std::error::Error;
use std::fmt;
use super::lexer::Span;

// Errors
#[derive(Debug)]
pub enum ParseError {
    Lex(LexError),
    Syntax(SyntaxError),
    //UnexpectedEOF,

}

impl ParseError {

    fn is_lex_error(&self) -> bool {
        matches!(self, ParseError::Lex(_))
    }

    fn is_syntax_error(&self) -> bool{
        matches!(self, ParseError::Syntax(_))
    }
    
}


impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Lex(e) => write!(f, "scanning input error: {}", e),
            ParseError::Syntax(e) => write!(f, "syntax error: {}", e),
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
    message: &'static str,
    loc: Span,
}

impl SyntaxError {
    pub fn new(message: &'static str, loc: Span) -> Self
    {
        SyntaxError {
            message,
            loc,
        }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.loc, self.message)
    }
}

impl Error for SyntaxError {}


// Errors for lexical analysis
#[derive(Debug)]
pub struct LexError {
    message: &'static str,
    file: String,
    line: usize,
    column: usize,
}

impl LexError {
    pub fn new(message: &'static str, file: String, line: usize, column: usize) -> Self {
        LexError { message, file, line, column}
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{}` line {}, column {}: {}", self.file, self.line, self.column, self.message)
    }
}

impl Error for LexError {}


