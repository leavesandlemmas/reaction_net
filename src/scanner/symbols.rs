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
    Quote,
    Comma,
    Identifier(String),
    Number(usize),
}

pub type Tokens = Vec<Symbol>;
