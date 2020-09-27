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

    Semicolon,
    Comma,
    Colon,
    Label,

    OpenRoundBracket,
    CloseRoundBracket,
    OpenSquareBracket,
    CloseSquareBracket,
    OpenCurlyBracket,
    CloseCurlyBracket,

    EqualsSign,
    Period,

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
        match self {
            Token::OpExponentiation => write!(f, "^"),
            Token::OpLogicalNot => write!(f, "not"),
            Token::OpLength => write!(f, "#"),

            Token::OpMultiplication => write!(f, "*"),
            Token::OpDivision => write!(f, "/"),
            Token::OpFloorDivision => write!(f, "//"),
            Token::OpModulo => write!(f, "%"),

            Token::OpAddition => write!(f, "+"),
            Token::Minus => write!(f, "-"),

            Token::OpConcatenation => write!(f, ".."),

            Token::OpLeftShift => write!(f, "<<"),
            Token::OpRightShift => write!(f, ">>"),

            Token::OpBitwiseAnd => write!(f, "&"),
            Token::Tilde => write!(f, "~"),
            Token::OpBitwiseOr => write!(f, "|"),

            Token::OpEquality => write!(f, "=="),
            Token::OpInequality => write!(f, "~="),
            Token::OpLessThan => write!(f, "<"),
            Token::OpGreaterThan => write!(f, ">"),
            Token::OpLessOrEqual => write!(f, "<="),
            Token::OpGreaterOrEqual => write!(f, ">="),

            Token::OpLogicalAnd => write!(f, "and"),
            Token::OpLogicalOr => write!(f, "or"),

            Token::Semicolon => write!(f, ";"),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::Label => write!(f, "::"),

            Token::OpenRoundBracket => write!(f, "("),
            Token::CloseRoundBracket => write!(f, ")"),
            Token::OpenSquareBracket => write!(f, "["),
            Token::CloseSquareBracket => write!(f, "]"),
            Token::OpenCurlyBracket => write!(f, "{{"),
            Token::CloseCurlyBracket => write!(f, "}}"),

            Token::Variable(s) => write!(f, "\"{}\"", s),
            Token::Numeral(n) => write!(f, "\"{}\"", n),
            Token::NormalStringLiteral(s) => write!(f, "\"{}\"", s),
            Token::CharStringLiteral(s) => write!(f, "'{}'", s),
            Token::EqualsSign => write!(f, "="),
            Token::Period => write!(f, "."),

            Token::Break    => write!(f, "break"),
            Token::Do       => write!(f, "do"),
            Token::Else     => write!(f, "else"),
            Token::ElseIf   => write!(f, "elseif"),
            Token::End      => write!(f, "end"),
            Token::False    => write!(f, "false"),
            Token::For      => write!(f, "for"),
            Token::Function => write!(f, "function"),
            Token::GoTo     => write!(f, "goto"),
            Token::If       => write!(f, "if"),
            Token::In       => write!(f, "in"),
            Token::Local    => write!(f, "local"),
            Token::Nil      => write!(f, "nil"),
            Token::Repeat   => write!(f, "repeat"),
            Token::Return   => write!(f, "return"),
            Token::Then     => write!(f, "then"),
            Token::True     => write!(f, "true"),
            Token::Until    => write!(f, "until"),
            Token::VarArg   => write!(f, "..."),
            Token::While    => write!(f, "while"),
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
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexicalError::UnrecognizedSymbol(i, ch) => {
                write!(f, "lexical error: unrecognized symbol '{}' at {}", ch, i)
            }
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
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Token<'input>, usize), LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                None => return None, // end of file

                Some((_, ' ')) | Some((_, '\n')) | Some((_, '\r')) | Some((_, '\t')) => continue,

                // operators
                Some((i, '^')) => return Some(Ok((i, Token::OpExponentiation, i + 1))),
                Some((i, '#')) => return Some(Ok((i, Token::OpLength, i + 1))),

                Some((i, '*')) => return Some(Ok((i, Token::OpMultiplication, i + 1))),
                Some((i, '%')) => return Some(Ok((i, Token::OpModulo, i + 1))),
                Some((i, '/')) => match self.chars.peek() {
                    Some((_, '/')) => {
                        self.chars.next();
                        return Some(Ok((i, Token::OpFloorDivision, i + 2)));
                    }
                    _ => return Some(Ok((i, Token::OpDivision, i + 1))),
                },

                Some((i, '+')) => return Some(Ok((i, Token::OpAddition, i + 1))),
                Some((i, '-')) => return Some(Ok((i, Token::Minus, i + 1))),

                Some((i, '.')) => match self.chars.peek() {
                    Some((_, '.')) => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some((_, '.')) => {
                                self.chars.next();
                                return Some(Ok((i, Token::VarArg, i + 3)));
                            },
                            _ => return Some(Ok((i, Token::OpConcatenation, i + 2))),
                        }
                    },
                    _ => return Some(Ok((i, Token::Period, i + 1))),
                },

                Some((i, '<')) => match self.chars.peek() {
                    Some((_, '<')) => {
                        self.chars.next();
                        return Some(Ok((i, Token::OpLeftShift, i + 2)));
                    }
                    Some((_, '=')) => {
                        self.chars.next();
                        return Some(Ok((i, Token::OpLessOrEqual, i + 2)));
                    }
                    _ => return Some(Ok((i, Token::OpLessThan, i + 1))),
                },

                Some((i, '>')) => match self.chars.peek() {
                    Some((_, '>')) => {
                        self.chars.next();
                        return Some(Ok((i, Token::OpRightShift, i + 2)));
                    }
                    Some((_, '=')) => {
                        self.chars.next();
                        return Some(Ok((i, Token::OpGreaterOrEqual, i + 2)));
                    }
                    _ => return Some(Ok((i, Token::OpGreaterThan, i + 1))),
                },

                Some((i, '&')) => return Some(Ok((i, Token::OpBitwiseAnd, i + 1))),
                Some((i, '~')) => match self.chars.peek() {
                    Some((_, '=')) => {
                        self.chars.next();
                        return Some(Ok((i, Token::OpInequality, i + 2)));
                    }
                    _ => return Some(Ok((i, Token::Tilde, i + 1))),
                },
                Some((i, '|')) => return Some(Ok((i, Token::OpBitwiseOr, i + 1))),

                Some((i, '=')) => match self.chars.peek() {
                    Some((_, '=')) => {
                        self.chars.next();
                        return Some(Ok((i, Token::OpEquality, i + 2)));
                    }
                    _ => return Some(Ok((i, Token::EqualsSign, i + 1))),
                },

                Some((i, ';')) => return Some(Ok((i, Token::Semicolon, i + 1))),
                Some((i, ',')) => return Some(Ok((i, Token::Comma, i + 1))),
                Some((i, ':')) => match self.chars.peek() {
                    Some((_, ':')) => {
                        self.chars.next();
                        return Some(Ok((i, Token::Label, i + 2)));
                    }
                    _ => return Some(Ok((i, Token::Colon, i + 1))),
                },

                Some((i, '(')) => return Some(Ok((i, Token::OpenRoundBracket, i + 1))),
                Some((i, ')')) => return Some(Ok((i, Token::CloseRoundBracket, i + 1))),
                Some((i, '[')) => return Some(Ok((i, Token::OpenSquareBracket, i + 1))),
                Some((i, ']')) => return Some(Ok((i, Token::CloseSquareBracket, i + 1))),
                Some((i, '{')) => return Some(Ok((i, Token::OpenCurlyBracket, i + 1))),
                Some((i, '}')) => return Some(Ok((i, Token::CloseCurlyBracket, i + 1))),

                Some((i, ch)) if ch == '"' => {
                    let end = self.get_string_end('"', i);
                    return Some(Ok((i, Token::NormalStringLiteral(&self.input[i + 1..end - 1]), end)));
                }

                Some((i, ch)) if ch == '\'' => {
                    let end = self.get_string_end('\'', i);
                    return Some(Ok((i, Token::CharStringLiteral(&self.input[i + 1..end - 1]), end)));
                }

                Some((i, ch)) if ch.is_ascii_digit() => {
                    let end = self.get_number_end(i);
                    return Some(Ok((i, Token::Numeral(&self.input[i..end]), end)));
                }

                Some((i, ch)) if ch.is_ascii_alphabetic() => {
                    let end = self.get_variable_end(i);
                    let variable = &self.input[i..end];

                    match KEYWORDS.get(variable) {
                        Some(w) => return Some(Ok((i, *w, end))),
                        _ => return Some(Ok((i, Token::Variable(&self.input[i..end]), end))),
                    };
                }

                Some((i, ch)) => return Some(Err(LexicalError::UnrecognizedSymbol(i, ch))),
            }
        }
    }
}
