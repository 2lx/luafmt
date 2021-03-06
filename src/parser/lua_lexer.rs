use phf::phf_map;
use std::fmt;

use super::lexer_util::*;

type TChars<'a> = std::iter::Peekable<std::iter::Enumerate<std::str::Chars<'a>>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
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

    Variable(String),
    Numeral(String),
    NormalStringLiteral(String),
    CharStringLiteral(String),
    MultiLineStringLiteral(usize, String),

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

    SheBang(String),
    EOF,
}

impl fmt::Display for Token {
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

            SheBang(s) => write!(f, "{}\n", s),
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
    chars: TChars<'input>,
    input: &'input str,
    at_end: bool,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer { chars: input.chars().enumerate().peekable(), input, at_end: false }
    }

    fn consume_ok(&mut self, l: usize, tok: Token, r: usize) -> Option<Result<(usize, Token, usize), LexicalError>> {
        self.chars.next();
        return Some(Ok((l, tok, r)));
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Token, usize), LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;
        let ok = |l: usize, tok: Token, r: usize| -> Option<Self::Item> {
            return Some(Ok((l, tok, r)));
        };

        loop {
            match self.chars.peek() {
                None => {
                    if !self.at_end {
                        self.at_end = true;
                        let index = self.input.chars().count();
                        return ok(index, EOF, index);
                    }
                    return None;
                }

                Some(&(_, ' ')) | Some(&(_, '\n')) | Some(&(_, '\r')) | Some(&(_, '\t')) => {
                    self.chars.next();
                }

                Some(&(i, '^')) => return self.consume_ok(i, OpExponentiation, i + 1),
                Some(&(i, '#')) => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some(&(_, '!')) => {
                            self.chars.next();
                            let (_, end, mut val) = get_shebang_ends(&mut self.chars, i + 2);
                            val.insert_str(0, "#!");

                            return ok(i, SheBang(val), end);
                        }
                        _ => return ok(i, OpLength, i + 1),
                    }
                }
                Some(&(i, '*')) => return self.consume_ok(i, OpMultiplication, i + 1),
                Some(&(i, '%')) => return self.consume_ok(i, OpModulo, i + 1),
                Some(&(i, '/')) => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some(&(_, '/')) => return self.consume_ok(i, OpFloorDivision, i + 2),
                        _ => return ok(i, OpDivision, i + 1),
                    }
                }

                Some(&(i, '+')) => return self.consume_ok(i, OpAddition, i + 1),
                Some(&(i, '-')) => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some(&(_, '-')) => {
                            self.chars.next();
                            let (_, _, _, _, succ, _) = get_comment_start_ends_and_type(&mut self.chars, i + 2);
                            if !succ {
                                return Some(Err(LexicalError::UnexpectedEOF));
                            }

                            continue;
                        }
                        _ => return ok(i, Minus, i + 1),
                    }
                }

                Some(&(i, '.')) => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some(&(_, '.')) => {
                            self.chars.next();

                            match self.chars.peek() {
                                Some(&(_, '.')) => return self.consume_ok(i, VarArg, i + 3),
                                _ => return ok(i, OpConcatenation, i + 2),
                            }
                        }
                        Some(&(_, ch)) if ch.is_ascii_digit() => {
                            let (end, succ, mut val) = get_float_end(&mut self.chars, i);
                            val.insert(0, '.');

                            match succ {
                                true => return ok(i, Numeral(val), end),
                                false => return Some(Err(LexicalError::UnexpectedEOF)),
                            }
                        }
                        _ => return ok(i, Period, i + 1),
                    }
                }

                Some(&(i, '<')) => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some(&(_, '<')) => return self.consume_ok(i, OpLeftShift, i + 2),
                        Some(&(_, '=')) => return self.consume_ok(i, OpLessOrEqual, i + 2),
                        _ => return ok(i, OpLessThan, i + 1),
                    }
                }

                Some(&(i, '>')) => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some(&(_, '>')) => return self.consume_ok(i, OpRightShift, i + 2),
                        Some(&(_, '=')) => return self.consume_ok(i, OpGreaterOrEqual, i + 2),
                        _ => return ok(i, OpGreaterThan, i + 1),
                    }
                }

                Some(&(i, '&')) => return self.consume_ok(i, OpBitwiseAnd, i + 1),
                Some(&(i, '~')) => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some(&(_, '=')) => return self.consume_ok(i, OpInequality, i + 2),
                        _ => return ok(i, Tilde, i + 1),
                    }
                }
                Some(&(i, '|')) => return self.consume_ok(i, OpBitwiseOr, i + 1),

                Some(&(i, '=')) => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some(&(_, '=')) => return self.consume_ok(i, OpEquality, i + 2),
                        _ => return ok(i, EqualsSign, i + 1),
                    }
                }

                Some(&(i, ';')) => return self.consume_ok(i, Semicolon, i + 1),
                Some(&(i, ',')) => return self.consume_ok(i, Comma, i + 1),
                Some(&(i, ':')) => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some(&(_, ':')) => return self.consume_ok(i, Label, i + 2),
                        _ => return ok(i, Colon, i + 1),
                    }
                }

                Some(&(i, '(')) => return self.consume_ok(i, OpenRoundBracket, i + 1),
                Some(&(i, ')')) => return self.consume_ok(i, CloseRoundBracket, i + 1),
                Some(&(i, '{')) => return self.consume_ok(i, OpenCurlyBracket, i + 1),
                Some(&(i, '}')) => return self.consume_ok(i, CloseCurlyBracket, i + 1),

                Some(&(i, ']')) => return self.consume_ok(i, CloseSquareBracket, i + 1),
                Some(&(token_start, '[')) => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some(&(level_start, '=')) => {
                            let level = get_multiline_string_level(&mut self.chars, level_start);
                            match self.chars.peek() {
                                Some(&(square_2_start, '[')) => {
                                    self.chars.next();
                                    let text_start = square_2_start + 1;
                                    let (_, token_end, succ, val) =
                                        get_multiline_string_ends(&mut self.chars, level, text_start);

                                    match succ {
                                        true => {
                                            let token = MultiLineStringLiteral(level, val);
                                            return ok(token_start, token, token_end);
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
                            let (_, token_end, succ, val) = get_multiline_string_ends(&mut self.chars, 0, text_start);

                            match succ {
                                true => {
                                    let token = MultiLineStringLiteral(0, val);
                                    return ok(token_start, token, token_end);
                                }
                                false => return Some(Err(LexicalError::UnexpectedEOF)),
                            }
                        }
                        _ => return ok(token_start, OpenSquareBracket, token_start + 1),
                    }
                }

                Some(&(i, ch @ '"')) | Some(&(i, ch @ '\'')) => {
                    self.chars.next();
                    let (_, token_end, succ, val) = get_string_ends(&mut self.chars, ch, i);
                    match (succ, ch) {
                        (true, '\'') => return ok(i, CharStringLiteral(val), token_end),
                        (true, '"') => return ok(i, NormalStringLiteral(val), token_end),
                        _ => return Some(Err(LexicalError::UnexpectedEOF)),
                    }
                }

                Some(&(i, '0')) => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some(&(_, 'x')) => {
                            self.chars.next();
                            let (end, succ, mut val) = get_hex_integer_end(&mut self.chars, i + 2);
                            val.insert_str(0, "0x");

                            match succ {
                                true => return ok(i, Numeral(val), end),
                                false => return Some(Err(LexicalError::UnexpectedEOF)),
                            }
                        }
                        _ => {
                            let (end, succ, mut val) = get_float_end(&mut self.chars, i + 1);
                            val.insert(0, '0');

                            match succ {
                                true => return ok(i, Numeral(val), end),
                                false => return Some(Err(LexicalError::UnexpectedEOF)),
                            }
                        }
                    }
                }

                Some(&(i, ch)) if ch.is_ascii_digit() => {
                    self.chars.next();
                    let (end, succ, mut val) = get_float_end(&mut self.chars, i + 1);
                    val.insert(0, ch);

                    match succ {
                        true => return ok(i, Numeral(val), end),
                        false => return Some(Err(LexicalError::UnexpectedEOF)),
                    }
                }

                Some(&(i, ch)) if ch.is_ascii_alphabetic() || ch == '_' => {
                    let (end, succ, val) = get_variable_end(&mut self.chars, i);
                    match succ {
                        true => {
                            match KEYWORDS.get(&val[..]) {
                                Some(w) => return ok(i, w.clone(), end),
                                _ => return ok(i, Variable(val), end),
                            };
                        }
                        false => return Some(Err(LexicalError::UnexpectedEOF)),
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
fn test_lua_lexer() {
    type TRes<'a> = Vec<Result<(usize, Token, usize), LexicalError>>;
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

    let tokens = Lexer::new("\n#!/usr/bin/lua\n  ").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((1, SheBang("#!/usr/bin/lua".to_string()), 16)), Ok((18, EOF, 18))));

    let tokens = Lexer::new("  a = b  ").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Ok((2, Variable("a".to_string()), 3)),
            Ok((4, EqualsSign, 5)),
            Ok((6, Variable("b".to_string()), 7)),
            Ok((9, EOF, 9))
        )
    );

    let tokens = Lexer::new("a = \"st'ri'[[n]]g\"").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Ok((0, Variable("a".to_string()), 1)),
            Ok((2, EqualsSign, 3)),
            Ok((4, NormalStringLiteral("st'ri'[[n]]g".to_string()), 18)),
            Ok((18, EOF, 18))
        )
    );

    let tokens = Lexer::new("a = '[[s]]t\"ri\"ng'").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Ok((0, Variable("a".to_string()), 1)),
            Ok((2, EqualsSign, 3)),
            Ok((4, CharStringLiteral("[[s]]t\"ri\"ng".to_string()), 18)),
            Ok((18, EOF, 18))
        )
    );

    let tokens = Lexer::new("a = [[]]").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Ok((0, Variable("a".to_string()), 1)),
            Ok((2, EqualsSign, 3)),
            Ok((4, MultiLineStringLiteral(0, "".to_string()), 8)),
            Ok((8, EOF, 8))
        )
    );

    let tokens = Lexer::new("a = [[st\"r'i\"n'g]]").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Ok((0, Variable("a".to_string()), 1)),
            Ok((2, EqualsSign, 3)),
            Ok((4, MultiLineStringLiteral(0, "st\"r'i\"n'g".to_string()), 18)),
            Ok((18, EOF, 18))
        )
    );

    let tokens = Lexer::new("a = [=[]=]").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Ok((0, Variable("a".to_string()), 1)),
            Ok((2, EqualsSign, 3)),
            Ok((4, MultiLineStringLiteral(1, "".to_string()), 10)),
            Ok((10, EOF, 10))
        )
    );

    let tokens = Lexer::new("a = [===[st\"r'i\"n'g]===]").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Ok((0, Variable("a".to_string()), 1)),
            Ok((2, EqualsSign, 3)),
            Ok((4, MultiLineStringLiteral(3, "st\"r'i\"n'g".to_string()), 24)),
            Ok((24, EOF, 24))
        )
    );

    let tokens = Lexer::new("[===[]=]]]]==]==]===]").collect::<TRes>();
    assert_eq!(tokens, vec!(Ok((0, MultiLineStringLiteral(3, "]=]]]]==]==".to_string()), 21)), Ok((21, EOF, 21))));

    let tokens = Lexer::new("for a in pairs(tbl) do x.fn(a) end").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Ok((0, For, 3)),
            Ok((4, Variable("a".to_string()), 5)),
            Ok((6, In, 8)),
            Ok((9, Variable("pairs".to_string()), 14)),
            Ok((14, OpenRoundBracket, 15)),
            Ok((15, Variable("tbl".to_string()), 18)),
            Ok((18, CloseRoundBracket, 19)),
            Ok((20, Do, 22)),
            Ok((23, Variable("x".to_string()), 24)),
            Ok((24, Period, 25)),
            Ok((25, Variable("fn".to_string()), 27)),
            Ok((27, OpenRoundBracket, 28)),
            Ok((28, Variable("a".to_string()), 29)),
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
            Ok((4, Variable("a".to_string()), 5)),
            Ok((6, EqualsSign, 7)),
            Ok((8, Numeral("5".to_string()), 9)),
            Ok((9, Comma, 10)),
            Ok((11, Numeral("1".to_string()), 12)),
            Ok((12, Comma, 13)),
            Ok((14, Minus, 15)),
            Ok((15, Numeral("1".to_string()), 16)),
            Ok((17, Do, 19)),
            Ok((20, Variable("x".to_string()), 21)),
            Ok((21, Period, 22)),
            Ok((22, Variable("fn".to_string()), 24)),
            Ok((24, OpenRoundBracket, 25)),
            Ok((25, Variable("a".to_string()), 26)),
            Ok((26, CloseRoundBracket, 27)),
            Ok((28, End, 31)),
            Ok((31, EOF, 31))
        )
    );

    let tokens = Lexer::new("a = ({a=3}).a").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Ok((0, Variable("a".to_string()), 1)),
            Ok((2, EqualsSign, 3)),
            Ok((4, OpenRoundBracket, 5)),
            Ok((5, OpenCurlyBracket, 6)),
            Ok((6, Variable("a".to_string()), 7)),
            Ok((7, EqualsSign, 8)),
            Ok((8, Numeral("3".to_string()), 9)),
            Ok((9, CloseCurlyBracket, 10)),
            Ok((10, CloseRoundBracket, 11)),
            Ok((11, Period, 12)),
            Ok((12, Variable("a".to_string()), 13)),
            Ok((13, EOF, 13))
        )
    );

    let tokens = Lexer::new("c = a--\n+--[[342]]b").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Ok((0, Variable("c".to_string()), 1)),
            Ok((2, EqualsSign, 3)),
            Ok((4, Variable("a".to_string()), 5)),
            Ok((8, OpAddition, 9)),
            Ok((18, Variable("b".to_string()), 19)),
            Ok((19, EOF, 19))
        )
    );

    // test unicode
    let source = r#"local tbl = { field1 = "Какой-то текст", field2 = "Some text" }"#;
    let tokens = Lexer::new(source).collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Ok((0, Local, 5)),
            Ok((6, Variable("tbl".to_string()), 9)),
            Ok((10, EqualsSign, 11)),
            Ok((12, OpenCurlyBracket, 13)),
            Ok((14, Variable("field1".to_string()), 20)),
            Ok((21, EqualsSign, 22)),
            Ok((23, NormalStringLiteral("Какой-то текст".to_string()), 39)),
            Ok((39, Comma, 40)),
            Ok((41, Variable("field2".to_string()), 47)),
            Ok((48, EqualsSign, 49)),
            Ok((50, NormalStringLiteral("Some text".to_string()), 61)),
            Ok((62, CloseCurlyBracket, 63)),
            Ok((63, EOF, 63))
        )
    );
}

#[test]
fn test_lua_lexer_errors() {
    type TRes<'a> = Vec<Result<(usize, Token, usize), LexicalError>>;
    use LexicalError::*;
    use Token::*;

    let tokens = Lexer::new("a = [[str").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(Ok((0, Variable("a".to_string()), 1)), Ok((2, EqualsSign, 3)), Err(UnexpectedEOF), Ok((9, EOF, 9)))
    );

    let tokens = Lexer::new("--[[string").collect::<TRes>();
    assert_eq!(tokens, vec!(Err(UnexpectedEOF), Ok((10, EOF, 10))));

    let tokens = Lexer::new("a = 0.123e").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(Ok((0, Variable("a".to_string()), 1)), Ok((2, EqualsSign, 3)), Err(UnexpectedEOF), Ok((10, EOF, 10)))
    );

    let tokens = Lexer::new("a = 0.123e-").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(Ok((0, Variable("a".to_string()), 1)), Ok((2, EqualsSign, 3)), Err(UnexpectedEOF), Ok((11, EOF, 11)))
    );

    let tokens = Lexer::new("a = \"string").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(Ok((0, Variable("a".to_string()), 1)), Ok((2, EqualsSign, 3)), Err(UnexpectedEOF), Ok((11, EOF, 11)))
    );

    let tokens = Lexer::new("a = 'string").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(Ok((0, Variable("a".to_string()), 1)), Ok((2, EqualsSign, 3)), Err(UnexpectedEOF), Ok((11, EOF, 11)))
    );

    let tokens = Lexer::new("a = [[strin").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(Ok((0, Variable("a".to_string()), 1)), Ok((2, EqualsSign, 3)), Err(UnexpectedEOF), Ok((11, EOF, 11)))
    );

    let tokens = Lexer::new("a = `").collect::<TRes>();
    assert_eq!(
        tokens,
        vec!(
            Ok((0, Variable("a".to_string()), 1)),
            Ok((2, EqualsSign, 3)),
            Err(UnrecognizedSymbol(4, '`')),
            Ok((5, EOF, 5))
        )
    );
}
