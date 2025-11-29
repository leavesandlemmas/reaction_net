// standard imports
use std::error::Error;
use std::fmt;
// import grammar symbols
use crate::language::grammar;
use crate::language::grammar::{Terminal, Token};
use crate::language::scanner::{LexError, LineNum, Scanner};

// import reaction network

// Errors for syntax analysis
#[derive(Debug)]
pub struct SyntaxError
{
    message: String,
    line: LineNum,
}

impl SyntaxError {
    pub fn new<S>(message: S) -> Self 
    where S : Into<String> + AsRef<str>
    {
        SyntaxError { message : message.into(), line: 0 }
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
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Lex(e) => write!(f, "Scanning Error: {}", e),
            ParseError::Syntax(e) => write!(f, "Syntax Error: {}", e),
            ParseError::UnexpectedEOF => write!(f, "Unexpected enf of input"),
        }
    }
}

impl Error for ParseError {}

impl From<LexError> for ParseError {
    fn from (e : LexError) -> Self {
        ParseError::Lex(e) 
    }
}

impl From<SyntaxError> for ParseError {
    fn from (e : SyntaxError) -> Self {
        ParseError::Syntax(e) 
    }
}

// Parser struct contains syntax analysis logic
pub struct Parser<'a> {
    scanner : Scanner <'a>,
    lookahead : Option<Token>,
}

type Maybe<T> = Result<Option<T>, ParseError>;

impl<'a> Parser<'a> {
  
    
    pub fn new(scanner : Scanner<'a>) -> Self {
        Self {
            scanner,
            lookahead : None, 
        }
    }
//    pub fn new(terminals: T) -> Self {
//        Self {
//            terminals: terminals.peekable(),
//        }
//    }
    
    // actions for token stream 
    // advance to next character
    fn pop_token(&mut self) -> Maybe<Token> {
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
    fn peek_token(&mut self) -> Maybe<&Token> {
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
    fn next_if(&mut self, predicate: impl FnOnce(&Terminal) -> bool) -> Maybe<Token> {
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
            predicate(&token.symbol_type)
        } else {
            false
        }
    }
    
    // check if next token matches without consuming
    fn peek_if_match(&mut self, symbol: Terminal) -> bool {
        self.peek_if(|x : &Terminal| *x == symbol)       
    }

    // check if next token matches; consume if yes 
    fn next_if_match(&mut self, symbol: Terminal) -> Maybe<Token> {
        self.next_if(|x : &Terminal| *x == symbol)
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

    fn emit_error<S, E>(msg : S) -> Result<E, ParseError>
    where
        S: Into<String> + AsRef<str>,
    {
        let e = SyntaxError::new(msg);
        Err(ParseError::Syntax(e))
    }



    // build CRN from recursiving descent parsing
    pub fn parse(&mut self) -> Result<(), ParseError> {
        self.reaction_list()?;
        Ok(())
    }

    // grammar productions for recursive descent
    fn reaction_list(&mut self) -> Result<(), ParseError> {
        println!("Deriving reaction_list");
        self.reaction()?;
        self.next_reaction()?;
        Ok(())
    }

    fn reaction(&mut self) -> Result<(), ParseError> {
        println!("Deriving reaction");
        
        let left_cplx = self.complex()?;
        let y = self.yield_symbol()?;
        let right_cplx = self.complex()?;
//        let rxn = match y {
//            Terminal::RightArrow => Reaction::forward(left_cplx, right_cplx),
//            Terminal::LeftArrow => Reaction::forward(right_cplx, left_cplx),
//            Terminal::LeftRightArrow => Reaction::reversible(left_cplx, right_cplx),
//            Terminal::Equal => Reaction::reversible(left_cplx, right_cplx),
//        };
//        crn.add_reaction(rxn);
        Ok(())
    }

    fn next_reaction(&mut self) -> Result<(), ParseError> {
        println!("Deriving next_reaction");
        if self.advance_if_match(Terminal::SemiColon) {
            let next = self.peek_token()?; 
            if next.is_some() {
                self.reaction()?;
                self.next_reaction()?;
            }
            Ok(())
        } else {
            let msg = format!(
                    "Expected newline or ';' but found unexpected");
           Self::emit_error(msg)
          }
    }

    fn yield_symbol(&mut self) -> Result<Terminal, ParseError> {
        println!("Deriving yield");
        let maybe_token = self.next_if(grammar::is_yield_symbol)?;
        if let Some(s) = maybe_token {
            Ok(s.symbol_type)
        } else {
            Self::emit_error("Expected yield symbol '->', '<-', '<->' or '='")
        }
    }

    fn complex(&mut self) -> Result<(), ParseError> {
        println!("Deriving complex");
//        let cplx = Complex::new();
        self.monomial()?;
        self.next_monomial()?;
        Ok(())
    }

    fn next_monomial(&mut self) -> Result<(), ParseError> {
        println!("Deriving next_monomial");
        if self.advance_if_match(Terminal::Plus) {
            self.monomial()?;
            self.next_monomial()?;
        }
        Ok(())
    }

    fn monomial(&mut self) -> Result<(), ParseError> {
        println!("Deriving monomial");
        if self.advance_if_match(Terminal::Number) {
            self.advance_if_match(Terminal::Star);
        }
        self.species()?;
        Ok(())
    }

    fn species(&mut self) -> Result<(), ParseError> {
        println!("Deriving species");
        if self.peek_if_match(Terminal::Identifier) {
            let token = self.pop_token();
            //crn.register_species(token.attribute.unwrap());
            Ok(())
        } else if self.advance_if_match(Terminal::LeftParen) {
            self.complex()?;
            if !self.advance_if_match(Terminal::RightParen) {
                return Self::emit_error("Unmatched parentheses. Expected ')' but found 's'");
            }
            Ok(())
        } else {
            Self::emit_error("Factor Error.")
        }
    }
}
