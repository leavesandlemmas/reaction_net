mod symbols;

pub struct Scanner {
    cursor: usize,
    characters: Vec<char>,
}

impl Scanner {
    pub fn new(string: &str) -> Self {
        Self {
            cursor: 0,
            characters: string.chars().collect(),
        }
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    // returns the next character without advancing
    pub fn peek(&self) -> Option<&char> {
        self.characters.get(self.cursor)
    }

    pub fn is_done(&self) -> bool {
        self.cursor == self.characters.len()
    }
    // advance to next character
    pub fn pop(&mut self) -> Option<&char> {
        match self.characters.get(self.cursor) {
            Some(character) => {
                self.cursor += 1;
                Some(character)
            }
            None => None,
        }
    }

    pub fn take(&mut self, target: &char) -> bool {
        match self.characters.get(self.cursor) {
            Some(character) => {
                if target == character {
                    self.cursor += 1;
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    pub fn transform<T>(&mut self, cb: impl FnOnce(&char) -> Option<T>) -> Option<T> {
        match self.characters.get(self.cursor) {
            Some(input) => match cb(input) {
                Some(output) => {
                    self.cursor += 1;
                    Some(output)
                }
                None => None,
            },
            None => None,
        }
    }

    pub fn scan<T>(&mut self, cb: impl Fn(&str) -> Option<Action<T>>) -> Result<Option<T>, Error> {
        let mut sequence = String::new();
        let mut require = false;
        let mut request = None;

        loop {
            match self.characters.get(self.cursor) {
                Some(target) => {
                    sequence.push(*target);

                    match cb(&seqeuence) {
                        Some(Action::Return(result)) => {
                            self.cursor += 1;
                            break Ok(Some(result));
                        }
                        Some(Action::Request(result)) => {
                            self.cursor += 1;
                            require = false;
                            request = Some(result);
                        }
                        Some(Action::Require) => {
                            self.cursor += 1;
                            require = true;
                        }
                        None => {
                            if require {
                                break Err(Error::Character(self.cursor));
                            } else {
                                break Ok(request);
                            }
                        }
                    }
                }
                None => {
                    if require {
                        break Err(Error::EndOfLine);
                    } else {
                        Ok(request)
                    }
                }
            }
        }
    }
}


pub enum Action<T> {
    // If iteration is returns none, return T without advancing the cursor
    Request(T),
    //  If the next iteration returns None, return None without
    Require,
    /// Immediately advance the cursor and return T
    Return(T)
}

pub fn tokenize(string : &str) -> bool {
    let mut scanner = Scanner::new(string);
    
    loop {
    
    

    }    
    
    scanner.cursor() > 0 && scanner.is_done()
}

struct Tokens {
    tokens: Vec<Symbol>,
}

impl Tokens {
    fn parse(characters: impl Iterator<Item = char>) -> Result<Tokens, String> {
        let mut tokens: Vec<Symbol> = Vec::new();
        let mut iterator = characters.peekable();
        let mut line: u64 = 1;
        while let Some(c) = iterator.next() {
            if c.is_whitespace() {
                if c == '\n' {
                    line += 1
                }
            } else {
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
                    '"' => tokens.push(Symbol::Quote),
                    '\'' => tokens.push(Symbol::Tick),
                    ',' => tokens.push(Symbol::Comma),     
                    '/' => {
                        if let Some('/') = iterator.peek() {
                            iterator.next();
                            while let Some(c) = iterator.next() {
                                if c == '\n' {
                                    line += 1;
                                    break ;
                                }
                            }
                        } else if let Some('*') = iterator.peek(){
                            iterator.next();
                            while let Some(c) = iterator.next() {
                                if c == '*' {
                                    if  let Some('/') = iterator.peek() {break;}

                                }
                            }
                        } else {tokens.push(Symbol::Slash);}
                    }               
                    '-' => {
                        if let Some('>') = iterator.peek() {
                            iterator.next();
                            tokens.push(Symbol::RightArrow);
                        } else {
                            tokens.push(Symbol::Minus);
                        }
                    }
                    '>' => tokens.push(Symbol::Greater),
                    '<' => {
                        if let Some('-') = iterator.peek() {
                            iterator.next();
                            if let Some('>') = iterator.peek() {
                                tokens.push(Symbol::LeftRightArrow);
                            } else {
                                tokens.push(Symbol::LeftArrow);
                            }
                        } else {
                            tokens.push(Symbol::Less);
                        }
                    }
                    _ => {
                        if c.is_alphanumeric() {
                            let mut lexeme = String::new();
                            lexeme.push(c);
                            let mut number = c.is_ascii_digit();
                            while let Some(c) = iterator.next() {
                                if c.is_alphanumeric() {
                                    number &= c.is_ascii_digit();
                                    lexeme.push(c);
                                } else {
                                    break;
                                }
                            }
                            if number {let n :u32= lexeme.parse().expect("Couldn't parse integer."); tokens.push(Symbol::Number(n));} else {
tokens.push(Symbol::Species(lexeme));
                            }
                            
                        } else {
                            return Err(format!("Line {line}: Character not recognized {c}."));
                        }
                    }
                };
            }
        }
        Ok(Tokens { tokens })
    }

    fn print(&self) {
        for token in &self.tokens {
            println!("{token:?}");
        }
    }
}

