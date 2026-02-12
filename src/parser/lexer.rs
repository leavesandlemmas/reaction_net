use crate::parser::tokens::Terminal;
use std::fmt;
use std::str::Chars;
use super::error::{ParseError, ParseErrorKind};

pub struct Lexer<'lex>
{
    source: &'lex str,
    source_name: &'lex str,
    characters : Chars<'lex>,
    lookahead : Option<char>,
    loc: Span,
    eof: bool,
}

pub type MaybeTerminal = Result<Terminal, ParseError>;

impl<'lex> Lexer<'lex>
{
    
    pub fn new(source: &'lex str, source_name: &'lex str) -> Self {
        Self{
            source,
            source_name,
            characters: source.chars(),
            loc: Span::new(),
            lookahead: None,
            eof: false,
        }
    }

    // character stream methods
    // advance = pop: consume one character
    fn advance(&mut self) -> Option<char> {
        
        let next_char = match self.lookahead {
            Some(_) => self.lookahead.take(),
            None => self.characters.next(),
        };

        if let Some(ch) = next_char {
            self.loc.advance(ch);
        }
        
        next_char
    }
    
    fn advance_if(&mut self, func: impl FnOnce(&char) -> bool) -> Option<char> {
        let next_char = self.check().map(func);
        match next_char {
            Some(true) => self.advance(),
            other => None, 
        }
    }
    
    fn advance_if_eq(&mut self, ch: &char) -> Option<char> {
        self.advance_if(|c| c == ch)
    }
    
    // check = peek: get next character without consuming
    fn check(&mut self) -> Option<&char> {
        if self.lookahead.is_none() {
            self.lookahead.replace(self.characters.next()?);
        }

        self.lookahead.as_ref()
    }
    
    fn clear_buffers(&mut self) {
        self.loc.clear();
    }

    fn lexeme(&self) -> &str {
        &self.source[self.loc.start..self.loc.end]
    }

    pub fn location(&self) -> (usize, usize) {
        (self.loc.line, self.loc.column)
    }

    pub fn name(&self) -> &str {
        self.source_name
    }

    // Terminal stream methods
    // Emit new terminal
    pub fn pop(&mut self) -> Option<Result<Terminal, ParseError>> {
        if self.eof {
            return None;
        }
        
        let terminal = self.lexify();
        Some(terminal)
    }

    // match characters against a terminals
    // this method is used by peek and pop
    fn lexify(&mut self) ->  Result<Terminal, ParseError> {
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
            '=' => Terminal::Equal,
            '\''=> Terminal::Tick,
            ',' => Terminal::Comma,
            '>' => Terminal::Greater,
            '/' => Terminal::Slash,
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

    fn skip_comments(&mut self) -> Result<(), ParseError> {
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

    fn skip_block_comment(&mut self) -> Result<(), ParseError> {
        // Look for */ sequence
        // record location of /* start in case of unterminated block
        let (line, col) = (self.loc.line, self.loc.column); 
        loop { 
            match self.advance() {
                None => {
                    let e = ParseError::new(
                        ParseErrorKind::UnterminatedBlockComment, 
                        self.source_name.to_string(), line, col);
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

    fn quoted_identifier(&mut self) -> Result<Terminal, ParseError> {
        // Look for closing "        
        // record location of /* start in case of unterminated block
        let (line, col) = (self.loc.line, self.loc.column); 
        self.clear_buffers();
        loop {
            match self.check() {
                None => {
                    let e = ParseError::new(
                        ParseErrorKind::UnterminatedQuote, 
                        self.source_name.to_string(), line, col);
                    return Err(e);
                },
                Some('"') => {
                    let s = self.lexeme();
                    let token = Terminal::Identifier(s.to_string());
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
        let s = self.lexeme();
        // check keywords
        match s {
            "where" => return Terminal::Where,
            _ => (),
        };
        // test number
        let maybe_number = s.parse::<i64>();
        match maybe_number {
            Ok(n) => Terminal::Number(n),
            _ => Terminal::Identifier(s.to_string()),
        }
    }
}

impl<'lex> Iterator for Lexer<'lex>
{
    type Item = Result<Terminal, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
       self.pop()
    }
}



// location information
#[derive(Clone, Debug)]
pub struct Span {
    pub start: usize, // byte offset
    pub end: usize,   // byte offset
    pub line: usize,  // 1-based
    pub column: usize, // 1-based
}

impl Span {
    
    pub fn new() -> Self {
        Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    type TestResult = Result<(), ParseError>;

    #[test]
    fn test_span() -> TestResult {
        let source = "+ -> A";
        let mut lexer = Lexer::new(source, "test");
        lexer.lexify()?;
        assert_eq!(lexer.loc.start, 0);
        assert_eq!(lexer.loc.end, 1);
        lexer.lexify()?;
        assert_eq!(lexer.loc.start, 2);
        assert_eq!(lexer.loc.end, 4);
        lexer.lexify()?;
        assert_eq!(lexer.loc.start, 5);
        assert_eq!(lexer.loc.end, 6);
        Ok(())
    }

    #[test]
    fn test_multi_char_tokens() -> TestResult {
        let source = "-> <- <->";
        let mut lexer = Lexer::new(source, "test");
        assert_eq!(lexer.lexify()?, Terminal::RightArrow);
        assert_eq!(lexer.lexify()?, Terminal::LeftArrow);
        assert_eq!(lexer.lexify()?, Terminal::LeftRightArrow);
        Ok(())
    }

    #[test]
    fn test_comments() -> TestResult {
        let source = "//this is a comment
->//this line ends with a comment
*/* 
This is a block comment

*/+";
        let mut lexer = Lexer::new(source, "test");

        assert_eq!(lexer.lexify()?, Terminal::Newline);
        assert_eq!(lexer.lexify()?, Terminal::RightArrow);
        assert_eq!(lexer.lexify()?, Terminal::Newline);
        assert_eq!(lexer.lexify()?, Terminal::Star);
        assert_eq!(lexer.lexify()?, Terminal::Plus);
        Ok(())
    }

    #[test]
    fn test_quoted_identifier() -> TestResult {
        let source = "\"2-oxoglutarate\"";
        let mut lexer = Lexer::new(source, "test");
        assert_eq!(
            lexer.lexify()?,
            Terminal::Identifier("2-oxoglutarate".to_string())
        );
        Ok(())
    }

    #[test]
    fn test_identifier() -> TestResult {
        let source = "2-oxoglutarate->A";
        let mut lexer = Lexer::new(source, "test");
        assert_eq!(lexer.lexify()?, Terminal::Number(2));
        assert_eq!(lexer.lexify()?, Terminal::Minus);
        assert_eq!(
            lexer.lexify()?,
            Terminal::Identifier("oxoglutarate".to_string())
        );
        assert_eq!(lexer.lexify()?, Terminal::RightArrow);
        assert_eq!(lexer.lexify()?, Terminal::Identifier("A".to_string()));
        Ok(())
    }

    #[test]
    fn test_keyword_where() -> TestResult {
        let source = "wher where whereas";
        let mut lexer = Lexer::new(source, "test");
        assert_eq!(lexer.lexify()?, Terminal::Identifier("wher".to_string()));
        assert_eq!(lexer.lexify()?, Terminal::Where);
        assert_eq!(lexer.lexify()?, Terminal::Identifier("whereas".to_string()));
        Ok(())
    }

    #[test]
    fn test_lexer_iterator() -> TestResult {
        let source = "A + B -> C + D";
        let lexer = Lexer::new(source, "test");
        let tokens: Result<Vec<Terminal>, ParseError> = lexer.collect();
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
        assert_eq!(tokens?, expected_tokens);
        Ok(())
    }

    #[test]
    fn test_newline_separators() -> TestResult {
        let source = "A->B\nC->D";
        let lexer = Lexer::new(source, "test");
        let tokens: Result<Vec<Terminal>, ParseError> = lexer.collect();

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
        assert_eq!(tokens?, expected_tokens);
        Ok(())
    }
}
