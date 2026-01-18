use std::iter::Peekable;
use std::str::Chars;
use super::grammar::Terminal; 

struct Lexer<'lex> {
    source : &'lex str,
    characters : Peekable<Chars<'lex>>,
    lookahead: Option<Terminal<'lex>>,
    lexeme : Lexeme,
}

impl<'lex> Lexer<'lex> {

    fn new(source : &'lex str) -> Self {
        let characters = source.chars().peekable();
        let lexeme = Lexeme::new();
        Self {source, characters, lookahead : None , lexeme}
    }

    // consume one terminal
    pub fn pop(&mut self) -> Option<Terminal<'lex>> {
        // loop {
        //     advance

        // }
        // Terminal::EndOfFile
        todo!()
    }

    // lookahead at next termina
    pub fn peek(&mut self) -> Option<Terminal<'lex>> {
        // self.tokens.last().copied().unwrap_or(Token::Eof)
        // if self.lookahead.is_none() {
        //     let t = self.pop();
        //     self.lookahead = Some(t);
        //     return t;
        // } 
        // self.lookahead
        todo!()
    }

    // next character
    fn advance(&mut self) -> Option<char> {
        self.characters.next()
    }

    // next character
    fn check(&mut self) -> Option<char> {
        // if let Some((idx, ch)) = self.characters.next() {
        //     self.cursor = idx;
        //     Some(ch)
        // } else {
        //     None
        // }
        todo!()
    }

    
    // turn a character into a terminal
    fn lexify(&mut self, c : char) -> Terminal<'lex> {
        loop {
        // remove whitespace
            if c.is_whitespace() {
                
                if c == '\n' {
                    self.increment_line_num();
                    let t = Ok(Terminal::SemiColon);
                    return Some(t); // semicolon inserted at line break
                }
                continue;
            }

            match c {
                '(' => return Terminal::LeftParen,
                ')' => Terminal::RightParen,
                '{' => Terminal::LeftBrace,
                '}' => Terminal::RightBrace,
                '[' => Terminal::LeftBracket,
                ']' => Terminal::RightBracket,
                '+' => Terminal::Plus,
                '*' => Terminal::Star,
                ';' => Terminal::SemiColon,
                ':' => Terminal::Colon,
                '=' => Terminal::Equal,
                '/' => {
                    if let Some(&ch) = self.characters.peek() {
                        if (ch == '/') {
                            loop { 
                                if self.characters.next_if(|c| *c != '\n').is_none() {
                                    break;
                                } 

                            }
                        }
                    }
                },
                '\'' => Terminal::Tick,
                ',' => Terminal::Comma,
                '-' => self.minus_or_rightarrow(),
                '>' => Terminal::Greater,
                '<' => self.less_or_arrow(),
                '\"' => self.quoted_identifier(),
                _ => panic!("Bad Terminal"),        
            }
        }
    }

    fn slash_or_comment(&mut self) {
        let next_char = self.characters.peek();
        match next_char {
            Some('/') => self.comment(), 
            Some('*') => self.multiline_comment(),
            _ => ,
        }
    }
    
    // fn minus_or_rightarrow(&mut self) -> Terminal {
    //     // arrow ?
    //     if self.characters.next_if(|c| *c == '>') {
    //         Terminal::RightArrow
    //     } else {
    //         Terminal::Minus
    //     }
    // }

    // fn less_or_arrow(&mut self) -> Terminal {
    //     //arrow ?
    //     if self.characters.next_if(|c| *c == '-') {
    //         // double arrow?
    //         if self.characters.next_if(|c| *c == '>') {
    //             Terminal::LeftRightArrow
    //         } else {
    //             Terminal::LeftArrow
    //         }
    //     } else {
    //         Terminal::Less
    //     }
    // }

    // fn quoted_identifier(&mut self) -> Terminal {
    //     let mut lexeme = String::new();
    //     while let Some(c) = self.pop() {
    //         if c == '\"' {
    //             break;
    //         }
    //         lexeme.push(c);
    //     }
    //     Terminal::Identifier(lexeme)
    // }

    // fn identifier_or_number(&mut self, c: char) -> Terminal {
    //     let mut lexeme = String::new();
    //     lexeme.push(c);
    //     while let Some(c) = self.take_next_if(|c| c.is_alphanumeric()) {
    //         lexeme.push(c);
    //     }
    //     let maybe_number = lexeme.parse::<u64>();
    //     match maybe_number {
    //         Ok(n) => Terminal::Number(n),
    //         _ => Terminal::Identifier(lexeme),
    //     }
    // }
}
// refers a lexeme in the source
struct Lexeme {
    start : usize,
    end : usize,
}


impl Lexeme {

    fn new() -> Self {
        Self {start : 0, end : 0}
    }

    fn advance_and_clear(&mut self) {
        self.start = self.end;
    } 

    fn add_char(&mut self, ch : char) {
        self.end += ch.len_utf8();
    }

    fn show<'a>(&self, source : &'a str) -> &'a str {
        &source[self.start..self.end]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_lexer_advance(){
        let source = "A + B";
        let mut lexer = Lexer::new(&source);
        let maybe_ch = lexer.advance();
        assert_eq!(maybe_ch, Some('A'));

        let maybe_ch = lexer.advance();
        assert_eq!(maybe_ch, Some(' '));

        
        let maybe_ch = lexer.advance();
        assert_eq!(maybe_ch, Some('+'));

        
        let maybe_ch = lexer.advance();
        assert_eq!(maybe_ch, Some(' '));

        
        let maybe_ch = lexer.advance();
        assert_eq!(maybe_ch, Some('B'));
    }

}