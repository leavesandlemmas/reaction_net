// standard imports
use std::error::Error;
use std::fmt;
use std::iter::Peekable;
use std::str::Chars;
// import terminal symbols
use crate::language::grammar::{SymbolData, Terminal, Token};

pub type LineNum = u32;

// Errors for lexical analysis
#[derive(Debug)]
pub struct LexError {
    message: String,
    line: LineNum,
}

impl LexError {
    pub fn new(message: String, line: LineNum) -> Self {
        LexError { message, line }
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scanner Error on Line {}: {}", self.line, self.message)
    }
}

impl Error for LexError {}

pub type ScanResult = Result<Token, LexError>;

// Scanner contains lexical analysis logic
pub struct Scanner<'a> {
    characters: Peekable<Chars<'a>>,
    line: LineNum,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            characters: source.chars().peekable(),
            line: 1,
        }
    }

    pub fn scan(source: &'a str) -> Self {
        Self::new(source)
    }

    pub fn get_line_num(&self) -> LineNum {
        self.line
    }

    fn increment_line_num(&mut self) {
        self.line += 1;
    }

    //advance to next character
    fn pop(&mut self) -> Option<char> {
        self.characters.next()
    }

    fn take_next_if(&mut self, func: impl FnOnce(&char) -> bool) -> Option<char> {
        self.characters.next_if(func)
    }

    fn match_next(&mut self, func: impl FnOnce(&char) -> bool) -> bool {
        self.characters.next_if(func).is_some()
    }

    fn comment(&mut self) {
        while let Some(c) = self.pop() {
            if c == '\n' {
                self.increment_line_num();
                break;
            }
        }
    }

    fn multline_comment(&mut self) {
        while let Some(c) = self.pop() {
            if c == '\n' {
                self.increment_line_num();
            }

            if c == '*' {
                if self.match_next(|c| *c == '/') {
                    break;
                }
            }
        }
    }

    fn comment_or_slash(&mut self) -> Option<Token> {
        if self.match_next(|c| *c == '/') {
            self.comment();
            None
        } else if self.match_next(|c| *c == '*') {
            self.multline_comment();
            None
        } else {
            Some(Token::new(Terminal::Slash))
        }
    }
    fn rightarrow_or_minus(&mut self) -> Token {
        // arrow ?
        if self.match_next(|c| *c == '>') {
            Token::new(Terminal::RightArrow)
        } else {
            Token::new(Terminal::Minus)
        }
    }

    fn leftarrow_or_less(&mut self) -> Token {
        //arrow ?
        if self.match_next(|c| *c == '-') {
            // double arrow?
            if self.match_next(|c| *c == '>') {
                Token::new(Terminal::LeftRightArrow)
            } else {
                Token::new(Terminal::LeftArrow)
            }
        } else {
            Token::new(Terminal::Less)
        }
    }

    fn quoted_identifier(&mut self) -> Token {
        let mut lexeme = String::new();
        while let Some(c) = self.pop() {
            if c == '\"' {
                break;
            }
            lexeme.push(c);
        }
        Token::with_data(Terminal::Identifier, SymbolData::Identifier(lexeme))
    }

    fn identifier_or_number(&mut self, c: char) -> Token {
        let mut lexeme = String::new();
        lexeme.push(c);
        let mut number = c.is_ascii_digit();
        while let Some(c) = self.take_next_if(|c| c.is_alphanumeric()) {
            number &= c.is_ascii_digit();
            lexeme.push(c);
        }
        if number {
            let n: u64 = lexeme
                .parse()
                .expect("Couldn't parse stoichiometric coefficient as integer.");
            Token::with_data(Terminal::Number, SymbolData::Number(n))
        } else {
            Token::with_data(Terminal::Identifier, SymbolData::Identifier(lexeme))
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = ScanResult;

    // preform lexical analysis; return list of tokens or LexError
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.pop() {
            // remove whitespace
            if c.is_whitespace() {
                if c == '\n' {
                    self.increment_line_num();
                    let s = Ok(Token::new(Terminal::SemiColon));
                    return Some(s); // semicolon inserted at line break
                }
                continue;
            }

            // remove comments
            if c == '/' {
                let maybe_slash = self.comment_or_slash();
                if maybe_slash.is_some() {
                    let s = Ok(maybe_slash?);
                    return Some(s);
                }
                continue;
            }

            if c.is_alphanumeric() {
                let s = self.identifier_or_number(c);
                return Some(Ok(s));
            }

            let result = match c {
                '(' => Ok(Token::new(Terminal::LeftParen)),
                ')' => Ok(Token::new(Terminal::RightParen)),
                '{' => Ok(Token::new(Terminal::LeftBrace)),
                '}' => Ok(Token::new(Terminal::RightBrace)),
                '[' => Ok(Token::new(Terminal::LeftBracket)),
                ']' => Ok(Token::new(Terminal::RightBracket)),
                '+' => Ok(Token::new(Terminal::Plus)),
                '*' => Ok(Token::new(Terminal::Star)),
                ';' => Ok(Token::new(Terminal::SemiColon)),
                ':' => Ok(Token::new(Terminal::Colon)),
                '=' => Ok(Token::new(Terminal::Equal)),
                '\'' => Ok(Token::new(Terminal::Tick)),
                ',' => Ok(Token::new(Terminal::Comma)),
                '-' => Ok(self.rightarrow_or_minus()),
                '>' => Ok(Token::new(Terminal::Greater)),
                '<' => Ok(self.leftarrow_or_less()),
                '\"' => Ok(self.quoted_identifier()),
                _ => Err(LexError::new(
                    format!("Character not recognized {}.", c),
                    self.line,
                )),
            }; // match-arm
            return Some(result);
        }
        None
    }
}
