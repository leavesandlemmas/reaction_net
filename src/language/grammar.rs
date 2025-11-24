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
    Number(StoichCoef)
}

pub type StoichCoef = u32;

// non-terminal grammar symbols
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Nonterminal { }


pub enum Symbol {
   Token(Terminal),
   Syntax(Nonterminal)
}

// productions are fn
