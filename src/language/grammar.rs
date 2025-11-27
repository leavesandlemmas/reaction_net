use crate::language::registry::{Registry, IdNum};

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
    Identifier,
    Number,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub symbol_type: Terminal,
    pub symbol_id: Option<IdNum>,
}

impl Token {
    pub fn new(symbol_type: Terminal) -> Self {
        Self {
            symbol_type,
            symbol_id: None,
        }
    }

    pub fn with_string(registry : &mut Registry, symbol_type: Terminal, attribute: String) -> Self {
        let id = registry.register(attribute);
        match symbol_type {
            Terminal::Identifier => Self {
                symbol_type,
                symbol_id: Some(id),
            },
            _ => Self::new(symbol_type),
        }
    }

    pub fn with_number(symbol_type: Terminal, attribute: IdNum) -> Self {
        match symbol_type {
            Terminal::Number => Self {
                symbol_type,
                symbol_id: Some(attribute),
            },
            _ => Self::new(symbol_type),
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
