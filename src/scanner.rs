// standard imports
use std::fmt;
use std::error::Error;
use std::iter::Peekable;
// declare symbols 
mod symbols;
use symbols::Symbol;
use symbols::Tokens;
use symbols::StoichCoef;

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

// preform lexical analysis; return list of tokens or LexErrpr
pub fn lexify(characters: &str) -> Result<Tokens, LexError> {
    let mut scanner = Scanner::new(characters.chars());
    let mut tokens: Tokens = Vec::new();

    while let Some(c) = scanner.pop() {
        // remove whitespace
        if c.is_whitespace() {
            if c == '\n' {
                scanner.increment_line_num();
                tokens.push(Symbol::SemiColon) // semicolon inserted at line break
            }
            continue;
        }

        if c.is_alphanumeric() {
            let s = scanner.identifier_or_number(c);
            tokens.push(s);
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
            '-' => tokens.push(scanner.rightarrow_or_minus()),
            '>' => tokens.push(Symbol::Greater),
            '<' => tokens.push(scanner.leftarrow_or_less()),
            '/' => if let Some(s) = scanner.comment_or_slash() { tokens.push(s); },
            '\"' => tokens.push(scanner.quoted_identifier()),
            _ => {
                return Err(LexError::new(
                    format!("Character not recognized {}.", c),
                    scanner.line,
                ));
            }
        } // match-arm
    } // while-loop

    Ok(tokens)
}

// Scanner class contains lexical analysis logic
pub struct Scanner<T: Iterator<Item = char>> {
    characters: Peekable<T>,
    line: LineNum,
}

impl<T: Iterator<Item = char>> Scanner<T> {
    fn new(characters: T) -> Self {
        Self {
            characters: characters.peekable(),
            line: 1,
        }
    }

    pub fn scan(characters: T) -> Self {
        Self {
            characters: characters.peekable(),
            line: 1,
        }
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

    fn comment_or_slash(&mut self) -> Option<Symbol> {
        if self.match_next(|c| *c == '/') {
            self.comment();
            None 
        } else if self.match_next(|c| *c == '*') {
            self.multline_comment();
            None 
        } else {
            Some(Symbol::Slash)
        }
    }
    fn rightarrow_or_minus(&mut self) -> Symbol {
        // arrow ?
        if self.match_next(|c| *c == '>') {
            Symbol::RightArrow
        } else {
            Symbol::Minus
        }
    }

    fn leftarrow_or_less(&mut self) -> Symbol {
        //arrow ?
        if self.match_next(|c| *c == '-') {
            // double arrow?
            if self.match_next(|c| *c == '>') {
                Symbol::LeftRightArrow
            } else {
                Symbol::LeftArrow
            }
        } else {
            Symbol::Less
        }
    }

    fn quoted_identifier(&mut self)  -> Symbol {
        let mut lexeme = String::new();
        while let Some(c) = self.pop() {
            if c == '\"' {
                break;
            }
            lexeme.push(c);
        }
        Symbol::Identifier(lexeme)
    }

    fn identifier_or_number(&mut self, c: char) -> Symbol {
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
            Symbol::Number(n)
        } else {
            Symbol::Identifier(lexeme)
        }
    }
}


impl<T : Iterator<Item = char>> Iterator for Scanner<T> {

    type Item = Result<Symbol, LexError>;

    // preform lexical analysis; return list of tokens or LexError
    fn next(&mut self) -> Option<Self::Item> {


    while let Some(c) = self.pop() {
        // remove whitespace
        if c.is_whitespace() {
            if c == '\n' {
                self.increment_line_num();
                let s = Ok(Symbol::SemiColon);
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
            '(' => Ok(Symbol::LeftParen),
            ')' => Ok(Symbol::RightParen),
            '{' => Ok(Symbol::LeftBrace),
            '}' => Ok(Symbol::RightBrace),
            '[' => Ok(Symbol::LeftBracket),
            ']' => Ok(Symbol::RightBracket),
            '+' => Ok(Symbol::Plus),
            '*' => Ok(Symbol::Star),
            ';' => Ok(Symbol::SemiColon),
            ':' => Ok(Symbol::Colon),
            '=' => Ok(Symbol::Equal),
            '\'' => Ok(Symbol::Tick),
            ',' => Ok(Symbol::Comma),
            '-' => Ok(self.rightarrow_or_minus()),
            '>' => Ok(Symbol::Greater),
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