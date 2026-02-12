mod tokens;
mod lexer;
mod error;

//use crate::registry::Registry;
//use crate::matrix::CscMatrix;
use crate::parser::tokens::Terminal;
use crate::parser::lexer::Lexer;
use crate::parser::error::{ParseError, ParseErrorKind};


// Parser struct contains syntax analysis logic
pub struct Parser<'lex> {
    lexer: Lexer<'lex>,
    lookahead: Option<Terminal>,
    reaction_names: Vec<Option<String>>,
}

impl<'lex> Parser<'lex> 
{

    pub fn new(source: &'lex str, source_name: &'lex str) -> Self {
        Self {lexer: Lexer::new(source, source_name), lookahead : None, reactions : Vec::new()}
    }

    // actions for token stream; must handle error?
    // advance to next character
    fn advance(&mut self) -> Result<Option<Terminal>, ParseError> {
        match self.lookahead {
            None => self.lexer.pop().transpose(),
            Some(_) => Ok(self.lookahead.take()), 
        }
    }

    
    fn advance_if(&mut self, func : impl FnOnce(&Terminal) -> bool) -> Result<bool, ParseError> {
        if self.peek_if(func)? {
            self.advance()?;
            Ok(true)
        } else {
            Ok(false)
        }
    } 

    fn advance_if_eq(&mut self, token : Terminal) -> Result<bool, ParseError> {
        self.advance_if(|t| *t == token)
    } 

    //look at next character without consuming
    fn peek(&mut self) -> Result<Option<&Terminal>, ParseError> {
        if self.lookahead.is_none() {
            self.lookahead = self.advance()?;            
        }
        Ok(self.lookahead.as_ref())
    }

        //look at next character without consuming
    fn peek_if(&mut self, func : impl FnOnce(&Terminal) -> bool) -> Result<bool, ParseError> {
        let value = self.peek()?.map(func); 
        let out = match value {
            Some(true) => true,
            _ => false,
        };
        Ok(out)
    }

    //look at next character without consuming
    fn peek_if_eq(&mut self, token : Terminal) -> Result<bool, ParseError> {
        self.peek_if(|t| *t==token)
    }

    fn emit_error(&self, kind : ParseErrorKind) -> ParseError {
        let file = self.lexer.name().to_string();
        let (line, col) = self.lexer.location();
        ParseError::new(kind, file, line, col)
    }

    // build AST for CRN from recursiving descent parsing
    pub fn parse(&mut self) -> Result<(), ParseError> {
        self.reaction_list()?;
        Ok(())
    }

    // grammar productions for recursive descent
    fn reaction_list(&mut self) -> Result<(), ParseError> {
        self.reaction()?;
        while !self.peek_if_eq(Terminal::EOF)? {
            self.line_separator()?;

            match self.peek()? {
                None | Some(Terminal::EOF) => break,
                other => self.reaction()?,
            }
        }
        
        Ok(())
    }

    fn line_separator(&mut self) -> Result<(), ParseError> {
        if self.advance_if(|x| x.is_line_separator())? {
            Ok(())
        } else {
            let e = self.emit_error(ParseErrorKind::MissingLineSep);
            Err(e)
        }
    }

    fn reaction(&mut self) -> Result<(), ParseError> {
        // check if empty reaction
        match self.peek()? {
            Some(Terminal::Newline) 
            | Some(Terminal::SemiColon) 
            | Some(Terminal::EOF) => return Ok(()),
            _ => (),
        }

        // check for reaction name
        let name = if self.peek_if(|x| x.is_identifier()) {
            
        } else {
            
        }

        self.reaction_names.push(name);

        self.complex()?;
        self.yield_symbol()?;
        self.complex()?;

        if self.advance_if_eq(Terminal::Colon)? {
            self.kinetics()?
        } 

        loop {
            if self.peek_if(|x| !x.is_yield_symbol())? {
                break;
            }
            self.yield_symbol()?;
            self.complex()?;
            
        }      

        Ok(())
    }

    fn yield_symbol(&mut self) -> Result<(), ParseError> {
        todo!();
    }

    fn kinetics(&mut self) -> Result<(), ParseError> {
        todo!();
    }

    fn complex(&mut self) -> Result<(), ParseError> {
        todo!();
    }
/*
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

    fn stoich_coef(&mut self) -> Result<(), ParserError> {
        
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
*/
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance() -> Result<(), ParseError>{
        let source = "( + - )";
        let mut parser = Parser::new(source, "test");
        
        assert_eq!(parser.advance()?, Some(Terminal::LeftParen));
        assert_eq!(parser.peek()? , Some(&Terminal::Plus));  

        Ok(())
    
    }

    #[test]
    fn test_empty() -> Result<(), ParseError>{
        let source = "";
        let mut parser = Parser::new(source, "test");
        parser.parse()?;
        Ok(())
    }  

    #[test]
    fn test_newline_separators() -> Result<(), ParseError>{
        let source = "\n\n;;\n\n";
        let mut parser = Parser::new(source, "test");
        parser.parse()?;
        Ok(())
    }   
}