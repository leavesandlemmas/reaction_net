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

    pub fn unwrap_number(self) -> u64 {
    
    } 
    
    pub fn unwrap_identifier(self) -> Result<String, Error> {
        match self {
            Terminal::Identifier(s) => Ok(s),
            _ => Err(),
        }
    }
}

//#[derive(Debug, Clone)]
//pub struct Token {
//    symbol_type: Terminal,
//    symbol_id: Option<Attribute>,
//}
//
//impl Token {
//    pub fn new(symbol_type: Terminal) -> Self {
//        Self {
//            symbol_type,
//            symbol_id: None,
//        }
//    }
//
//    pub fn identifier(lexeme : String) -> Self {
//        Self {symbol_type : Terminal::Identifier, symbol_id : Some (Attribute::Id(lexeme))}
//    }
//        
//    
//    pub fn number(num : u64) -> Self {
//        Self {symbol_type : Terminal::Identifier, symbol_id : Some (Attribute::Num(num))}
//    }
//
//}
//
//
//#[derive(Debug)]
//enum Attribute {
//    Num(u64),
//    Id(String),
//}
//
//

pub fn is_yield_symbol(s: &Terminal) -> bool {
    match s {
        Terminal::RightArrow => true,
        Terminal::LeftArrow => true,
        Terminal::LeftRightArrow => true,
        Terminal::Equal => true,
        _ => false,
    }
}
