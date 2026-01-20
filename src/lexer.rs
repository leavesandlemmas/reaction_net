use super::grammar::Terminal;
use std::iter::Peekable;
use std::str::Chars;



pub struct Lexer<I>   
where 
    I : Iterator<Item = char> 
{
    characters: Peekable<I>,
    lexeme: Lexeme,   
    eof: bool,
}

impl<I> Lexer<I>
where
    I : Iterator<Item = char>
{

    pub fn new(characters: I) -> Self {
        let lexeme = Lexeme::new();
        Self {
            characters : characters.peekable(),
            lexeme,
            eof: false,
        }
    }
    
    // character stream methods
    // advance = pop: consume one character
    fn advance(&mut self) -> Option<char> {
        let next_char = self.characters.next();
        if let Some(ch) = next_char {
            self.lexeme.push(ch);
        }
        next_char
    }

    fn advance_if(&mut self, func: impl FnOnce(&char) -> bool) -> Option<char> {
        let next_char = self.characters.next_if(func);
        if let Some(ch) = next_char {
            self.lexeme.push(ch);
        }
        next_char
    }

    fn advance_if_eq(&mut self, ch: &char) -> Option<char> {
        let next_char = self.characters.next_if_eq(ch);
        if let Some(c) = next_char {
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

    // Terminal stream methods
    // Emit new terminal
    pub fn pop(&mut self) -> Option<Terminal> {
        if self.eof {
            return None;
        }
        let token = self.lexify();
        Some(token)
    }

    // match characters against a terminals
    // this method is used by peek and pop
    fn lexify(&mut self) -> Terminal {
        self.skip_whitespace();
        self.skip_comments();

        self.lexeme.clear();
        let next_char = self.advance();
        if next_char.is_none() {
            self.eof = true;
            return Terminal::EOF;
        }

        let ch = next_char.unwrap();

        match ch {
            '\n' => return Terminal::Newline,
            '(' => return Terminal::LeftParen,
            ')' => return Terminal::RightParen,
            '{' => return Terminal::LeftBrace,
            '}' => return Terminal::RightBrace,
            '[' => return Terminal::LeftBracket,
            ']' => return Terminal::RightBracket,
            '+' => return Terminal::Plus,
            '*' => return Terminal::Star,
            ';' => return Terminal::SemiColon,
            ':' => return Terminal::Colon,
            '=' => return Terminal::Equal,
            '\'' => return Terminal::Tick,
            ',' => return Terminal::Comma,
            '>' => return Terminal::Greater,
            '/' => return Terminal::Slash,
            '<' => return self.less_or_arrow(),
            '-' => return self.minus_or_arrow(),
            '\"' => return self.quoted_identifier(),
            _ => return self.identifier(),
        }
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

    fn skip_comments(&mut self) {
        if self.advance_if_eq(&'/').is_some() {
            let ch = self.advance_if(|c| *c == '/' || *c == '*');
            match ch {
                Some('/') => self.skip_line_comment(),
                Some('*') => self.skip_block_comment(),
                _ => return,
            }
        }
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

    fn skip_block_comment(&mut self) {
        // Look for */ sequence
        loop {
            match self.advance() {
                None => panic!("Unterminated block comment"),
                Some('*') if self.advance_if_eq(&'/').is_some() => {
                    return;
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

    fn quoted_identifier(&mut self) -> Terminal {
        // Look for closing "
        self.lexeme.clear();
        loop {
            match self.check() {
                None => panic!("Unterminated literal identifier."),
                Some('"') => {
                    let s = self.lexeme.show();
                    return Terminal::Identifier(s.to_string());
                }
                Some(ch) => {
                    self.advance();
                    continue;
                }
            }
        }
    }

    fn identifier(&mut self) -> Terminal {
        while let Some(ch) = self.advance_if(|c| c.is_alphanumeric()) {}
        let s = self.lexeme.show();
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

impl<I> Iterator for Lexer<I>
where
    I : Iterator<Item = char>
{
    type Item = Terminal;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lexeme_count() {
        let source = "+ -> A";
        let mut lexer = Lexer::new(source.chars());
        lexer.lexify();
        assert_eq!(lexer.lexeme.start, 0);
        assert_eq!(lexer.lexeme.end, 1);
        lexer.lexify();
        assert_eq!(lexer.lexeme.start, 2);
        assert_eq!(lexer.lexeme.end, 4);
        lexer.lexify();
        assert_eq!(lexer.lexeme.start, 5);
        assert_eq!(lexer.lexeme.end, 6);
    }

    #[test]
    fn test_multi_char_tokens() {
        let source = "-> <- <->";
        let mut lexer = Lexer::new(source.chars());
        assert_eq!(lexer.lexify(), Terminal::RightArrow);
        assert_eq!(lexer.lexify(), Terminal::LeftArrow);
        assert_eq!(lexer.lexify(), Terminal::LeftRightArrow);
    }

    #[test]
    fn test_comments() {
        let source = "//this is a comment
->//this line ends with a comment
*/* 
This is a block comment

*/+";
        let mut lexer = Lexer::new(source.chars());

        assert_eq!(lexer.lexify(), Terminal::Newline);
        assert_eq!(lexer.lexify(), Terminal::RightArrow);
        assert_eq!(lexer.lexify(), Terminal::Newline);
        assert_eq!(lexer.lexify(), Terminal::Star);
        assert_eq!(lexer.lexify(), Terminal::Plus);
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
        let tokens: Vec<Terminal> = lexer.collect();
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
        let tokens: Vec<Terminal> = lexer.collect();
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
