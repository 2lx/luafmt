use super::lexer_util::*;
use std::fmt;
use std::str::CharIndices;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'input> {
    OneLineComment(&'input str),
    MultiLineComment(usize, &'input str),
    NewLine,
    EOF,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Token::*;
        match self {
            OneLineComment(s) => write!(f, "--{}\n", s),
            MultiLineComment(level, s) => {
                let level_str = (0..*level).map(|_| "=").collect::<String>();
                write!(f, "--[{}[{}]{}]", level_str, s, level_str)
            }
            NewLine => write!(f, "<NewLine>"),
            EOF => write!(f, "<EOF>"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LexicalError {
    UnrecognizedSymbol(usize, char),
    UnexpectedEOF,
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexicalError::UnrecognizedSymbol(i, ch) => {
                write!(f, "lexical error: unrecognized symbol '{}' at {}", ch, i)
            }
            LexicalError::UnexpectedEOF => write!(f, "lexical error: unexpected end of file"),
        }
    }
}

pub struct Lexer<'input> {
    chars: std::iter::Peekable<CharIndices<'input>>,
    input: &'input str,
    at_end: bool,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer { chars: input.char_indices().peekable(), input, at_end: false }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Token<'input>, usize), LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;
        loop {
            match self.chars.next() {
                None => {
                    if !self.at_end {
                        self.at_end = true;
                        return Some(Ok((self.input.len(), EOF, self.input.len())));
                    }
                    return None;
                }
                Some((_, ' ')) | Some((_, '\r')) | Some((_, '\t')) => continue,

                Some((i, '\n')) => return Some(Ok((i, NewLine, i + 1))),
                Some((token_start, '-')) => match self.chars.peek() {
                    Some(&(_, '-')) => {
                        self.chars.next();

                        let (text_start, text_end, token_end, opt_level, succ) =
                            get_comment_start_ends_and_type(&mut self.chars, token_start + 2);
                        if !succ {
                            return Some(Err(LexicalError::UnexpectedEOF));
                        }

                        match opt_level {
                            Some(level) => {
                                return Some(Ok((
                                    token_start,
                                    MultiLineComment(level, &self.input[text_start..text_end]),
                                    token_end,
                                )))
                            }
                            None => {
                                return Some(Ok((
                                    token_start,
                                    OneLineComment(&self.input[text_start..text_end]),
                                    token_end,
                                )))
                            }
                        };
                    }
                    Some(&(ip, chp)) => return Some(Err(LexicalError::UnrecognizedSymbol(ip, chp))),
                    None => return Some(Err(LexicalError::UnexpectedEOF)),
                },

                Some((i, ch)) => return Some(Err(LexicalError::UnrecognizedSymbol(i, ch))),
            }
        }
    }
}

#[test]
fn test_comment_lexer() {
    type TRes<'a> = Vec<Result<(usize, Token<'a>, usize), LexicalError>>;

    let tokens = Lexer::new("").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((0, Token::EOF, 0))));

    let tokens = Lexer::new("  ").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((2, Token::EOF, 2))));

    let tokens = Lexer::new("  \n  ").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((2, Token::NewLine, 3)), Ok((5, Token::EOF, 5))));

    let tokens = Lexer::new("\n   \n").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((0, Token::NewLine, 1)), Ok((4, Token::NewLine, 5)), Ok((5, Token::EOF, 5))));

    let tokens = Lexer::new("--\n").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((0, Token::OneLineComment(""), 3)), Ok((3, Token::EOF, 3))));

    let tokens = Lexer::new("--123\n").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((0, Token::OneLineComment("123"), 6)), Ok((6, Token::EOF, 6))));

    let tokens = Lexer::new("--123").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((0, Token::OneLineComment("123"), 5)), Ok((5, Token::EOF, 5))));

    let tokens = Lexer::new("  --  123  \n  ").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((2, Token::OneLineComment("  123  "), 12)), Ok((14, Token::EOF, 14))));

    let tokens = Lexer::new("--[[]]").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((0, Token::MultiLineComment(0, ""), 6)), Ok((6, Token::EOF, 6))));

    let tokens = Lexer::new("--[=[123]=]").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((0, Token::MultiLineComment(1, "123"), 11)), Ok((11, Token::EOF, 11))));

    let tokens = Lexer::new("--[=123]=]\n").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((0, Token::OneLineComment("[=123]=]"), 11)), Ok((11, Token::EOF, 11))));

    let tokens = Lexer::new("--[=123]=]").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((0, Token::OneLineComment("[=123]=]"), 10)), Ok((10, Token::EOF, 10))));
}