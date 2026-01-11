// standard imports
use std::error::Error;
use std::fmt;
// import grammar symbols
use super::grammar;
use super::grammar::Terminal;
use super::scanner::{LexError, LineNum, Scanner};

// import reaction network ast
use crate::ast;

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

// Errors
#[derive(Debug)]
pub enum ParseError {
    Lex(LexError),
    Syntax(SyntaxError),
    UnexpectedEOF,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Lex(e) => write!(f, "Scanning Error: {}", e),
            ParseError::Syntax(e) => write!(f, "Syntax Error: {}", e),
            ParseError::UnexpectedEOF => write!(f, "Unexpected enf of input"),
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

// Parser struct contains syntax analysis logic
pub struct Parser<'a> {
    scanner: Scanner<'a>,
    lookahead: Option<Terminal>,
}

type Maybe<T> = Result<Option<T>, ParseError>;

impl<'a> Parser<'a> {
    pub fn new(scanner: Scanner<'a>) -> Self {
        Self {
            scanner,
            lookahead: None,
        }
    }

    // actions for token stream
    // advance to next character
    fn pop_token(&mut self) -> Maybe<Terminal> {
        // check lookahead buffer first
        if let Some(token) = self.lookahead.take() {
            return Ok(Some(token));
        }

        // pop next token and handle lex error
        match self.scanner.next() {
            Some(Ok(token)) => Ok(Some(token)),
            Some(Err(e)) => Err(ParseError::Lex(e)),
            None => Ok(None),
        }
    }

    // look at next character without consuming
    fn peek_token(&mut self) -> Maybe<&Terminal> {
        // check buffer
        if self.lookahead.is_some() {
            return Ok(self.lookahead.as_ref());
        }
        // if buffer is none, then pop and put into buffer
        // handling errors
        match self.scanner.next() {
            Some(Ok(token)) => {
                self.lookahead = Some(token);
                Ok(self.lookahead.as_ref())
            }
            Some(Err(e)) => Err(ParseError::Lex(e)),
            None => Ok(None),
        }
    }

    // advance to next character if next token satisfies predicate
    fn next_if(&mut self, predicate: impl FnOnce(&Terminal) -> bool) -> Maybe<Terminal> {
        if self.peek_if(predicate) {
            self.pop_token()
        } else {
            Ok(None)
        }
    }

    // check if next token satisfies predicate
    fn peek_if(&mut self, predicate: impl FnOnce(&Terminal) -> bool) -> bool {
        let m = self.peek_token();
        if let Ok(Some(token)) = self.peek_token() {
            predicate(&token)
        } else {
            false
        }
    }

    // check if next token matches without consuming
    fn peek_if_match(&mut self, symbol: Terminal) -> bool {
        self.peek_if(|x: &Terminal| *x == symbol)
    }

    // check if next token matches; consume if yes
    fn next_if_match(&mut self, symbol: Terminal) -> Maybe<Terminal> {
        self.next_if(|x: &Terminal| *x == symbol)
    }

    // check if next token matches; consume if yes
    fn advance_if_match(&mut self, symbol: Terminal) -> bool {
        let matched = self.peek_if_match(symbol);
        if matched {
            let _ = self.pop_token();
            true
        } else {
            false
        }
    }

    fn emit_error<S, E>(&self, msg: S) -> Result<E, ParseError>
    where
        S: Into<String> + AsRef<str>,
    {
        let e = SyntaxError::new(msg, self.scanner.get_line_num());
        Err(ParseError::Syntax(e))
    }

    // build AST for CRN from recursiving descent parsing
    pub fn parse(&mut self) -> Result<ast::Network, ParseError> {
        let network = self.reaction_list()?;
        Ok(network)
    }

    // grammar productions for recursive descent
    fn reaction_list(&mut self) -> Result<ast::Network, ParseError> {
        let mut network = ast::Network::new();
        let rxn = self.reaction()?;
        network.add_reaction(rxn);
        let network = self.next_reaction(network)?;
        Ok(network)
    }

    fn reaction(&mut self) -> Result<ast::Reaction, ParseError> {

        let left = self.complex()?;
        let arr = self.yield_symbol()?;
        let right = self.complex()?;
        let rxn = ast::Reaction::new(None, arr, left, right);
        Ok(rxn)
    }

    fn next_reaction(&mut self, mut network : ast::Network) -> Result<ast::Network, ParseError> {
        if self.advance_if_match(Terminal::SemiColon) {
            if self.peek_if(|x| *x != Terminal::SemiColon) {
                let rxn = self.reaction()?;
                network.add_reaction(rxn);
            }
            let next = self.peek_token()?;
            if next.is_some() {
                return self.next_reaction(network);
            }
            Ok(network)
        } else {
            let msg = format!("Expected newline or ';' but found unexpected");
            self.emit_error(msg)
        }
    }

    fn yield_symbol(&mut self) -> Result<ast::Arrow, ParseError> {
        let maybe_token = self.next_if(grammar::is_yield_symbol)?;
        if let Some(s) = maybe_token {
            let arr = match s {
            Terminal::RightArrow => ast::Arrow::Right,
            Terminal::LeftArrow => ast::Arrow::Left,
            Terminal::LeftRightArrow => ast::Arrow::Reversible,
            Terminal::Equal => ast::Arrow::Reversible,
            _ => panic!("`yield_symbol()` returned terminal that was not an arrow."),
            };
            Ok(arr)
        } else {
            self.emit_error("Expected yield symbol '->', '<-', '<->' or '='")
        }
    }

    fn complex(&mut self) -> Result<ast::Complex, ParseError> {
        let mut cplx = ast::Complex::new();
        let term = self.monomial()?;
        cplx.add_term(term);
        let cplx = self.next_monomial(cplx)?;
        Ok(cplx)
    }

    fn next_monomial(&mut self, mut cplx: ast::Complex) -> Result<ast::Complex, ParseError> {
        if self.advance_if_match(Terminal::Plus) {
            let term = self.monomial()?;
            cplx.add_term(term);
            return self.next_monomial(cplx);
        }
        Ok(cplx)
    }

    fn monomial(&mut self) -> Result<(String, u64), ParseError> {
        let coef = if self.peek_if(|x| x.is_number()) {
            let Some(Terminal::Number(coef)) = self.pop_token()? else {
                panic!("Could't unwrap Number")
            };
            self.advance_if_match(Terminal::Star);
            coef
        } else {
            1
        };
        let species = self.species()?;
        Ok((species, coef))
    }

    fn species(&mut self) -> Result<String, ParseError> {
        let maybe_token = self.peek_token()?;
        if self.peek_if(|x| x.is_identifier()) {
            let Some(Terminal::Identifier(sp)) = self.pop_token()? else {
                panic!("Couldn't unwrap Identifier!")
            };
            Ok(sp)
        //        } else if self.advance_if_match(Terminal::LeftParen) {
        //            self.complex()?;
        //            if !self.advance_if_match(Terminal::RightParen) {
        //                return Self::emit_error("Unmatched parentheses. Expected ')' but found 's'");
        //            }
        //            Ok(())
        } else {
            self.emit_error("Factor Error.")
        }
    }
}
