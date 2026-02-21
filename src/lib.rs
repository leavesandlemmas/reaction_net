pub mod cli;

// module declarations
mod source;
mod parse;
mod sparse;
mod registry;

mod network;

use source::SourceFile;
use parse::Lexer;
use parse::Parser;
use parse::ParseError;
use network::NetworkBuilder;

pub fn parse_crn(net: &mut NetworkBuilder, f: SourceFile) -> Result<(), ParseError> {
    let src= f.content(); 
    let name = f.name();
    let tokens = Lexer::new(src, name.as_ref());
    let mut parser = Parser::new(tokens);
    parser.parse(net)
}
