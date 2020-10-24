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

    fn consume_ok(
        &mut self,
        l: usize,
        tok: Token<'input>,
        r: usize,
    ) -> Option<Result<(usize, Token<'input>, usize), LexicalError>> {
        self.chars.next();
        return Some(Ok((l, tok, r)));
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Token<'input>, usize), LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;
        let ok = |l: usize, tok: Token<'input>, r: usize| -> Option<Self::Item> {
            return Some(Ok((l, tok, r)));
        };

        loop {
            match self.chars.peek() {
                None => {
                    if !self.at_end {
                        self.at_end = true;
                        return ok(self.input.len(), EOF, self.input.len());
                    }
                    return None;
                }
                Some(&(_, ' ')) | Some(&(_, '\r')) | Some(&(_, '\t')) => {
                    self.chars.next();
                }

                Some(&(i, '\n')) => return self.consume_ok(i, NewLine, i + 1),
                Some(&(token_start, '-')) => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some(&(_, '-')) => {
                            self.chars.next();

                            let (text_start, text_end, token_end, opt_level, succ) =
                                get_comment_start_ends_and_type(&mut self.chars, token_start + 2);
                            if !succ {
                                return Some(Err(LexicalError::UnexpectedEOF));
                            }

                            let token;
                            match opt_level {
                                Some(level) => token = MultiLineComment(level, &self.input[text_start..text_end]),
                                None => token = OneLineComment(&self.input[text_start..text_end]),
                            };
                            return ok(token_start, token, token_end);
                        }
                        Some(&(ip, chp)) => return Some(Err(LexicalError::UnrecognizedSymbol(ip, chp))),
                        None => return Some(Err(LexicalError::UnexpectedEOF)),
                    }
                }

                Some(&(i, ch)) => {
                    self.chars.next();
                    return Some(Err(LexicalError::UnrecognizedSymbol(i, ch)));
                }
            }
        }
    }
}

#[test]
fn test_comment_lexer() {
    type TRes<'a> = Vec<Result<(usize, Token<'a>, usize), LexicalError>>;
    use Token::*;

    let tokens = Lexer::new("").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((0, EOF, 0))));

    let tokens = Lexer::new("  ").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((2, EOF, 2))));

    let tokens = Lexer::new("  \n  ").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((2, NewLine, 3)), Ok((5, EOF, 5))));

    let tokens = Lexer::new("\n   \n").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((0, NewLine, 1)), Ok((4, NewLine, 5)), Ok((5, EOF, 5))));

    let tokens = Lexer::new("--\n").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((0, OneLineComment(""), 3)), Ok((3, EOF, 3))));

    let tokens = Lexer::new("--123\n").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((0, OneLineComment("123"), 6)), Ok((6, EOF, 6))));

    let tokens = Lexer::new("--123").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((0, OneLineComment("123"), 5)), Ok((5, EOF, 5))));

    let tokens = Lexer::new("  --  123  \n  ").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((2, OneLineComment("  123  "), 12)), Ok((14, EOF, 14))));

    let tokens = Lexer::new("--[[]]").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((0, MultiLineComment(0, ""), 6)), Ok((6, EOF, 6))));

    let tokens = Lexer::new("--[=[123]=]").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((0, MultiLineComment(1, "123"), 11)), Ok((11, EOF, 11))));

    let tokens = Lexer::new("--[=123]=]\n").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((0, OneLineComment("[=123]=]"), 11)), Ok((11, EOF, 11))));

    let tokens = Lexer::new("\n\n  --123\n").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(Ok((0, NewLine, 1)), Ok((1, NewLine, 2)), Ok((4, OneLineComment("123"), 10)), Ok((10, EOF, 10)))
    );

    let tokens = Lexer::new("--[=123]=]").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((0, OneLineComment("[=123]=]"), 10)), Ok((10, EOF, 10))));

    let tokens = Lexer::new("--[=123]=]\n--[=[]=]").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(Ok((0, OneLineComment("[=123]=]"), 11)), Ok((11, MultiLineComment(1, ""), 19)), Ok((19, EOF, 19)))
    );

    let tokens = Lexer::new("--12345678\n--[=]=]\n").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(Ok((0, OneLineComment("12345678"), 11)), Ok((11, OneLineComment("[=]=]"), 19)), Ok((19, EOF, 19)))
    );

    let tokens = Lexer::new("--[===[trtstrst]====]==]==]=]]]========]==]========]===]").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(Ok((0, MultiLineComment(3, "trtstrst]====]==]==]=]]]========]==]========"), 56)), Ok((56, EOF, 56)))
    );
}

#[test]
fn test_comment_lexer_errors() {
    type TRes<'a> = Vec<Result<(usize, Token<'a>, usize), LexicalError>>;
    use LexicalError::*;
    use Token::*;

    let tokens = Lexer::new("`").collect::<TRes>();
    assert_eq!(tokens, vec!(Err(UnrecognizedSymbol(0, '`')), Ok((1, EOF, 1))));

    let tokens = Lexer::new("a = b").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Err(UnrecognizedSymbol(0, 'a')),
            Err(UnrecognizedSymbol(2, '=')),
            Err(UnrecognizedSymbol(4, 'b')),
            Ok((5, EOF, 5))
        )
    );
}
