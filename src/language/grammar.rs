use std::error::Error;

// Terminal Symbols
#[derive(Debug, PartialEq, Eq, Clone)]
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
    Identifier(String),
    Number(u64),
}

impl Terminal {
    
    pub fn is_number(&self) -> bool {
        matches!(*self, Terminal::Number(_))
    } 
    
    pub fn is_identifier(&self) -> bool {
        matches!(*self, Terminal::Identifier(_))
    }

    pub fn get_number(self) -> u64 {
         match self {
            Terminal::Number(n) => n,
            _ => panic!("Cannot get_number fr"),
        }    
    } 
    
    pub fn get_identifier(self) -> Option<String> {
        match self {
            Terminal::Identifier(s) => Some(s),
            _ => None,
        }
    }
}


pub fn is_yield_symbol(s: &Terminal) -> bool {
    match s {
        Terminal::RightArrow => true,
        Terminal::LeftArrow => true,
        Terminal::LeftRightArrow => true,
        Terminal::Equal => true,
        _ => false,
    }
}
