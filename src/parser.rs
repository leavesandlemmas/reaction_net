// import grammar symbols
use super::grammar;
use super::grammar::Terminal;
use super::error::{ParseError,  SyntaxError};
use super::scanner::Scanner;

use crate::registry::Registry;
use crate::matrix::CscMatrix;
// Parser struct contains syntax analysis logic
pub struct Parser<I>
where
    I : Iterator<Item = Terminal>,
{
    tokens: I,
    lookahead: Option<Terminal>,
    species: Registry<String>,
    complex: CscMatrix<i64>,
}

type Maybe<T> = Result<Option<T>, ParseError>;

impl<I> Parser<I>
where
    I : Iterator<Item = Terminal>,
{
    pub fn new(tokens :I) -> Self {
        Self {
            tokens,
            lookahead: None,
            species: Registry::new(),
            complex: CscMatrix::new(),
        }
    }

    // actions for token stream
    // advance to next character
    fn pop_token(&mut self) -> Maybe<Terminal> {
        // check lookahead buffer first
        if let Some(token) = self.lookahead.take() {
            return Ok(Some(token));
        }

        // pop next token and handle lex error
        match self.tokens.next() {
            Some(Ok(token)) => Ok(Some(token)),
            Some(Err(e)) => Err(ParseError::Lex(e)),
            None => Ok(None),
        }
    }

    // look at next character without consuming
    fn peek_token(&mut self) -> Maybe<&Terminal> {
        // check buffer
        if self.lookahead.is_some() {
            return Ok(self.lookahead.as_ref());
        }
        // if buffer is none, then pop and put into buffer
        // handling errors
        match self.tokens.next() {
            Some(Ok(token)) => {
                self.lookahead = Some(token);
                Ok(self.lookahead.as_ref())
            }
            Some(Err(e)) => Err(ParseError::Lex(e)),
            None => Ok(None),
        }
    }

    // advance to next character if next token satisfies predicate
    fn next_if(&mut self, predicate: impl FnOnce(&Terminal) -> bool) -> Maybe<Terminal> {
        if self.peek_if(predicate) {
            self.pop_token()
        } else {
            Ok(None)
        }
    }

    // check if next token satisfies predicate
    fn peek_if(&mut self, predicate: impl FnOnce(&Terminal) -> bool) -> bool {
        if let Ok(Some(token)) = self.peek_token() {
            predicate(&token)
        } else {
            false
        }
    }

    // check if next token matches without consuming
    fn peek_if_match(&mut self, symbol: Terminal) -> bool {
        self.peek_if(|x: &Terminal| *x == symbol)
    }

    // check if next token matches; consume if yes
    fn next_if_match(&mut self, symbol: Terminal) -> Maybe<Terminal> {
        self.next_if(|x: &Terminal| *x == symbol)
    }

    // check if next token matches; consume if yes
    fn advance_if_match(&mut self, symbol: Terminal) -> bool {
        let matched = self.peek_if_match(symbol);
        if matched {
            let _ = self.pop_token();
            true
        } else {
            false
        }
    }

    fn emit_error<S, E>(&self, msg: S) -> Result<E, ParseError>
    where
        S: Into<String> + AsRef<str>,
    {
        let e = SyntaxError::new(msg, self.tokens.get_line_num());
        Err(ParseError::Syntax(e))
    }

    // build AST for CRN from recursiving descent parsing
    pub fn parse(&mut self) -> Result<(), ParseError> {
        self.reaction_list()?;
        Ok(())
    }

    // grammar productions for recursive descent
    fn reaction_list(&mut self) -> Result<(), ParseError> {
    
        self.reaction()?;
        self.next_reaction()?;
        Ok(())
    }

    fn reaction(&mut self) -> Result<(), ParseError> {

        self.complex()?;
        self.yield_symbol()?;
        self.complex()?;
        Ok(())
    }

    fn next_reaction(&mut self) -> Result<(), ParseError> {
        if self.advance_if_match(Terminal::SemiColon) {
            if self.peek_if(|x| *x != Terminal::SemiColon) {
                let rxn = self.reaction()?;
            }
            let next = self.peek_token()?;
            if next.is_some() {
                return self.next_reaction();
            }
            Ok(())
        } else {
            let msg = format!("Expected newline or ';' but found unexpected");
            self.emit_error(msg)
        }
    }

    fn yield_symbol(&mut self) -> Result<(), ParseError> {
        let maybe_token = self.next_if(grammar::is_yield_symbol)?;
        if let Some(s) = maybe_token {
            // let arr = match s {
            // Terminal::RightArrow => ast::Arrow::Right,
            // Terminal::LeftArrow => ast::Arrow::Left,
            // Terminal::LeftRightArrow => ast::Arrow::Reversible,
            // Terminal::Equal => ast::Arrow::Reversible,
            // _ => panic!("`yield_symbol()` returned terminal that was not an arrow."),
            // };
            Ok(())
        } else {
            self.emit_error("Expected yield symbol '->', '<-', '<->' or '='")
        }
    }

    fn complex(&mut self) -> Result<(), ParseError> {
        self.monomial()?;
        self.next_monomial()?;
        Ok(())
    }

    fn next_monomial(&mut self) -> Result<(), ParseError> {
        if self.advance_if_match(Terminal::Plus) {
            self.monomial()?;
            
            return self.next_monomial();
        }
        Ok(())
    }

    fn monomial(&mut self) -> Result<(), ParseError> {
        let coef = if self.peek_if(|x| x.is_number()) {
            let Some(Terminal::Number(coef)) = self.pop_token()? else {
                panic!("Could't unwrap Number")
            };
            self.advance_if_match(Terminal::Star);
            coef
        } else {
            1
        };
        self.species()?;
        Ok(())
    }

    fn species(&mut self) -> Result<(), ParseError> {
        if self.peek_if(|x| x.is_identifier()) {
            let Some(Terminal::Identifier(sp)) = self.pop_token()? else {
                panic!("Couldn't unwrap Identifier!")
            };
            Ok(())
        //        } else if self.advance_if_match(Terminal::LeftParen) {
        //            self.complex()?;
        //            if !self.advance_if_match(Terminal::RightParen) {
        //                return Self::emit_error("Unmatched parentheses. Expected ')' but found 's'");
        //            }
        //            Ok(())
        } else {
            self.emit_error("Factor Error.")
        }
    }
}


#[cfg(test)]
mod tests {

}