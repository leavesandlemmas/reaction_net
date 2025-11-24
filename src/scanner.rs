// standard imports
use std::fmt;
use std::error::Error;
use std::iter::Peekable;
// import terminal symbols
use crate::language::grammar::Terminal;
use crate::language::grammar::StoichCoef;

type LineNum = u32;

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

// Scanner class contains lexical analysis logic
pub struct Scanner<T: Iterator<Item = char>> {
    characters: Peekable<T>,
    line: LineNum
}

impl<T: Iterator<Item = char>> Scanner<T> {
    fn new(characters: T) -> Self {
        Self {
            characters: characters.peekable(),
            line: 1
        }
    }

    pub fn scan(characters: T) -> Self {
        Self::new(characters)
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

    fn comment_or_slash(&mut self) -> Option<Terminal> {
        if self.match_next(|c| *c == '/') {
            self.comment();
            None 
        } else if self.match_next(|c| *c == '*') {
            self.multline_comment();
            None 
        } else {
            Some(Terminal::Slash)
        }
    }
    fn rightarrow_or_minus(&mut self) -> Terminal {
        // arrow ?
        if self.match_next(|c| *c == '>') {
            Terminal::RightArrow
        } else {
            Terminal::Minus
        }
    }

    fn leftarrow_or_less(&mut self) -> Terminal {
        //arrow ?
        if self.match_next(|c| *c == '-') {
            // double arrow?
            if self.match_next(|c| *c == '>') {
                Terminal::LeftRightArrow
            } else {
                Terminal::LeftArrow
            }
        } else {
            Terminal::Less
        }
    }

    fn quoted_identifier(&mut self)  -> Terminal {
        let mut lexeme = String::new();
        while let Some(c) = self.pop() {
            if c == '\"' {
                break;
            }
            lexeme.push(c);
        }
        Terminal::Identifier(lexeme)
    }

    fn identifier_or_number(&mut self, c: char) -> Terminal {
        let mut lexeme = String::new();
        lexeme.push(c);
        let mut number = c.is_ascii_digit();
        while let Some(c) = self.take_next_if(|c| c.is_alphanumeric()) {
            number &= c.is_ascii_digit();
            lexeme.push(c);
        }
        if number {
            let n: StoichCoef = lexeme.parse().expect(
                "Couldn't parse stoichiometric coefficient as integer.");
            Terminal::Number(n)
        } else {
            Terminal::Identifier(lexeme)
        }
    }
}


impl<T : Iterator<Item = char>> Iterator for Scanner<T> {

    type Item = Result<Terminal, LexError>;

    // preform lexical analysis; return list of tokens or LexError
    fn next(&mut self) -> Option<Self::Item> {


    while let Some(c) = self.pop() {
        // remove whitespace
        if c.is_whitespace() {
            if c == '\n' {
                self.increment_line_num();
                let s = Ok(Terminal::SemiColon);
                return Some(s) // semicolon inserted at line break
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
            return Some(Ok(s))
        }

        let result = match c {
            '(' => Ok(Terminal::LeftParen),
            ')' => Ok(Terminal::RightParen),
            '{' => Ok(Terminal::LeftBrace),
            '}' => Ok(Terminal::RightBrace),
            '[' => Ok(Terminal::LeftBracket),
            ']' => Ok(Terminal::RightBracket),
            '+' => Ok(Terminal::Plus),
            '*' => Ok(Terminal::Star),
            ';' => Ok(Terminal::SemiColon),
            ':' => Ok(Terminal::Colon),
            '=' => Ok(Terminal::Equal),
            '\'' => Ok(Terminal::Tick),
            ',' => Ok(Terminal::Comma),
            '-' => Ok(self.rightarrow_or_minus()),
            '>' => Ok(Terminal::Greater),
            '<' => Ok(self.leftarrow_or_less()),
            '\"' => Ok(self.quoted_identifier()),
            _ => {
                Err(LexError::new(
                    format!("Character not recognized {}.", c),
                    self.line,
                ))
            }
        }; // match-arm
        return Some(result);
    }
    None 

}
}
