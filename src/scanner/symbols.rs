// symbols
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Symbol {
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
    Number(StoichCoef),
}

pub type Tokens = Vec<Symbol>;

pub type StoichCoef = u32;