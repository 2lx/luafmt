#[allow(clippy::all)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod lua_syntax;
mod lua_lexer;
mod lua_test;

pub mod basics;
pub mod lua_ast;

use lalrpop_util::ParseError;

pub fn parse_lua(src: &str) -> Result<lua_ast::Node, ParseError<usize, lua_lexer::Token, lua_lexer::LexicalError>> {
    let lexer = lua_lexer::Lexer::new(src);
    lua_syntax::ChunkParser::new().parse(src, lexer)
}
