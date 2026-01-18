// Terminal Symbols
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Terminal<'lex> {
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
    Newline,
    EndOfFile,  
    Identifier(&'lex str),
    StoichCoef(u64),
}

impl<'lex> Terminal<'lex> {
    pub fn is_coef(&self) -> bool {
        matches!(*self, Terminal::StoichCoef(_))
    }

    pub fn is_identifier(&self) -> bool {
        matches!(*self, Terminal::Identifier(_))
    }
    
}

pub fn is_yield_symbol<'lex>(s: &'lex Terminal) -> bool {
    match s {
        Terminal::RightArrow => true,
        Terminal::LeftArrow => true,
        Terminal::LeftRightArrow => true,
        Terminal::Equal => true,
        _ => false,
    }
}
