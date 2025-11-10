use std::fmt;

mod symbols;

use std::error::Error;
use std::iter::Peekable;
use symbols::Symbol;
use symbols::Tokens;

#[derive(Debug)]
pub struct LexError {
    message: String,
    line: usize,
}

impl LexError {
    pub fn new(message: String, line: usize) -> Self {
        LexError { message, line }
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scanner Error on Line {}: {}", self.line, self.message)
    }
}

impl Error for LexError {}

pub fn lexify(characters: &str) -> Result<Tokens, LexError> {
    let mut scanner = Scanner::new(characters.chars());
    let mut tokens: Tokens = Vec::new();

    while let Some(c) = scanner.pop() {
        // remove whitespace
        if c.is_whitespace() {
            if c == '\n' {
                scanner.line += 1;
                tokens.push(Symbol::SemiColon) // semicolon inserted at line break
            }
            continue;
        }

        match c {
            '(' => tokens.push(Symbol::LeftParen),
            ')' => tokens.push(Symbol::RightParen),
            '{' => tokens.push(Symbol::LeftBrace),
            '}' => tokens.push(Symbol::RightBrace),
            '[' => tokens.push(Symbol::LeftBracket),
            ']' => tokens.push(Symbol::RightBracket),
            '+' => tokens.push(Symbol::Plus),
            '*' => tokens.push(Symbol::Star),
            ';' => tokens.push(Symbol::SemiColon),
            ':' => tokens.push(Symbol::Colon),
            '=' => tokens.push(Symbol::Equal),
            '\'' => tokens.push(Symbol::Tick),
            ',' => tokens.push(Symbol::Comma),
            '/' => scanner.comment_or_slash(&mut tokens),
            '-' => scanner.rightarrow_or_minus(&mut tokens),
            '>' => tokens.push(Symbol::Greater),
            '<' => scanner.leftarrow_or_less(&mut tokens),
            '\"' => scanner.quoted_identifier(&mut tokens),
            _ => {
                if c.is_alphanumeric() {
                    scanner.identifier_or_number(c, &mut tokens)
                } else {
                    return Err(LexError::new(
                        format!("Character not recognized {}.", c),
                        scanner.line,
                    ));
                }
            }
        }
    }

    Ok(tokens)
}

pub struct Scanner<T: Iterator<Item = char>> {
    characters: Peekable<T>,
    line: usize,
}

impl<T: Iterator<Item = char>> Scanner<T> {
    fn new(characters: T) -> Self {
        Self {
            characters: characters.peekable(),
            line: 1,
        }
    }

    fn increment_line_num(&mut self) {
        self.line += 1;
    }

    //returns the next character without advancing
    fn peek(&mut self) -> Option<&char> {
        self.characters.peek()
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
                self.line += 1;
                break;
            }
        }
    }

    fn multline_comment(&mut self) {
        while let Some(c) = self.pop() {
            if c == '\n' {
                self.line += 1;
            }

            if c == '*' {
                if self.match_next(|c| *c == '/') {
                    break;
                }
            }
        }
    }

    fn comment_or_slash(&mut self, tokens: &mut Tokens) {
        if self.match_next(|c| *c == '/') {
            self.comment()
        } else if self.match_next(|c| *c == '*') {
            self.multline_comment()
        } else {
            tokens.push(Symbol::Slash);
        }
    }
    fn rightarrow_or_minus(&mut self, tokens: &mut Tokens) {
        // arrow ?
        if self.match_next(|c| *c == '>') {
            tokens.push(Symbol::RightArrow);
        } else {
            tokens.push(Symbol::Minus);
        }
    }

    fn leftarrow_or_less(&mut self, tokens: &mut Tokens) {
        //arrow ?
        if self.match_next(|c| *c == '-') {
            // double arrow?
            if self.match_next(|c| *c == '>') {
                tokens.push(Symbol::LeftRightArrow);
            } else {
                tokens.push(Symbol::LeftArrow);
            }
        } else {
            tokens.push(Symbol::Less);
        }
    }

    fn quoted_identifier(&mut self, tokens: &mut Tokens) {
        let mut lexeme = String::new();
        while let Some(c) = self.pop() {
            if c == '"' {
                break;
            }
            lexeme.push(c);
        }
        tokens.push(Symbol::Identifier(lexeme));
    }

    fn identifier_or_number(&mut self, c: char, tokens: &mut Tokens) {
        let mut lexeme = String::new();
        lexeme.push(c);
        let mut number = c.is_ascii_digit();
        while let Some(c) = self.take_next_if(|c| c.is_alphanumeric()) {
            number &= c.is_ascii_digit();
            lexeme.push(c);
        }
        if number {
            let n: usize = lexeme.parse().expect("Couldn't parse integer.");
            tokens.push(Symbol::Number(n));
        } else {
            tokens.push(Symbol::Identifier(lexeme));
        }
    }
}
