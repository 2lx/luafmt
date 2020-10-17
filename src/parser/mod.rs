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

type LuaParserError<'a> = ParseError<usize, lua_lexer::Token<'a>, lua_lexer::LexicalError>;
type CommentParserError<'a> = ParseError<usize, comment_lexer::Token<'a>, comment_lexer::LexicalError>;

pub fn parse_lua(src: &str) -> Result<lua_ast::Node, LuaParserError> {
    let lexer = lua_lexer::Lexer::new(src);
    lua_syntax::ChunkParser::new().parse(src, lexer)
}

pub fn parse_comment(src: &str) -> Result<comment_ast::Node, CommentParserError> {
    let lexer = comment_lexer::Lexer::new(src);
    comment_syntax::ChunkParser::new().parse(src, lexer)
}
