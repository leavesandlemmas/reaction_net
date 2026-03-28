// standard imports
use std::iter::Peekable;
use std::str::Chars;

// import terminal symbols
use super::grammar::Terminal;
use super::LineNum;
use super::error::LexError;



// Scanner contains lexical analysis logic
pub struct Scanner<I, T>
where
    I: Iterator<Item = T>,
{
    characters: Peekable<I>,
    line: LineNum,
}

impl<'a> Scanner<Chars<'a>, char>
{
    pub fn scan(source: &'a str) -> Self {
        Self::new(source.chars())
    }
}

impl<I> Scanner<I, char>
where
    I: Iterator<Item = char>,
{
    pub fn new(source: I) -> Self {
        Self {
            characters: source.peekable(),
            line: 1,
        }
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

    fn comment_or_slash(&mut self) -> Option<Result<Terminal, LexError>> {
        if self.match_next(|c| *c == '/') {
            self.comment();
            None
        } else if self.match_next(|c| *c == '*') {
            self.multline_comment();
            None
        } else {
            Some(Ok(Terminal::Slash))
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

    fn quoted_identifier(&mut self) -> Terminal {
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
        while let Some(c) = self.take_next_if(|c| c.is_alphanumeric()) {
            lexeme.push(c);
        }
        let maybe_number = lexeme.parse::<u64>();
        match maybe_number {
            Ok(n) => Terminal::Number(n),
            _ => Terminal::Identifier(lexeme),
        }
    }
}

impl<I> Iterator for Scanner<I, char>
where
    I: Iterator<Item = char>,
{
    type Item = Result<Terminal, LexError>;

    // preform lexical analysis; return list of tokens or LexError
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.pop() {
            // remove whitespace
            if c.is_whitespace() {
                if c == '\n' {
                    self.increment_line_num();
                    let t = Ok(Terminal::SemiColon);
                    return Some(t); // semicolon inserted at line break
                }
                continue;
            }

            // remove comments
            if c == '/' {
                let maybe_slash = self.comment_or_slash();
                if maybe_slash.is_some() {
                    return maybe_slash;
                }
                continue;
            }

            if c.is_alphanumeric() {
                let t = self.identifier_or_number(c);
                return Some(Ok(t));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_input() {
        let source = "A + B -> C + D";
        let tokens : Vec<Terminal> = Scanner::scan(source)
            .take_while(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect();
        assert_eq!(tokens, vec![
            Terminal::Identifier("A".to_string()),
            Terminal::Plus,
            Terminal::Identifier("B".to_string()),
            Terminal::RightArrow,
            Terminal::Identifier("C".to_string()),
            Terminal::Plus,
            Terminal::Identifier("D".to_string())            
            ])

    }
}
