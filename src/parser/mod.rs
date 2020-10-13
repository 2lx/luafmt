#[allow(clippy::all)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod syntax;
mod lexer;
mod test;
pub mod nodes;

use lalrpop_util::ParseError;

pub fn parse(src: &str) -> Result<nodes::Node, ParseError<usize, lexer::Token, lexer::LexicalError>> {
    let lexer = lexer::Lexer::new(src);
    syntax::ChunkParser::new().parse(src, lexer)
}
