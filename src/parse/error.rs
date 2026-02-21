#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseErrorKind {
    UnterminatedQuote,
    UnterminatedBlockComment,
    MissingLineSep,
}

#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorKind,
    file: String,
    line: usize,
    column: usize,
}

impl ParseError {

    pub fn new(kind: ParseErrorKind, file: String, line: usize, column: usize) -> Self {
        Self {kind, file, line, column}
    }
    
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "`{}` line {}, column {} :", self.file, self.line, self.column)?;
        use ParseErrorKind::*;
        match self.kind {
            UnterminatedQuote => write!(f, "literal identifier is missing closing `\"`"), 
            UnterminatedBlockComment => write!(f, "block comment is missing closing `*/`"),
            MissingLineSep => write!(f, "missing separator `;` or `newline` between reactions"),
        }
    }
}

impl std::error::Error for  ParseError {}


