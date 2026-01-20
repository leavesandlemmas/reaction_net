// import grammar symbols
use super::grammar;
use super::grammar::Terminal;
//use super::error::SyntaxError;
use super::lexer::Lexer;

//use crate::registry::Registry;
//use crate::matrix::CscMatrix;


// Parser struct contains syntax analysis logic
pub struct Parser
{
    lexer : Lexer,
}

impl Parser
{

    pub fn new() -> Self {
        Self { }
    }

    // actions for token stream
    // advance to next character
    fn advance(&mut self) -> Option<Terminal> {
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
    fn peek(&mut self) -> Option<&Terminal> {
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

    // build AST for CRN from recursiving descent parsing
    pub fn parse(&mut self, I) -> Result<(), ParseError> {
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
          
            Ok(())
        } else {
            panic!("Expected yield symbol '->', '<-', '<->' or '='")
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
        } else {
            panic!("Factor Error.")
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    
}
