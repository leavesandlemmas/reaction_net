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
    Number
}

pub type StoichCoef = u32;


pub fn is_yield_symbol(s : &Terminal) -> bool {
    match s {
        Terminal::RightArrow => true,
        Terminal::LeftArrow => true,
        Terminal::LeftRightArrow => true,
        Terminal::Equal => true,
        _ => false
    }
}
