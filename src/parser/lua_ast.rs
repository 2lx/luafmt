use std::fmt;

use super::common::*;
use crate::config::*;
use crate::{cfg_write, cfg_write_helper};
use crate::formatting::loc_hint::*;
use crate::formatting::list;
use crate::formatting::indent_hint::*;

#[derive(Debug)]
pub enum Node {
    BinaryOp(Loc, [Loc; 2], Str<'static>, Box<Node>, Box<Node>),
    UnaryOp(Loc, [Loc; 1], Str<'static>, Box<Node>),
    UnaryNot(Loc, [Loc; 1], Box<Node>),

    Var(Loc, [Loc; 1], Box<Node>, Box<Node>),
    RoundBrackets(Loc, [Loc; 2], Box<Node>),
    ArgsRoundBrackets(Loc, [Loc; 2], Box<Node>),
    ArgsRoundBracketsEmpty(Loc, [Loc; 1]),

    Nil(Loc),
    False(Loc),
    True(Loc),
    VarArg(Loc),
    Break(Loc),
    Numeral(Loc, String),
    NormalStringLiteral(Loc, String),
    CharStringLiteral(Loc, String),
    MultiLineStringLiteral(Loc, usize, String),

    TableConstructor(Loc, [Loc; 2], Box<Node>),
    TableConstructorEmpty(Loc, [Loc; 1]),
    Fields(Loc, Vec<(Loc, Node, Loc, String)>),
    FieldNamedBracket(Loc, [Loc; 4], Box<Node>, Box<Node>),
    FieldNamed(Loc, [Loc; 2], Box<Node>, Box<Node>),
    FieldSequential(Loc, Box<Node>),

    TableIndex(Loc, [Loc; 2], Box<Node>),
    TableMember(Loc, [Loc; 1], Box<Node>),
    ExpList(Loc, Vec<(Loc, Node, Loc, String)>),
    NameList(Loc, Vec<(Loc, Node, Loc, String)>),
    ParList(Loc, Vec<(Loc, Node, Loc, String)>),
    VarList(Loc, Vec<(Loc, Node, Loc, String)>),
    VarRoundSuffix(Loc, [Loc; 3], Box<Node>, Box<Node>),
    VarSuffixList(Loc, Vec<(Loc, Node)>),
    FnMethodCall(Loc, [Loc; 2], Box<Node>, Box<Node>),
    FunctionDef(Loc, [Loc; 1], Box<Node>),
    FuncBody(Loc, [Loc; 2]),
    FuncBodyB(Loc, [Loc; 3], Box<Node>),
    FuncPBody(Loc, [Loc; 3], Box<Node>),
    FuncPBodyB(Loc, [Loc; 4], Box<Node>, Box<Node>),
    FuncName(Loc, Vec<(Loc, Node, Loc, String)>),
    FuncNameSelf(Loc, [Loc; 2], Vec<(Loc, Node, Loc, String)>, Box<Node>),
    FuncDecl(Loc, [Loc; 2], Box<Node>, Box<Node>),
    LocalFuncDecl(Loc, [Loc; 3], Box<Node>, Box<Node>),

    StatementList(Loc, Vec<(Loc, Node)>),
    DoEnd(Loc, [Loc; 1]),
    DoBEnd(Loc, [Loc; 2], Box<Node>),
    VarsExprs(Loc, [Loc; 2], Box<Node>, Box<Node>),
    Name(Loc, String),
    Label(Loc, [Loc; 2], Box<Node>),
    GoTo(Loc, [Loc; 1], Box<Node>),
    WhileDo(Loc, [Loc; 3], Box<Node>),
    WhileDoB(Loc, [Loc; 4], Box<Node>, Box<Node>),
    RepeatUntil(Loc, [Loc; 2], Box<Node>),
    RepeatBUntil(Loc, [Loc; 3], Box<Node>, Box<Node>),
    ForInt(Loc, [Loc; 7], Box<Node>, Box<Node>, Box<Node>),
    ForIntB(Loc, [Loc; 8], Box<Node>, Box<Node>, Box<Node>, Box<Node>),
    ForIntStep(Loc, [Loc; 9], Box<Node>, Box<Node>, Box<Node>, Box<Node>),
    ForIntStepB(Loc, [Loc; 10], Box<Node>, Box<Node>, Box<Node>, Box<Node>, Box<Node>),
    ForRange(Loc, [Loc; 5], Box<Node>, Box<Node>),
    ForRangeB(Loc, [Loc; 6], Box<Node>, Box<Node>, Box<Node>),

    LocalNames(Loc, [Loc; 1], Box<Node>),
    LocalNamesExprs(Loc, [Loc; 3], Box<Node>, Box<Node>),

    IfThen(Loc, [Loc; 3], Box<Node>),
    IfThenB(Loc, [Loc; 4], Box<Node>, Box<Node>),
    IfThenElse(Loc, [Loc; 4], Box<Node>),
    IfThenBElse(Loc, [Loc; 5], Box<Node>, Box<Node>),
    IfThenElseB(Loc, [Loc; 5], Box<Node>, Box<Node>),
    IfThenBElseB(Loc, [Loc; 6], Box<Node>, Box<Node>, Box<Node>),
    IfThenElseIf(Loc, [Loc; 4], Box<Node>, Box<Node>),
    IfThenBElseIf(Loc, [Loc; 5], Box<Node>, Box<Node>, Box<Node>),
    IfThenElseIfElse(Loc, [Loc; 5], Box<Node>, Box<Node>),
    IfThenBElseIfElse(Loc, [Loc; 6], Box<Node>, Box<Node>, Box<Node>),
    IfThenElseIfElseB(Loc, [Loc; 6], Box<Node>, Box<Node>, Box<Node>),
    IfThenBElseIfElseB(Loc, [Loc; 7], Box<Node>, Box<Node>, Box<Node>, Box<Node>),
    ElseIfThenVec(Loc, Vec<(Loc, Node)>),
    ElseIfThen(Loc, [Loc; 2], Box<Node>),
    ElseIfThenB(Loc, [Loc; 3], Box<Node>, Box<Node>),

    RetStatNone(Loc),
    RetStatExpr(Loc, [Loc; 1], Box<Node>),
    RetStatNoneComma(Loc, [Loc; 1]),
    RetStatExprComma(Loc, [Loc; 2], Box<Node>),
    StatsRetStat(Loc, [Loc; 1], Box<Node>, Box<Node>),
    Chunk(Loc, Box<Node>, Loc),
    SheBangChunk(Loc, Box<Node>, Loc, Box<Node>, Loc),
    Semicolon(Loc),
    SheBang(Loc, String),
}

impl<'a> list::NoSepListItem<'a> for Node {
    fn list_item_prefix_hint(&self, _: &'a Config) -> &'a str {
        use Node::*;
        match self {
            Semicolon(..) | ArgsRoundBrackets(..) | ArgsRoundBracketsEmpty(..) | TableIndex(..)
                | TableMember(..) | FnMethodCall(..) | TableConstructor(..) | TableConstructorEmpty(..) => "",
            _ => " ",
        }
    }

    fn list_item_suffix_hint(&self, _: &'a Config) -> &'a str {
        // use Node::*;
        match self {
            _ => "",
        }
    }

    fn need_indent(&self, cfg: &'a Config) -> bool {
        use Node::*;
        match self {
            VarsExprs(..) | RepeatUntil(..) | RepeatBUntil(..) | LocalNames(..) | LocalNamesExprs(..) | Break(..)
                | GoTo(..) | DoEnd(..) | DoBEnd(..) | WhileDo(..) | WhileDoB(..) | IfThen(..) | IfThenB(..)
                | IfThenElse(..) | IfThenBElse(..) | IfThenElseB(..) | IfThenBElseB(..) | IfThenElseIf(..)
                | IfThenBElseIf(..) | IfThenElseIfElse(..) | IfThenBElseIfElse(..) | IfThenElseIfElseB(..)
                | IfThenBElseIfElseB(..) | ForInt(..) | ForIntB(..) | ForIntStep(..) | ForIntStepB(..)
                | ForRange(..) | ForRangeB(..) | FuncDecl(..) | LocalFuncDecl(..) | Var(..) | VarRoundSuffix(..)
                | RoundBrackets(..) => {
                    if cfg.indentation_string.is_some() {
                        cfg.indent_every_statement.unwrap_or(false)
                    } else {
                        false
                    }
                }
            _ => false,
        }
    }
}

impl list::SepListOfItems<Node> for Node {
    fn items(&self) -> Option<&Vec::<(Loc, Node, Loc, String)>> {
        use Node::*;
        match self {
            Fields(_, items) | ExpList(_, items) | NameList(_, items) | VarList(_, items) | ParList(_, items)
                | FuncName(_, items) | FuncNameSelf(_, _, items, _) => Some(items),
            _ => None,
        }
    }

    fn element_prefix_hint(&self) -> &str {
        use Node::*;
        match self {
            Fields(..) | ExpList(..) | NameList(..) | VarList(..) | ParList(..) => " ",
            FuncName(..) | FuncNameSelf(..) => "",
            _ => "",
        }
    }

    fn separator(&self, cfg: &Config) -> Option<String> {
        use Node::*;
        match self {
            Fields(..) => cfg.field_separator.clone(),
            ExpList(..) | NameList(..) | VarList(..) | ParList(..) => Some(",".to_string()),
            FuncName(..) | FuncNameSelf(..) => Some(".".to_string()),
            _ => None,
        }
    }

    fn trailing_separator(&self, cfg: &Config) -> Option<bool> {
        use Node::*;
        match self {
            Fields(..) => cfg.write_trailing_field_separator.clone(),
            ExpList(..) | NameList(..) | VarList(..) | ParList(..) | FuncName(..) | FuncNameSelf(..) => Some(false),
            _ => None,
        }
    }
}


impl ConfiguredWrite for Node {
    fn configured_write(&self, f: &mut dyn fmt::Write, cfg: &Config, buf: &str, state: &mut State) -> fmt::Result {
        use Node::*;

        #[allow(non_snake_case)]
        let Hint = CommentLocHint;
        let cfg_write_list = list::cfg_write_list_items::<Node, CommentLocHint>;
        let cfg_write_sep_list = list::cfg_write_sep_list::<Node, CommentLocHint>;

        match self {
            BinaryOp(_, locs, tok, l, r) => {
                cfg_write!(f, cfg, buf, state, l, Hint(&locs[0], " "), tok, Hint(&locs[1], " "), r)
            }
            UnaryOp(_, locs, tok, r) => cfg_write!(f, cfg, buf, state, tok, Hint(&locs[0], ""), r),
            UnaryNot(_, locs, r) => cfg_write!(f, cfg, buf, state, "not", Hint(&locs[0], " "), r),

            Var(_, locs, n1, n2) => cfg_write!(f, cfg, buf, state, n1, Hint(&locs[0], ""), n2),
            RoundBrackets(_, locs, r) => {
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), r, Hint(&locs[1], ""), ")")
            }
            ArgsRoundBrackets(_, locs, r) => {
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), r, Hint(&locs[1], ""), ")")
            }
            ArgsRoundBracketsEmpty(_, locs) => cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), ")"),

            Nil(_) => write!(f, "nil"),
            False(_) => write!(f, "false"),
            True(_) => write!(f, "true"),
            VarArg(_) => write!(f, "..."),
            Break(_) => write!(f, "break"),

            // literals
            Numeral(_, s) => write!(f, "{}", s),
            NormalStringLiteral(_, s) => write!(f, "\"{}\"", s),
            CharStringLiteral(_, s) => write!(f, "'{}'", s),
            MultiLineStringLiteral(_, level, s) => {
                let level_str = (0..*level).map(|_| "=").collect::<String>();
                write!(f, "[{}[{}]{}]", level_str, s, level_str)
            }

            TableConstructor(_, locs, r) => {
                cfg_write!(f, cfg, buf, state, "{{", Hint(&locs[0], " "), r, Hint(&locs[1], " "), "}}")
            }
            TableConstructorEmpty(_, locs) => cfg_write!(f, cfg, buf, state, "{{", Hint(&locs[0], ""), "}}"),
            Fields(..) => {
                cfg_write_sep_list(f, cfg, buf, state, self)
            }
            FieldNamedBracket(_, locs, e1, e2) => {
                cfg_write!(f, cfg, buf, state, "[", Hint(&locs[0], ""), e1, Hint(&locs[1], ""), "]", Hint(&locs[2], " "),
                           "=", Hint(&locs[3], " "), e2)
            }
            FieldNamed(_, locs, e1, e2) => {
                cfg_write!(f, cfg, buf, state, e1, Hint(&locs[0], " "), "=", Hint(&locs[1], " "), e2)
            }
            FieldSequential(_, e) => cfg_write!(f, cfg, buf, state, e),

            TableIndex(_, locs, e) => cfg_write!(f, cfg, buf, state, "[", Hint(&locs[0], ""), e, Hint(&locs[1], ""), "]"),
            TableMember(_, locs, n) => cfg_write!(f, cfg, buf, state, ".", Hint(&locs[0], ""), n),
            ExpList(..) => cfg_write_sep_list(f, cfg, buf, state, self),
            NameList(..) => cfg_write_sep_list(f, cfg, buf, state, self),
            VarList(..) => cfg_write_sep_list(f, cfg, buf, state, self),
            StatementList(_, stts) => cfg_write_list(f, cfg, buf, state, stts),
            DoEnd(_, locs) => cfg_write!(f, cfg, buf, state, "do", Hint(&locs[0], " "), "end"),
            DoBEnd(_, locs, b) => {
                match (&cfg.indentation_string, &cfg.do_end_format) {
                    (Some(_), Some(1)) => {
                        cfg_write!(f, cfg, buf, state, "do", IndentHint(1), NewLineDecorator(Hint(&locs[0], "")), b,
                                   IndentHint(-1), NewLineDecorator(Hint(&locs[1], "")), "end")
                    }
                    _ => cfg_write!(f, cfg, buf, state, "do", Hint(&locs[0], " "), b, Hint(&locs[1], " "), "end"),
                }
            }
            VarsExprs(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, state, n1, Hint(&locs[0], " "), "=", Hint(&locs[1], " "), n2)
            }

            VarRoundSuffix(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), n1, Hint(&locs[1], ""), ")", Hint(&locs[2], ""),
                           n2)
            }
            VarSuffixList(_, suffs) => cfg_write_list(f, cfg, buf, state, suffs),
            FnMethodCall(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, state, ":", Hint(&locs[0], ""), n1, Hint(&locs[1], ""), n2)
            }
            ParList(..) => cfg_write_sep_list(f, cfg, buf, state, self),
            FunctionDef(_, locs, n) => cfg_write!(f, cfg, buf, state, "function", Hint(&locs[0], ""), n),
            FuncBody(_, locs) => cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), ")", Hint(&locs[1], " "), "end"),
            FuncBodyB(_, locs, n2) => {
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), ")", Hint(&locs[1], " "), n2, Hint(&locs[2], " "),
                           "end")
            }
            FuncPBody(_, locs, n1) => {
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), n1, Hint(&locs[1], ""), ")", Hint(&locs[2], " "),
                           "end")
            }
            FuncPBodyB(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), n1, Hint(&locs[1], ""), ")", Hint(&locs[2], " "),
                           n2, Hint(&locs[3], " "), "end")
            }
            FuncName(..) => cfg_write_sep_list(f, cfg, buf, state, self),
            FuncNameSelf(_, locs, _, n) => {
                cfg_write_sep_list(f, cfg, buf, state, self)?;
                cfg_write!(f, cfg, buf, state, Hint(&locs[0], ""), ":", Hint(&locs[1], ""), n)
            }
            FuncDecl(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, state, "function", Hint(&locs[0], " "), n1, Hint(&locs[1], ""), n2)
            }
            LocalFuncDecl(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, state, "local", Hint(&locs[0], " "), "function", Hint(&locs[1], " "), n1,
                           Hint(&locs[2], ""), n2)
            }

            LocalNames(_, locs, n) => cfg_write!(f, cfg, buf, state, "local", Hint(&locs[0], " "), n),
            LocalNamesExprs(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, state, "local", Hint(&locs[0], " "), n1, Hint(&locs[1], " "), "=",
                           Hint(&locs[2], " "), n2)
            }

            // if
            IfThen(_, locs, e1) => {
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           Hint(&locs[2], " "), "end")
            }
            IfThenB(_, locs, e1, b1) => {
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           Hint(&locs[2], " "), b1, Hint(&locs[3], " "), "end")
            }
            IfThenElse(_, locs, e1) => {
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           Hint(&locs[2], " "), "else", Hint(&locs[3], " "), "end")
            }
            IfThenBElse(_, locs, e1, b1) => {
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           Hint(&locs[2], " "), b1, Hint(&locs[3], " "), "else", Hint(&locs[4], " "), "end")
            }
            IfThenElseB(_, locs, e1, b2) => {
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           Hint(&locs[2], " "), "else", Hint(&locs[3], " "), b2, Hint(&locs[4], " "), "end")
            }
            IfThenBElseB(_, locs, e1, b1, b2) => {
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           Hint(&locs[2], " "), b1, Hint(&locs[3], " "), "else", Hint(&locs[4], " "),
                           b2, Hint(&locs[5], " "), "end")
            }
            IfThenElseIf(_, locs, e1, n) => {
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           Hint(&locs[2], " "), n, Hint(&locs[3], " "), "end")
            }
            IfThenBElseIf(_, locs, e1, b1, n) => {
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           Hint(&locs[2], " "), b1, Hint(&locs[3], " "), n, Hint(&locs[4], " "), "end")
            }
            IfThenElseIfElse(_, locs, e1, n) => {
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           Hint(&locs[2], " "), n, Hint(&locs[3], " "), "else", Hint(&locs[4], " "), "end")
            }
            IfThenBElseIfElse(_, locs, e1, b1, n) => {
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           Hint(&locs[2], " "), b1, Hint(&locs[3], " "), n, Hint(&locs[4], " "), "else",
                           Hint(&locs[5], " "), "end")
            }
            IfThenElseIfElseB(_, locs, e1, n, b2) => {
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           Hint(&locs[2], " "), n, Hint(&locs[3], " "), "else", Hint(&locs[4], " "), b2,
                           Hint(&locs[5], " "), "end")
            }
            IfThenBElseIfElseB(_, locs, e1, b1, n, b2) => {
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           Hint(&locs[2], " "), b1, Hint(&locs[3], " "), n, Hint(&locs[4], " "), "else",
                           Hint(&locs[5], " "), b2, Hint(&locs[6], " "), "end")
            }
            ElseIfThenVec(_, elems) => cfg_write_list(f, cfg, buf, state, elems),
            ElseIfThen(_, locs, e) => {
                cfg_write!(f, cfg, buf, state, "elseif", Hint(&locs[0], " "), e, Hint(&locs[1], " "), "then")
            }
            ElseIfThenB(_, locs, e, b) => {
                cfg_write!(f, cfg, buf, state, "elseif", Hint(&locs[0], " "), e, Hint(&locs[1], " "), "then",
                           Hint(&locs[2], " "), b)
            }

            Name(_, s) => write!(f, "{}", s),
            Label(_, locs, n) => cfg_write!(f, cfg, buf, state, "::", Hint(&locs[0], ""), n, Hint(&locs[1], ""), "::"),
            GoTo(_, locs, n) => cfg_write!(f, cfg, buf, state, "goto", Hint(&locs[0], " "), n),
            WhileDo(_, locs, e) => {
                cfg_write!(f, cfg, buf, state, "while", Hint(&locs[0], " "), e, Hint(&locs[1], " "), "do",
                           Hint(&locs[2], " "), "end")
            }
            WhileDoB(_, locs, e, n) => {
                cfg_write!(f, cfg, buf, state, "while", Hint(&locs[0], " "), e, Hint(&locs[1], " "), "do",
                           Hint(&locs[2], " "), n, Hint(&locs[3], " "), "end")
            }
            RepeatUntil(_, locs, e) => {
                cfg_write!(f, cfg, buf, state, "repeat", Hint(&locs[0], " "), "until", Hint(&locs[1], " "), e)
            }
            RepeatBUntil(_, locs, b, e) => {
                cfg_write!(f, cfg, buf, state, "repeat", Hint(&locs[0], " "), b, Hint(&locs[1], " "), "until",
                           Hint(&locs[2], " "), e)
            }

            ForInt(_, locs, n, e1, e2) => {
                cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "=",
                           Hint(&locs[2], " "), e1, Hint(&locs[3], ""), ",", Hint(&locs[4], " "), e2, Hint(&locs[5], " "),
                           "do", Hint(&locs[6], " "), "end")
            }
            ForIntB(_, locs, n, e1, e2, b) => {
                match (&cfg.indentation_string, &cfg.for_format) {
                    (Some(_), Some(1)) => {
                        cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "=",
                                   Hint(&locs[2], " "), e1, Hint(&locs[3], ""), ",", Hint(&locs[4], " "), e2,
                                   Hint(&locs[5], " "), "do", IndentHint(1), NewLineDecorator(Hint(&locs[6], "")), b,
                                   IndentHint(-1), NewLineDecorator(Hint(&locs[7], "")), "end")
                    }
                    _ => cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "=",
                                    Hint(&locs[2], " "), e1, Hint(&locs[3], ""), ",", Hint(&locs[4], " "), e2,
                                    Hint(&locs[5], " "), "do", Hint(&locs[6], " "), b, Hint(&locs[7], " "), "end"),
                }
            }
            ForIntStep(_, locs, n, e1, e2, e3) => {
                cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "=",
                           Hint(&locs[2], " "), e1, Hint(&locs[3], ""), ",", Hint(&locs[4], " "), e2, Hint(&locs[5], ""),
                           ",", Hint(&locs[6], " "), e3, Hint(&locs[7], " "), "do", Hint(&locs[8], " "), "end")
            },
            ForIntStepB(_, locs, n, e1, e2, e3, b) => {
                match (&cfg.indentation_string, &cfg.for_format) {
                    (Some(_), Some(1)) => {
                        cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "=",
                                   Hint(&locs[2], " "), e1, Hint(&locs[3], ""), ",", Hint(&locs[4], " "), e2,
                                   Hint(&locs[5], ""), ",", Hint(&locs[6], " "), e3, Hint(&locs[7], " "), "do",
                                   IndentHint(1), NewLineDecorator(Hint(&locs[8], "")), b, IndentHint(-1),
                                   NewLineDecorator(Hint(&locs[9], "")), "end")
                    }
                    _ => cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "=",
                                    Hint(&locs[2], " "), e1, Hint(&locs[3], ""), ",", Hint(&locs[4], " "), e2,
                                    Hint(&locs[5], ""), ",", Hint(&locs[6], " "), e3, Hint(&locs[7], " "), "do",
                                    Hint(&locs[8], " "), b, Hint(&locs[9], " "), "end")
                }
            },
            ForRange(_, locs, n, e) => {
                cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "in",
                           Hint(&locs[2], " "), e, Hint(&locs[3], " "), "do", Hint(&locs[4], " "), "end")
            }
            ForRangeB(_, locs, n, e, b) => {
                match (&cfg.indentation_string, &cfg.for_format) {
                    (Some(_), Some(1)) => {
                        cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "in",
                                   Hint(&locs[2], " "), e, Hint(&locs[3], " "), "do", IndentHint(1),
                                   NewLineDecorator(Hint(&locs[4], "")), b, IndentHint(-1),
                                   NewLineDecorator(Hint(&locs[5], "")), "end")
                    }
                    _ => cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "in",
                                    Hint(&locs[2], " "), e, Hint(&locs[3], " "), "do", Hint(&locs[4], " "), b,
                                    Hint(&locs[5], " "), "end")
                }
            }

            RetStatNone(_) => write!(f, "return"),
            RetStatExpr(_, locs, n) => cfg_write!(f, cfg, buf, state, "return", Hint(&locs[0], " "), n),
            RetStatNoneComma(_, locs) => cfg_write!(f, cfg, buf, state, "return", Hint(&locs[0], ""), ";"),
            RetStatExprComma(_, locs, n) => {
                cfg_write!(f, cfg, buf, state, "return", Hint(&locs[0], " "), n, Hint(&locs[1], ""), ";")
            }
            StatsRetStat(_, locs, n1, n2) => cfg_write!(f, cfg, buf, state, n1, Hint(&locs[0], " "), n2),
            Chunk(locl, n, locr) => cfg_write!(f, cfg, buf, state, Hint(&locl, ""), n, Hint(&locr, "")),
            SheBangChunk(locl, n, locm, b, locr) => {
                cfg_write!(f, cfg, buf, state, Hint(&locl, ""), n, Hint(&locm, ""), b, Hint(&locr, ""))
            }

            Semicolon(_) => write!(f, ";"),
            SheBang(_, s) => write!(f, "{}\n", s),
        }
    }
}
