use phf::phf_map;
use std::fmt;
use std::str::CharIndices;

#[derive(Clone, Copy, Debug)]
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
    MultilineStringLiteral(usize, &'input str),

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
            MultilineStringLiteral(level, s) => {
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

#[derive(Debug)]
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
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices().peekable(),
            input,
        }
    }

    fn get_number_end(&mut self, start: usize) -> usize {
        let mut end = start;

        while let Some((i, ch)) = self.chars.peek() {
            if !ch.is_ascii_digit() && *ch != '.' {
                break;
            }
            end = *i;
            self.chars.next();
        }

        end + 1
    }

    fn get_variable_end(&mut self, start: usize) -> usize {
        let mut end = start;

        while let Some((i, ch)) = self.chars.peek() {
            if !ch.is_ascii_alphabetic() && !ch.is_ascii_digit() && *ch != '_' {
                break;
            }
            end = *i;
            self.chars.next();
        }

        end + 1
    }

    fn get_string_end(&mut self, prefix: char, start: usize) -> usize {
        let mut end = start;
        let mut escaped = false;

        while let Some((i, ch)) = self.chars.next() {
            end = i;
            if !escaped && ch == prefix {
                break;
            }
            escaped = ch == '\\';
        }

        end + 1
    }

    fn get_oneline_comment_end(&mut self, start: usize) -> usize {
        let mut end = start;

        while let Some((i, ch)) = self.chars.next() {
            end = i;
            if ch == '\n' {
                break;
            }
        }
        end + 1
    }

    fn get_multiline_string_level(&mut self, start: usize) -> (usize, usize) {
        let mut end = start;
        let mut level: usize = 0;

        while let Some(&(i, ch)) = self.chars.peek() {
            end = i;
            if ch != '=' {
                break;
            }
            level += 1;
            self.chars.next();
        }
        (end, level)
    }

    fn get_multiline_string_end(&mut self, level: usize, start: usize) -> usize {
        let mut end = start;
        let mut escaped = false;

        while let Some((i, ch)) = self.chars.next() {
            end = i;
            if !escaped && ch == ']' {
                let (_, cur_level) = self.get_multiline_string_level(i);

                if level == cur_level {
                    match self.chars.peek() {
                        Some((_, ']')) => {
                            self.chars.next();
                            break;
                        }
                        _ => (),
                    }
                }
            }
            escaped = ch == '\\';
        }

        end + 1
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Token<'input>, usize), LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;
        loop {
            match self.chars.next() {
                None => return None, // end of file

                Some((_, ' ')) | Some((_, '\n')) | Some((_, '\r')) | Some((_, '\t')) => continue,

                Some((i, '^')) => return Some(Ok((i, OpExponentiation, i + 1))),
                Some((i, '#')) => return Some(Ok((i, OpLength, i + 1))),

                Some((i, '*')) => return Some(Ok((i, OpMultiplication, i + 1))),
                Some((i, '%')) => return Some(Ok((i, OpModulo, i + 1))),
                Some((i, '/')) => match self.chars.peek() {
                    Some((_, '/')) => {
                        self.chars.next();
                        return Some(Ok((i, OpFloorDivision, i + 2)));
                    }
                    _ => return Some(Ok((i, OpDivision, i + 1))),
                },

                Some((i, '+')) => return Some(Ok((i, OpAddition, i + 1))),
                Some((i, '-')) => match self.chars.peek() {
                    Some((_, '-')) => {
                        let _ = self.get_oneline_comment_end(i);
                        continue;
                    }
                    _ => return Some(Ok((i, Minus, i + 1))),
                },

                Some((i, '.')) => match self.chars.peek() {
                    Some((_, '.')) => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some((_, '.')) => {
                                self.chars.next();
                                return Some(Ok((i, VarArg, i + 3)));
                            }
                            _ => return Some(Ok((i, OpConcatenation, i + 2))),
                        }
                    }
                    _ => return Some(Ok((i, Period, i + 1))),
                },

                Some((i, '<')) => match self.chars.peek() {
                    Some((_, '<')) => {
                        self.chars.next();
                        return Some(Ok((i, OpLeftShift, i + 2)));
                    }
                    Some((_, '=')) => {
                        self.chars.next();
                        return Some(Ok((i, OpLessOrEqual, i + 2)));
                    }
                    _ => return Some(Ok((i, OpLessThan, i + 1))),
                },

                Some((i, '>')) => match self.chars.peek() {
                    Some((_, '>')) => {
                        self.chars.next();
                        return Some(Ok((i, OpRightShift, i + 2)));
                    }
                    Some((_, '=')) => {
                        self.chars.next();
                        return Some(Ok((i, OpGreaterOrEqual, i + 2)));
                    }
                    _ => return Some(Ok((i, OpGreaterThan, i + 1))),
                },

                Some((i, '&')) => return Some(Ok((i, OpBitwiseAnd, i + 1))),
                Some((i, '~')) => match self.chars.peek() {
                    Some((_, '=')) => {
                        self.chars.next();
                        return Some(Ok((i, OpInequality, i + 2)));
                    }
                    _ => return Some(Ok((i, Tilde, i + 1))),
                },
                Some((i, '|')) => return Some(Ok((i, OpBitwiseOr, i + 1))),

                Some((i, '=')) => match self.chars.peek() {
                    Some((_, '=')) => {
                        self.chars.next();
                        return Some(Ok((i, OpEquality, i + 2)));
                    }
                    _ => return Some(Ok((i, EqualsSign, i + 1))),
                },

                Some((i, ';')) => return Some(Ok((i, Semicolon, i + 1))),
                Some((i, ',')) => return Some(Ok((i, Comma, i + 1))),
                Some((i, ':')) => match self.chars.peek() {
                    Some((_, ':')) => {
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
                Some((i, '[')) => match self.chars.peek() {
                    Some((_, '=')) => {
                        let (str_begin, level) = self.get_multiline_string_level(i);
                        match self.chars.peek() {
                            Some(&(si, '[')) => {
                                self.chars.next();

                                let end = self.get_multiline_string_end(level, si);
                                return Some(Ok((
                                    i,
                                    MultilineStringLiteral(
                                        level,
                                        &self.input[str_begin + 1..end - 1],
                                    ),
                                    end,
                                )));
                            }
                            Some((chi, chu)) => {
                                return Some(Err(LexicalError::UnrecognizedSymbol(*chi, *chu)))
                            }
                            None => return Some(Err(LexicalError::UnexpectedEOF)),
                        }
                    }
                    Some((_, '[')) => {
                        self.chars.next();
                        let str_begin = i + 2;
                        let end = self.get_multiline_string_end(0, i + 1);
                        return Some(Ok((
                            i,
                            MultilineStringLiteral(0, &self.input[str_begin..end - 1]),
                            end,
                        )));
                    }
                    _ => return Some(Ok((i, OpenSquareBracket, i + 1))),
                },

                Some((i, '"')) => {
                    let end = self.get_string_end('"', i);
                    return Some(Ok((
                        i,
                        NormalStringLiteral(&self.input[i + 1..end - 1]),
                        end,
                    )));
                }

                Some((i, '\'')) => {
                    let end = self.get_string_end('\'', i);
                    return Some(Ok((i, CharStringLiteral(&self.input[i + 1..end - 1]), end)));
                }

                Some((i, ch)) if ch.is_ascii_digit() => {
                    // TODO: https://www.lua.org/manual/5.1/manual.html#2.1
                    let end = self.get_number_end(i);
                    return Some(Ok((i, Numeral(&self.input[i..end]), end)));
                }

                Some((i, ch)) if ch.is_ascii_alphabetic() || ch == '_' => {
                    let end = self.get_variable_end(i);
                    let variable = &self.input[i..end];

                    match KEYWORDS.get(variable) {
                        Some(w) => return Some(Ok((i, *w, end))),
                        _ => return Some(Ok((i, Variable(&self.input[i..end]), end))),
                    };
                }

                Some((i, ch)) => return Some(Err(LexicalError::UnrecognizedSymbol(i, ch))),
            }
        }
    }
}
