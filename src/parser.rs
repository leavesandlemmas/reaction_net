// import grammar symbols
use super::grammar::Terminal;
//use super::error::SyntaxError;

//use crate::registry::Registry;
//use crate::matrix::CscMatrix;


// Parser struct contains syntax analysis logic
pub struct Parser<I> 
    where
I : Iterator<Item = Terminal>
{
    tokens : Peekable<I>,
}

impl<I> Parser<I> 
    where
I : Iterator<Item = Terminal>
{

    pub fn new(tokens : I) -> Self {
        Self { tokens : tokens.peekable() }
    }

    // actions for token stream
    // advance to next character
    fn advance(&mut self) -> Option<Terminal> {
        self.tokens.next()
    }

    fn advance_if_eq(&mut self, t : Terminal) -> Option<Terminal> {
        self.tokens.next_if_eq(&t)
    } 

    // look at next character without consuming
    fn peek(&mut self) -> Option<&Terminal> {
        self.tokens.peek()
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
        if self.advance_if_eq(Terminal::SemiColon) {
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
