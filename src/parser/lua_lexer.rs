use super::lexer_util::*;
use phf::phf_map;
use std::fmt;
use std::str::CharIndices;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'input> {
    OpExponentiation,
    OpLogicalNot,
    OpLength,
    OpMultiplication,
    OpDivision,
    OpFloorDivision,
    OpModulo,
    OpAddition,
    Minus,
    OpConcatenation,
    OpLeftShift,
    OpRightShift,
    OpBitwiseAnd,
    Tilde,
    OpBitwiseOr,
    OpEquality,
    OpInequality,
    OpLessThan,
    OpGreaterThan,
    OpLessOrEqual,
    OpGreaterOrEqual,
    OpLogicalAnd,
    OpLogicalOr,

    Variable(&'input str),
    Numeral(&'input str),
    NormalStringLiteral(&'input str),
    CharStringLiteral(&'input str),
    MultiLineStringLiteral(usize, &'input str),

    Semicolon,
    Comma,
    Colon,
    Label,
    EqualsSign,
    Period,
    OpenRoundBracket,
    CloseRoundBracket,
    OpenSquareBracket,
    CloseSquareBracket,
    OpenCurlyBracket,
    CloseCurlyBracket,

    Break,
    Do,
    Else,
    ElseIf,
    End,
    False,
    For,
    Function,
    GoTo,
    If,
    In,
    Local,
    Nil,
    Repeat,
    Return,
    Then,
    True,
    Until,
    VarArg,
    While,

    EOF,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Token::*;
        match self {
            OpExponentiation => write!(f, "^"),
            OpLogicalNot => write!(f, "not"),
            OpLength => write!(f, "#"),
            OpMultiplication => write!(f, "*"),
            OpDivision => write!(f, "/"),
            OpFloorDivision => write!(f, "//"),
            OpModulo => write!(f, "%"),
            OpAddition => write!(f, "+"),
            Minus => write!(f, "-"),
            OpConcatenation => write!(f, ".."),
            OpLeftShift => write!(f, "<<"),
            OpRightShift => write!(f, ">>"),
            OpBitwiseAnd => write!(f, "&"),
            Tilde => write!(f, "~"),
            OpBitwiseOr => write!(f, "|"),
            OpEquality => write!(f, "=="),
            OpInequality => write!(f, "~="),
            OpLessThan => write!(f, "<"),
            OpGreaterThan => write!(f, ">"),
            OpLessOrEqual => write!(f, "<="),
            OpGreaterOrEqual => write!(f, ">="),
            OpLogicalAnd => write!(f, "and"),
            OpLogicalOr => write!(f, "or"),

            Variable(s) => write!(f, "\"{}\"", s),
            Numeral(n) => write!(f, "\"{}\"", n),
            NormalStringLiteral(s) => write!(f, "\"{}\"", s),
            CharStringLiteral(s) => write!(f, "'{}'", s),
            MultiLineStringLiteral(level, s) => {
                let level_str = (0..*level).map(|_| "=").collect::<String>();
                write!(f, "[{}[{}]{}]", level_str, s, level_str)
            }

            Semicolon => write!(f, ";"),
            Comma => write!(f, ","),
            Colon => write!(f, ":"),
            Label => write!(f, "::"),
            EqualsSign => write!(f, "="),
            Period => write!(f, "."),
            OpenRoundBracket => write!(f, "("),
            CloseRoundBracket => write!(f, ")"),
            OpenSquareBracket => write!(f, "["),
            CloseSquareBracket => write!(f, "]"),
            OpenCurlyBracket => write!(f, "{{"),
            CloseCurlyBracket => write!(f, "}}"),

            Break => write!(f, "break"),
            Do => write!(f, "do"),
            Else => write!(f, "else"),
            ElseIf => write!(f, "elseif"),
            End => write!(f, "end"),
            False => write!(f, "false"),
            For => write!(f, "for"),
            Function => write!(f, "function"),
            GoTo => write!(f, "goto"),
            If => write!(f, "if"),
            In => write!(f, "in"),
            Local => write!(f, "local"),
            Nil => write!(f, "nil"),
            Repeat => write!(f, "repeat"),
            Return => write!(f, "return"),
            Then => write!(f, "then"),
            True => write!(f, "true"),
            Until => write!(f, "until"),
            VarArg => write!(f, "..."),
            While => write!(f, "while"),

            EOF => write!(f, "<EOF>"),
        }
    }
}

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "and"      => Token::OpLogicalAnd,
    "break"    => Token::Break,
    "do"       => Token::Do,
    "else"     => Token::Else,
    "elseif"   => Token::ElseIf,
    "end"      => Token::End,
    "false"    => Token::False,
    "for"      => Token::For,
    "function" => Token::Function,
    "goto"     => Token::GoTo,
    "if"       => Token::If,
    "in"       => Token::In,
    "local"    => Token::Local,
    "nil"      => Token::Nil,
    "not"      => Token::OpLogicalNot,
    "or"       => Token::OpLogicalOr,
    "repeat"   => Token::Repeat,
    "return"   => Token::Return,
    "then"     => Token::Then,
    "true"     => Token::True,
    "until"    => Token::Until,
    "while"    => Token::While,
};

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

                Some((_, ' ')) | Some((_, '\n')) | Some((_, '\r')) | Some((_, '\t')) => continue,

                Some((i, '^')) => return Some(Ok((i, OpExponentiation, i + 1))),
                Some((i, '#')) => return Some(Ok((i, OpLength, i + 1))),
                Some((i, '*')) => return Some(Ok((i, OpMultiplication, i + 1))),
                Some((i, '%')) => return Some(Ok((i, OpModulo, i + 1))),
                Some((i, '/')) => match self.chars.peek() {
                    Some(&(_, '/')) => {
                        self.chars.next();
                        return Some(Ok((i, OpFloorDivision, i + 2)));
                    }
                    _ => return Some(Ok((i, OpDivision, i + 1))),
                },

                Some((i, '+')) => return Some(Ok((i, OpAddition, i + 1))),
                Some((i, '-')) => match self.chars.peek() {
                    Some(&(_, '-')) => {
                        self.chars.next();
                        get_comment_start_ends_and_type(&mut self.chars, i + 2);

                        continue;
                    }
                    _ => return Some(Ok((i, Minus, i + 1))),
                },

                Some((i, '.')) => match self.chars.peek() {
                    Some(&(_, '.')) => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some(&(_, '.')) => {
                                self.chars.next();
                                return Some(Ok((i, VarArg, i + 3)));
                            }
                            _ => return Some(Ok((i, OpConcatenation, i + 2))),
                        }
                    }
                    Some(&(_, ch)) if ch.is_ascii_digit() => {
                        let (end, succ) = get_float_end(&mut self.chars, i);
                        match succ {
                            true => return Some(Ok((i, Numeral(&self.input[i..end]), end))),
                            false => return Some(Err(LexicalError::UnexpectedEOF)),
                        }
                    }
                    _ => return Some(Ok((i, Period, i + 1))),
                },

                Some((i, '<')) => match self.chars.peek() {
                    Some(&(_, '<')) => {
                        self.chars.next();
                        return Some(Ok((i, OpLeftShift, i + 2)));
                    }
                    Some(&(_, '=')) => {
                        self.chars.next();
                        return Some(Ok((i, OpLessOrEqual, i + 2)));
                    }
                    _ => return Some(Ok((i, OpLessThan, i + 1))),
                },

                Some((i, '>')) => match self.chars.peek() {
                    Some(&(_, '>')) => {
                        self.chars.next();
                        return Some(Ok((i, OpRightShift, i + 2)));
                    }
                    Some(&(_, '=')) => {
                        self.chars.next();
                        return Some(Ok((i, OpGreaterOrEqual, i + 2)));
                    }
                    _ => return Some(Ok((i, OpGreaterThan, i + 1))),
                },

                Some((i, '&')) => return Some(Ok((i, OpBitwiseAnd, i + 1))),
                Some((i, '~')) => match self.chars.peek() {
                    Some(&(_, '=')) => {
                        self.chars.next();
                        return Some(Ok((i, OpInequality, i + 2)));
                    }
                    _ => return Some(Ok((i, Tilde, i + 1))),
                },
                Some((i, '|')) => return Some(Ok((i, OpBitwiseOr, i + 1))),

                Some((i, '=')) => match self.chars.peek() {
                    Some(&(_, '=')) => {
                        self.chars.next();
                        return Some(Ok((i, OpEquality, i + 2)));
                    }
                    _ => return Some(Ok((i, EqualsSign, i + 1))),
                },

                Some((i, ';')) => return Some(Ok((i, Semicolon, i + 1))),
                Some((i, ',')) => return Some(Ok((i, Comma, i + 1))),
                Some((i, ':')) => match self.chars.peek() {
                    Some(&(_, ':')) => {
                        self.chars.next();
                        return Some(Ok((i, Label, i + 2)));
                    }
                    _ => return Some(Ok((i, Colon, i + 1))),
                },

                Some((i, '(')) => return Some(Ok((i, OpenRoundBracket, i + 1))),
                Some((i, ')')) => return Some(Ok((i, CloseRoundBracket, i + 1))),
                Some((i, '{')) => return Some(Ok((i, OpenCurlyBracket, i + 1))),
                Some((i, '}')) => return Some(Ok((i, CloseCurlyBracket, i + 1))),

                Some((i, ']')) => return Some(Ok((i, CloseSquareBracket, i + 1))),
                Some((token_start, '[')) => match self.chars.peek() {
                    Some(&(level_start, '=')) => {
                        let level = get_multiline_string_level(&mut self.chars, level_start);
                        match self.chars.peek() {
                            Some(&(square_2_start, '[')) => {
                                self.chars.next();
                                let text_start = square_2_start + 1;
                                let (text_end, token_end, succ) =
                                    get_multiline_string_ends(&mut self.chars, level, text_start);

                                match succ {
                                    true => {
                                        return Some(Ok((
                                            token_start,
                                            MultiLineStringLiteral(level, &self.input[text_start..text_end]),
                                            token_end,
                                        )))
                                    }
                                    false => return Some(Err(LexicalError::UnexpectedEOF)),
                                }
                            }
                            Some((chi, chu)) => return Some(Err(LexicalError::UnrecognizedSymbol(*chi, *chu))),
                            None => return Some(Err(LexicalError::UnexpectedEOF)),
                        }
                    }
                    Some(&(square_2_start, '[')) => {
                        self.chars.next();
                        let text_start = square_2_start + 1;
                        let (text_end, token_end, succ) = get_multiline_string_ends(&mut self.chars, 0, text_start);

                        match succ {
                            true => {
                                return Some(Ok((
                                    token_start,
                                    MultiLineStringLiteral(0, &self.input[text_start..text_end]),
                                    token_end,
                                )))
                            }
                            false => return Some(Err(LexicalError::UnexpectedEOF)),
                        }
                    }
                    _ => return Some(Ok((token_start, OpenSquareBracket, token_start + 1))),
                },

                Some((i, '"')) => {
                    let (text_end, token_end, succ) = get_string_ends(&mut self.chars, '"', i);
                    match succ {
                        true => return Some(Ok((i, NormalStringLiteral(&self.input[i + 1..text_end]), token_end))),
                        false => return Some(Err(LexicalError::UnexpectedEOF)),
                    }
                }

                Some((i, '\'')) => {
                    let (text_end, token_end, succ) = get_string_ends(&mut self.chars, '\'', i);
                    match succ {
                        true => return Some(Ok((i, CharStringLiteral(&self.input[i + 1..text_end]), token_end))),
                        false => return Some(Err(LexicalError::UnexpectedEOF)),
                    }
                }

                Some((i, '0')) => match self.chars.peek() {
                    Some(&(_, 'x')) => {
                        self.chars.next();
                        let (end, succ) = get_hex_integer_end(&mut self.chars, i + 2);
                        match succ {
                            true => return Some(Ok((i, Numeral(&self.input[i..end]), end))),
                            false => return Some(Err(LexicalError::UnexpectedEOF)),
                        }
                    }
                    _ => {
                        let (end, succ) = get_float_end(&mut self.chars, i + 1);
                        match succ {
                            true => return Some(Ok((i, Numeral(&self.input[i..end]), end))),
                            false => return Some(Err(LexicalError::UnexpectedEOF)),
                        }
                    }
                },

                Some((i, ch)) if ch.is_ascii_digit() => {
                    let (end, succ) = get_float_end(&mut self.chars, i + 1);
                    match succ {
                        true => return Some(Ok((i, Numeral(&self.input[i..end]), end))),
                        false => return Some(Err(LexicalError::UnexpectedEOF)),
                    }
                }

                Some((i, ch)) if ch.is_ascii_alphabetic() || ch == '_' => {
                    let (end, succ) = get_variable_end(&mut self.chars, i + 1);
                    match succ {
                        true => {
                            let variable = &self.input[i..end];

                            match KEYWORDS.get(variable) {
                                Some(w) => return Some(Ok((i, *w, end))),
                                _ => return Some(Ok((i, Variable(&self.input[i..end]), end))),
                            };
                        }
                        false => return Some(Err(LexicalError::UnexpectedEOF)),
                    }
                }

                Some((i, ch)) => return Some(Err(LexicalError::UnrecognizedSymbol(i, ch))),
            }
        }
    }
}

#[test]
fn test_lua_lexer() {
    type TRes<'a> = Vec<Result<(usize, Token<'a>, usize), LexicalError>>;
    use Token::*;

    let tokens = Lexer::new("").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((0, EOF, 0))));

    let tokens = Lexer::new("  ").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((2, EOF, 2))));

    let tokens = Lexer::new("  \n  ").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((5, EOF, 5))));

    let tokens = Lexer::new("\n   \n").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((5, EOF, 5))));

    let tokens = Lexer::new("--123\n--[[54354]]").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((17, EOF, 17))));

    let tokens = Lexer::new("  a = b  ").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(Ok((2, Variable("a"), 3)), Ok((4, EqualsSign, 5)), Ok((6, Variable("b"), 7)), Ok((9, EOF, 9)))
    );

    let tokens = Lexer::new("a = \"st'ri'[[n]]g\"").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Ok((0, Variable("a"), 1)),
            Ok((2, EqualsSign, 3)),
            Ok((4, NormalStringLiteral("st'ri'[[n]]g"), 18)),
            Ok((18, EOF, 18))
        )
    );

    let tokens = Lexer::new("a = '[[s]]t\"ri\"ng'").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Ok((0, Variable("a"), 1)),
            Ok((2, EqualsSign, 3)),
            Ok((4, CharStringLiteral("[[s]]t\"ri\"ng"), 18)),
            Ok((18, EOF, 18))
        )
    );

    let tokens = Lexer::new("a = [[]]").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Ok((0, Variable("a"), 1)),
            Ok((2, EqualsSign, 3)),
            Ok((4, MultiLineStringLiteral(0, ""), 8)),
            Ok((8, EOF, 8))
        )
    );

    let tokens = Lexer::new("a = [[st\"r'i\"n'g]]").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Ok((0, Variable("a"), 1)),
            Ok((2, EqualsSign, 3)),
            Ok((4, MultiLineStringLiteral(0, "st\"r'i\"n'g"), 18)),
            Ok((18, EOF, 18))
        )
    );

    let tokens = Lexer::new("a = [=[]=]").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Ok((0, Variable("a"), 1)),
            Ok((2, EqualsSign, 3)),
            Ok((4, MultiLineStringLiteral(1, ""), 10)),
            Ok((10, EOF, 10))
        )
    );

    let tokens = Lexer::new("a = [===[st\"r'i\"n'g]===]").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Ok((0, Variable("a"), 1)),
            Ok((2, EqualsSign, 3)),
            Ok((4, MultiLineStringLiteral(3, "st\"r'i\"n'g"), 24)),
            Ok((24, EOF, 24))
        )
    );

    let tokens = Lexer::new("[===[]=]]]]==]==]===]").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((0, MultiLineStringLiteral(3, "]=]]]]==]=="), 21)), Ok((21, EOF, 21))));

    let tokens = Lexer::new("for a in pairs(tbl) do x.fn(a) end").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Ok((0, For, 3)),
            Ok((4, Variable("a"), 5)),
            Ok((6, In, 8)),
            Ok((9, Variable("pairs"), 14)),
            Ok((14, OpenRoundBracket, 15)),
            Ok((15, Variable("tbl"), 18)),
            Ok((18, CloseRoundBracket, 19)),
            Ok((20, Do, 22)),
            Ok((23, Variable("x"), 24)),
            Ok((24, Period, 25)),
            Ok((25, Variable("fn"), 27)),
            Ok((27, OpenRoundBracket, 28)),
            Ok((28, Variable("a"), 29)),
            Ok((29, CloseRoundBracket, 30)),
            Ok((31, End, 34)),
            Ok((34, EOF, 34))
        )
    );

    let tokens = Lexer::new("for a = 5, 1, -1 do x.fn(a) end").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Ok((0, For, 3)),
            Ok((4, Variable("a"), 5)),
            Ok((6, EqualsSign, 7)),
            Ok((8, Numeral("5"), 9)),
            Ok((9, Comma, 10)),
            Ok((11, Numeral("1"), 12)),
            Ok((12, Comma, 13)),
            Ok((14, Minus, 15)),
            Ok((15, Numeral("1"), 16)),
            Ok((17, Do, 19)),
            Ok((20, Variable("x"), 21)),
            Ok((21, Period, 22)),
            Ok((22, Variable("fn"), 24)),
            Ok((24, OpenRoundBracket, 25)),
            Ok((25, Variable("a"), 26)),
            Ok((26, CloseRoundBracket, 27)),
            Ok((28, End, 31)),
            Ok((31, EOF, 31))
        )
    );

    let tokens = Lexer::new("a = ({a=3}).a").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Ok((0, Variable("a"), 1)),
            Ok((2, EqualsSign, 3)),
            Ok((4, OpenRoundBracket, 5)),
            Ok((5, OpenCurlyBracket, 6)),
            Ok((6, Variable("a"), 7)),
            Ok((7, EqualsSign, 8)),
            Ok((8, Numeral("3"), 9)),
            Ok((9, CloseCurlyBracket, 10)),
            Ok((10, CloseRoundBracket, 11)),
            Ok((11, Period, 12)),
            Ok((12, Variable("a"), 13)),
            Ok((13, EOF, 13))
        )
    );
}
