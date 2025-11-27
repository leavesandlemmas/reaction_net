use crate::language::registry::Registry;

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
    symbol_type: Terminal,
    symbol_id: Option<usize>,
}

impl Token {
    pub fn new(symbol: Terminal) -> Self {
        Self {
            symbol,
            attribute: None,
        }
    }

    pub fn with_data(symbol: Terminal, attribute: SymbolData) -> Self {
        match symbol {
            Terminal::Identifier => Self {
                symbol,
                attribute: Some(value),
            },
            Terminal::Number => Self {
                symbol,
                attribute: Some(value),
            },
            _ => Self::new(symbol),
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
