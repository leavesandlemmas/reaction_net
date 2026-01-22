use super::tokens::{Terminal, Token};
use std::iter::Peekable;
use std::str::Chars;
use std::fmt;
use crate::error::LexError;


pub struct Lexer<I>   
where 
    I : Iterator<Item = char> 
{
    characters: Peekable<I>,
    loc: Span,
    lexeme : String,   
    eof: bool,
}

impl<I> Lexer<I>
where
    I : Iterator<Item = char>
{

    pub fn new(characters: I) -> Self {
        Self {
            characters : characters.peekable(),
            loc : Span::new(String::new()),
            lexeme : String::with_capacity(16),
            eof: false,
        }
    }
    
    pub fn with_name(characters: I, name: String) -> Self {
        Self {
            characters : characters.peekable(),
            loc: Span::new(name),
            lexeme: String::with_capacity(16),
            eof: false,
        }
    }
    
    // character stream methods
    // advance = pop: consume one character
    fn advance(&mut self) -> Option<char> {
        let next_char = self.characters.next();
        if let Some(ch) = next_char {
            self.loc.advance(ch);
            self.lexeme.push(ch);
        }
        next_char
    }

    fn advance_if(&mut self, func: impl FnOnce(&char) -> bool) -> Option<char> {
        let next_char = self.characters.next_if(func);
        if let Some(ch) = next_char {
            self.loc.advance(ch);
            self.lexeme.push(ch);
        }
        next_char
    }

    fn advance_if_eq(&mut self, ch: &char) -> Option<char> {
        let next_char = self.characters.next_if_eq(ch);
        if let Some(c) = next_char {
            self.loc.advance(c);
            self.lexeme.push(c);
        }
        next_char
    }

    // check = peek: get next character without consuming
    fn check(&mut self) -> Option<&char> {
        self.characters.peek()
    }

    fn check_if(&mut self, func: impl FnOnce(&char) -> bool) -> bool {
        if let Some(ch_ref) = self.characters.peek() {
            func(ch_ref)
        } else {
            false
        }
    }

    fn check_if_eq(&mut self, ch: &char) -> bool {
        self.check_if(|c| c == ch)
    }

    // 
    fn clear_buffers(&mut self) {
        self.lexeme.clear();
        self.loc.clear();
    }

    // Terminal stream methods
    // Emit new terminal
    pub fn pop(&mut self) -> Option<Result<Token, LexError>> {
        if self.eof {
            return None;
        }
        let terminal = self.lexify()
            .map(
                |t| Token::new(t, self.loc.clone())
            );
        
        Some(terminal)
    }

    // match characters against a terminals
    // this method is used by peek and pop
    fn lexify(&mut self) -> Result<Terminal, LexError> {
        self.skip_whitespace();
        self.skip_comments()?;

        self.clear_buffers();
        let next_char = self.advance();
        if next_char.is_none() {
            self.eof = true;
            return Ok(Terminal::EOF);
        }

        let ch = next_char.unwrap();
        

        let token = match ch {
            '\n' => Terminal::Newline,
            '(' => Terminal::LeftParen,
            ')' => Terminal::RightParen,
            '{' => Terminal::LeftBrace,
            '}' => Terminal::RightBrace,
            '[' => Terminal::LeftBracket,
            ']' => Terminal::RightBracket,
            '+' => Terminal::Plus,
            '*' => Terminal::Star,
            ';' => Terminal::SemiColon,
            ':' => Terminal::Colon,
            '=' =>   Terminal::Equal,
            '\'' =>Terminal::Tick,
            ',' => Terminal::Comma,
            '>' => Terminal::Greater,
            '/' =>  Terminal::Slash,
            '<' => self.less_or_arrow(),
            '-' =>  self.minus_or_arrow(),
            '\"' => self.quoted_identifier()?,
            _ => self.identifier(),
        };
        return Ok(token)
    }

    // methods used inside lexify
    fn skip_whitespace(&mut self) {
        loop {
            let ch = self.advance_if(|c| c.is_whitespace() && *c != '\n');
            match ch {
                None => break,
                _ => continue,
            }
        }
    }

    fn skip_comments(&mut self) -> Result<(), LexError> {
        if self.advance_if_eq(&'/').is_some() {
            let ch = self.advance_if(|c| *c == '/' || *c == '*');
            match ch {
                Some('/') => self.skip_line_comment(),
                Some('*') => self.skip_block_comment()?,
                _ => (),
            }
        }
        return Ok(())
    }

    fn skip_line_comment(&mut self) {
        loop {
            let ch = self.advance_if(|c| *c != '\n');
            match ch {
                None => break,
                _ => continue,
            }
        }
    }

    fn skip_block_comment(&mut self) -> Result<(), LexError> {
        // Look for */ sequence
        // record location of /* start in case of unterminated block
        let (line, col) = (self.loc.line, self.loc.column); 
        loop { 
            match self.advance() {
                None => {
                    let e = LexError::new(
                        "unterminated block comment /*", self.loc.name.clone(), line, col);
                    return Err(e);
                },
                Some('*') if self.advance_if_eq(&'/').is_some() => {
                    return Ok(());
                }
                Some(_) => continue,
            }
        }
        
    }

    fn minus_or_arrow(&mut self) -> Terminal {
        match self.advance_if_eq(&'>') {
            None => Terminal::Minus,
            _ => Terminal::RightArrow,
        }
    }

    fn less_or_arrow(&mut self) -> Terminal {
        match self.advance_if_eq(&'-') {
            None => return Terminal::Less,
            _ => match self.advance_if_eq(&'>') {
                None => Terminal::LeftArrow,
                _ => Terminal::LeftRightArrow,
            },
        }
    }

    fn quoted_identifier(&mut self) -> Result<Terminal, LexError> {
        // Look for closing "        
        // record location of /* start in case of unterminated block
        let (line, col) = (self.loc.line, self.loc.column); 
        self.clear_buffers();
        loop {
            match self.check() {
                None => {
                    let e = LexError::new("unterminated literal\"",  self.loc.name.clone(), line, col);
                    return Err(e);
                },
                Some('"') => {
                    let token = Terminal::Identifier(self.lexeme.clone());
                    return Ok(token);  
                },
                Some(ch) => {
                    self.advance();
                    continue;
                }
            }
        }
    }

    fn identifier(&mut self) -> Terminal {
        while let Some(ch) = self.advance_if(|c| c.is_alphanumeric()) {}
        let s = self.lexeme.as_str();
        // check keywords
        match s {
            "where" => return Terminal::Where,
            _ => (),
        };
        // test number
        let maybe_number = s.parse::<i64>();
        match maybe_number {
            Ok(n) => Terminal::Number(n),
            _ => Terminal::Identifier(self.lexeme.clone()),
        }
    }
}

impl<I> Iterator for Lexer<I>
where
    I : Iterator<Item = char>
{
    type Item = Result<Token, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

// location information
#[derive(Clone, Debug)]
pub struct Span {
    pub name: String,
    pub start: usize, // byte offset
    pub end: usize,   // byte offset
    pub line: usize,  // 1-based
    pub column: usize, // 1-based
}

impl Span {
    
    pub fn new(name: String) -> Self {
        Self {
            name,
            start: 0,
            end: 0,
            line: 1,
            column: 1,
        }
    }
    
    pub fn advance(&mut self, ch : char){
        self.end += ch.len_utf8();
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;       
        }
    }

    pub fn clear(&mut self) {
        self.start = self.end;
    }

}


impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}


/*
#[derive(Debug)]
struct Lexeme {
    start: usize,
    end: usize,
    string: String,
}

impl Lexeme {
    fn new() -> Self {
        Self { start: 0, end: 0, string : String::new() }
    }

    fn clear(&mut self) {
        self.start = self.end;
        self.string.clear();
    }

    fn push(&mut self, ch: char) {
        self.end += ch.len_utf8();
        self.string.push(ch);
    }

    fn show(&self) -> &str {
        &self.string
    } 

}

*/
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_span() {
        let source = "+ -> A";
        let mut lexer = Lexer::new(source.chars());
        lexer.lexify();
        assert_eq!(lexer.loc.start, 0);
        assert_eq!(lexer.loc.end, 1);
        lexer.lexify();
        assert_eq!(lexer.loc.start, 2);
        assert_eq!(lexer.loc.end, 4);
        lexer.lexify();
        assert_eq!(lexer.loc.start, 5);
        assert_eq!(lexer.loc.end, 6);
    }

    #[test]
    fn test_multi_char_tokens() {
        let source = "-> <- <->";
        let mut lexer = Lexer::new(source.chars());
        assert_eq!(lexer.lexify().expect("check test"), Terminal::RightArrow);
        assert_eq!(lexer.lexify().expect("check test"), Terminal::LeftArrow);
        assert_eq!(lexer.lexify().expect("check test"), Terminal::LeftRightArrow);
    }

    #[test]
    fn test_comments() {
        let source = "//this is a comment
->//this line ends with a comment
*/* 
This is a block comment

*/+";
        let mut lexer = Lexer::new(source.chars());

        assert_eq!(lexer.lexify().unwrap(), Terminal::Newline);
        assert_eq!(lexer.lexify().unwrap(), Terminal::RightArrow);
        assert_eq!(lexer.lexify().unwrap(), Terminal::Newline);
        assert_eq!(lexer.lexify().unwrap(), Terminal::Star);
        assert_eq!(lexer.lexify().unwrap(), Terminal::Plus);
    }

    #[test]
    fn test_quoted_identifier() {
        let source = "\"2-oxoglutarate\"";
        let mut lexer = Lexer::new(source.chars());
        assert_eq!(
            lexer.lexify(),
            Terminal::Identifier("2-oxoglutarate".to_string())
        );
    }

    #[test]
    fn test_identifier() {
        let source = "2-oxoglutarate->A";
        let mut lexer = Lexer::new(source.chars());
        assert_eq!(lexer.lexify(), Terminal::Number(2));
        assert_eq!(lexer.lexify(), Terminal::Minus);
        assert_eq!(
            lexer.lexify(),
            Terminal::Identifier("oxoglutarate".to_string())
        );
        assert_eq!(lexer.lexify(), Terminal::RightArrow);
        assert_eq!(lexer.lexify(), Terminal::Identifier("A".to_string()));
    }

    #[test]
    fn test_keyword_where() {
        let source = "wher where whereas";
        let mut lexer = Lexer::new(source.chars());
        assert_eq!(lexer.lexify(), Terminal::Identifier("wher".to_string()));
        assert_eq!(lexer.lexify(), Terminal::Where);
        assert_eq!(lexer.lexify(), Terminal::Identifier("whereas".to_string()));
    }

    #[test]
    fn test_lexer_iterator() {
        let source = "A + B -> C + D";
        let mut lexer = Lexer::new(source.chars());
        let tokens: Vec<Terminal> = lexer.map(|t| t.to_terminal()).collect();
        let expected_tokens = [
            Terminal::Identifier("A".to_string()),
            Terminal::Plus,
            Terminal::Identifier("B".to_string()),
            Terminal::RightArrow,
            Terminal::Identifier("C".to_string()),
            Terminal::Plus,
            Terminal::Identifier("D".to_string()),
            Terminal::EOF,
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_newline_separators() {
        let source = "A->B\nC->D";
        let mut lexer = Lexer::new(source.chars());
        let tokens: Vec<Terminal> = lexer.map(|t| t.to_terminal()).collect();
        let expected_tokens = [
            Terminal::Identifier("A".to_string()),
            Terminal::RightArrow,
            Terminal::Identifier("B".to_string()),
            Terminal::Newline,
            Terminal::Identifier("C".to_string()),
            Terminal::RightArrow,
            Terminal::Identifier("D".to_string()),
            Terminal::EOF,
        ];
        assert_eq!(tokens, expected_tokens);
    }
}
