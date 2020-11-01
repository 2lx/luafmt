pub mod comment_ast;
pub mod common;
pub mod lua_ast;

mod lexer_util;

mod comment_lexer;
#[allow(clippy::all)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod comment_syntax;
mod comment_test;

mod lua_lexer;

#[allow(clippy::all)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod lua_syntax;
mod lua_test;

use lalrpop_util::ParseError;

type LuaParserError = ParseError<usize, lua_lexer::Token, lua_lexer::LexicalError>;
type CommentParserError = ParseError<usize, comment_lexer::Token, comment_lexer::LexicalError>;

pub fn parse_lua(src: &str) -> Result<lua_ast::Node, LuaParserError> {
    let lexer = lua_lexer::Lexer::new(src);
    lua_syntax::ChunkParser::new().parse(src, lexer)
}

pub fn parse_comment(src: &str) -> Result<comment_ast::Node, CommentParserError> {
    let lexer = comment_lexer::Lexer::new(src);
    comment_syntax::ChunkParser::new().parse(src, lexer)
}
