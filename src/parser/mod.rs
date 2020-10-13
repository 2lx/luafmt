pub mod basics;
pub mod comment_ast;
pub mod lua_ast;

mod comment_lexer;
mod comment_syntax;
mod comment_test;
mod lexer_util;
mod lua_lexer;
mod lua_syntax;
mod lua_test;

use lalrpop_util::ParseError;

pub fn parse_lua(src: &str) -> Result<lua_ast::Node, ParseError<usize, lua_lexer::Token, lua_lexer::LexicalError>> {
    let lexer = lua_lexer::Lexer::new(src);
    lua_syntax::ChunkParser::new().parse(src, lexer)
}

pub fn parse_comment(
    src: &str,
) -> Result<comment_ast::Node, ParseError<usize, comment_lexer::Token, comment_lexer::LexicalError>> {
    let lexer = comment_lexer::Lexer::new(src);
    comment_syntax::ChunkParser::new().parse(src, lexer)
}
