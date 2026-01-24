// Terminal Symbols
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Terminal {
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
    Comma,
    Where,
    Newline,
    EOF,  
    Identifier(String),
    Number(i64),
}

impl Terminal {
    
    pub fn is_number(&self) -> bool {
        matches!(self, Terminal::Number(_))   
    }
    
    pub fn is_identifier(&self) -> bool {
        matches!(self, Terminal::Identifier(_))   
    }

    pub fn is_yield_symbol (&self ) -> bool {
        match self {
            Terminal::RightArrow => true,
            Terminal::LeftArrow => true,
            Terminal::LeftRightArrow => true,
            Terminal::Equal => true,
            _ => false,
        }
    }
}

use super::lexer::Span;

#[derive(Clone, Debug)]
pub struct Token {
    token: Terminal, 
    loc: Span,
}

impl Token {

    pub fn new(token : Terminal, loc : Span) -> Self {
        Self {token, loc}
    }

    pub fn as_terminal(&self) -> &Terminal {
        &self.token
    }

    pub fn to_terminal(&self) -> Terminal {
        self.token.clone()
    }

}

