use std::fmt;

use crate::config::{Config, ConfiguredWrite};
use crate::{cfg_write, cfg_write_helper};
use crate::format::loc_hint::LocHint;
use crate::format::util::*;
use super::basics::*;

#[derive(Debug)]
pub enum Node {
    BinaryOp(Loc, [Loc; 2], Str, Box<Node>, Box<Node>),
    UnaryOp(Loc, [Loc; 1], Str, Box<Node>),
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
    Semicolon(Loc),
}

impl PrefixHintInNoSepList for Node {
    fn prefix_hint_in_no_sep_list(&self, _: &Config) -> &str {
        use Node::*;

        match self {
            Semicolon(_) | ArgsRoundBrackets(_, _, _) | ArgsRoundBracketsEmpty(_, _) | TableIndex(_, _, _)
                | TableMember(_, _, _) | FnMethodCall(_, _, _, _) | TableConstructor(_, _, _)
                | TableConstructorEmpty(_, _) => "",
            _ => " ",
        }
    }
}

impl ConfiguredWrite for Node {
    fn configured_write(&self, f: &mut dyn fmt::Write, cfg: &Config, buf: &str) -> fmt::Result {
        use Node::*;

        match self {
            BinaryOp(_, locs, tok, l, r) => {
                cfg_write!(f, cfg, buf, l, LocHint(&locs[0], " "), tok, LocHint(&locs[1], " "), r)
            }
            UnaryOp(_, locs, tok, r) => cfg_write!(f, cfg, buf, tok, LocHint(&locs[0], ""), r),
            UnaryNot(_, locs, r) => cfg_write!(f, cfg, buf, "not", LocHint(&locs[0], " "), r),

            Var(_, locs, n1, n2) => cfg_write!(f, cfg, buf, n1, LocHint(&locs[0], ""), n2),
            RoundBrackets(_, locs, r) => cfg_write!(f, cfg, buf, "(", LocHint(&locs[0], ""), r, LocHint(&locs[1], ""), ")"),
            ArgsRoundBrackets(_, locs, r) => cfg_write!(f, cfg, buf, "(", LocHint(&locs[0], ""), r, LocHint(&locs[1], ""), ")"),
            ArgsRoundBracketsEmpty(_, locs) => cfg_write!(f, cfg, buf, "(", LocHint(&locs[0], ""), ")"),

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
                cfg_write!(f, cfg, buf, "{{", LocHint(&locs[0], " "), r, LocHint(&locs[1], " "), "}}")
            }
            TableConstructorEmpty(_, locs) => cfg_write!(f, cfg, buf, "{{", LocHint(&locs[0], ""), "}}"),
            Fields(_, fields) => {
                cfg_write_sep_vector(f, cfg, buf, fields, " ", cfg.field_separator, cfg.trailing_field_separator)
            }
            FieldNamedBracket(_, locs, e1, e2) => {
                cfg_write!(f, cfg, buf, "[", LocHint(&locs[0], ""), e1, LocHint(&locs[1], ""), "]", LocHint(&locs[2], " "),
                           "=", LocHint(&locs[3], " "), e2)
            }
            FieldNamed(_, locs, e1, e2) => {
                cfg_write!(f, cfg, buf, e1, LocHint(&locs[0], " "), "=", LocHint(&locs[1], " "), e2)
            }
            FieldSequential(_, e) => cfg_write!(f, cfg, buf, e),

            TableIndex(_, locs, e) => cfg_write!(f, cfg, buf, "[", LocHint(&locs[0], ""), e, LocHint(&locs[1], ""), "]"),
            TableMember(_, locs, n) => cfg_write!(f, cfg, buf, ".", LocHint(&locs[0], ""), n),
            ExpList(_, exps) => cfg_write_sep_vector(f, cfg, buf, exps, " ", Some(","), Some(false)),
            NameList(_, names) => cfg_write_sep_vector(f, cfg, buf, names, " ", Some(","), Some(false)),
            VarList(_, vars) => cfg_write_sep_vector(f, cfg, buf, vars, " ", Some(","), Some(false)),
            StatementList(_, stts) => cfg_write_vector(f, cfg, buf, stts),
            DoEnd(_, locs) => cfg_write!(f, cfg, buf, "do", LocHint(&locs[0], " "), "end"),
            DoBEnd(_, locs, b) => cfg_write!(f, cfg, buf, "do", LocHint(&locs[0], " "), b, LocHint(&locs[1], " "), "end"),
            VarsExprs(_, locs, n1, n2) => cfg_write!(f, cfg, buf, n1, LocHint(&locs[0], " "), "=", LocHint(&locs[1], " "), n2),

            VarRoundSuffix(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, "(", LocHint(&locs[0], ""), n1, LocHint(&locs[1], ""), ")", LocHint(&locs[2], ""), n2)
            }
            VarSuffixList(_, suffs) => cfg_write_vector(f, cfg, buf, suffs),
            FnMethodCall(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, ":", LocHint(&locs[0], ""), n1, LocHint(&locs[1], ""), n2)
            }
            ParList(_, pars) => cfg_write_sep_vector(f, cfg, buf, pars, " ", Some(","), Some(false)),
            FunctionDef(_, locs, n) => cfg_write!(f, cfg, buf, "function", LocHint(&locs[0], ""), n),
            FuncBody(_, locs) => {
                cfg_write!(f, cfg, buf, "(", LocHint(&locs[0], ""), ")", LocHint(&locs[1], " "), "end")
            }
            FuncBodyB(_, locs, n2) => {
                cfg_write!(f, cfg, buf, "(", LocHint(&locs[0], ""), ")", LocHint(&locs[1], " "), n2, LocHint(&locs[2], " "),
                           "end")
            }
            FuncPBody(_, locs, n1) => {
                cfg_write!(f, cfg, buf, "(", LocHint(&locs[0], ""), n1, LocHint(&locs[1], ""), ")", LocHint(&locs[2], " "),
                           "end")
            }
            FuncPBodyB(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, "(", LocHint(&locs[0], ""), n1, LocHint(&locs[1], ""), ")", LocHint(&locs[2], " "),
                           n2, LocHint(&locs[3], " "), "end")
            }
            FuncName(_, names) => cfg_write_sep_vector(f, cfg, buf, names, "", Some("."), Some(false)),
            FuncNameSelf(_, locs, names, n) => {
                cfg_write_sep_vector(f, cfg, buf, names, "", Some("."), Some(false))?;
                cfg_write!(f, cfg, buf, LocHint(&locs[0], ""), ":", LocHint(&locs[1], ""), n)
            }
            FuncDecl(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, "function", LocHint(&locs[0], " "), n1, LocHint(&locs[1], ""), n2)
            }
            LocalFuncDecl(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, "local", LocHint(&locs[0], " "), "function", LocHint(&locs[1], " "), n1,
                           LocHint(&locs[2], ""), n2)
            }

            LocalNames(_, locs, n) => cfg_write!(f, cfg, buf, "local", LocHint(&locs[0], " "), n),
            LocalNamesExprs(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, "local", LocHint(&locs[0], " "), n1, LocHint(&locs[1], " "), "=",
                           LocHint(&locs[2], " "), n2)
            }

            // if
            IfThen(_, locs, e1) => {
                cfg_write!(f, cfg, buf, "if", LocHint(&locs[0], " "), e1, LocHint(&locs[1], " "), "then",
                           LocHint(&locs[2], " "), "end")
            }
            IfThenB(_, locs, e1, b1) => {
                cfg_write!(f, cfg, buf, "if", LocHint(&locs[0], " "), e1, LocHint(&locs[1], " "), "then",
                           LocHint(&locs[2], " "), b1, LocHint(&locs[3], " "), "end")
            }
            IfThenElse(_, locs, e1) => {
                cfg_write!(f, cfg, buf, "if", LocHint(&locs[0], " "), e1, LocHint(&locs[1], " "), "then",
                           LocHint(&locs[2], " "), "else", LocHint(&locs[3], " "), "end")
            }
            IfThenBElse(_, locs, e1, b1) => {
                cfg_write!(f, cfg, buf, "if", LocHint(&locs[0], " "), e1, LocHint(&locs[1], " "), "then",
                           LocHint(&locs[2], " "), b1, LocHint(&locs[3], " "), "else", LocHint(&locs[4], " "), "end")
            }
            IfThenElseB(_, locs, e1, b2) => {
                cfg_write!(f, cfg, buf, "if", LocHint(&locs[0], " "), e1, LocHint(&locs[1], " "), "then",
                           LocHint(&locs[2], " "), "else", LocHint(&locs[3], " "), b2, LocHint(&locs[4], " "), "end")
            }
            IfThenBElseB(_, locs, e1, b1, b2) => {
                cfg_write!(f, cfg, buf, "if", LocHint(&locs[0], " "), e1, LocHint(&locs[1], " "), "then",
                           LocHint(&locs[2], " "), b1, LocHint(&locs[3], " "), "else", LocHint(&locs[4], " "), b2,
                           LocHint(&locs[5], " "), "end")
            }
            IfThenElseIf(_, locs, e1, n) => {
                cfg_write!(f, cfg, buf, "if", LocHint(&locs[0], " "), e1, LocHint(&locs[1], " "), "then",
                           LocHint(&locs[2], " "), n, LocHint(&locs[3], " "), "end")
            }
            IfThenBElseIf(_, locs, e1, b1, n) => {
                cfg_write!(f, cfg, buf, "if", LocHint(&locs[0], " "), e1, LocHint(&locs[1], " "), "then",
                           LocHint(&locs[2], " "), b1, LocHint(&locs[3], " "), n, LocHint(&locs[4], " "), "end")
            }
            IfThenElseIfElse(_, locs, e1, n) => {
                cfg_write!(f, cfg, buf, "if", LocHint(&locs[0], " "), e1, LocHint(&locs[1], " "), "then",
                           LocHint(&locs[2], " "), n, LocHint(&locs[3], " "), "else", LocHint(&locs[4], " "), "end")
            }
            IfThenBElseIfElse(_, locs, e1, b1, n) => {
                cfg_write!(f, cfg, buf, "if", LocHint(&locs[0], " "), e1, LocHint(&locs[1], " "), "then",
                           LocHint(&locs[2], " "), b1, LocHint(&locs[3], " "), n, LocHint(&locs[4], " "), "else",
                           LocHint(&locs[5], " "), "end")
            }
            IfThenElseIfElseB(_, locs, e1, n, b2) => {
                cfg_write!(f, cfg, buf, "if", LocHint(&locs[0], " "), e1, LocHint(&locs[1], " "), "then",
                           LocHint(&locs[2], " "), n, LocHint(&locs[3], " "), "else", LocHint(&locs[4], " "), b2,
                           LocHint(&locs[5], " "), "end")
            }
            IfThenBElseIfElseB(_, locs, e1, b1, n, b2) => {
                cfg_write!(f, cfg, buf, "if", LocHint(&locs[0], " "), e1, LocHint(&locs[1], " "), "then",
                           LocHint(&locs[2], " "), b1, LocHint(&locs[3], " "), n, LocHint(&locs[4], " "), "else",
                           LocHint(&locs[5], " "), b2, LocHint(&locs[6], " "), "end")
            }
            ElseIfThenVec(_, elems) => cfg_write_vector(f, cfg, buf, elems),
            ElseIfThen(_, locs, e) => {
                cfg_write!(f, cfg, buf, "elseif", LocHint(&locs[0], " "), e, LocHint(&locs[1], " "), "then")
            }
            ElseIfThenB(_, locs, e, b) => {
                cfg_write!(f, cfg, buf, "elseif", LocHint(&locs[0], " "), e, LocHint(&locs[1], " "), "then",
                           LocHint(&locs[2], " "), b)
            }

            Name(_, s) => write!(f, "{}", s),
            Label(_, locs, n) => cfg_write!(f, cfg, buf, "::", LocHint(&locs[0], ""), n, LocHint(&locs[1], ""), "::"),
            GoTo(_, locs, n) => cfg_write!(f, cfg, buf, "goto", LocHint(&locs[0], " "), n),
            WhileDo(_, locs, e) => cfg_write!(f, cfg, buf, "while", LocHint(&locs[0], " "), e, LocHint(&locs[1], " "),
                                              "do", LocHint(&locs[2], " "), "end"),
            WhileDoB(_, locs, e, n) => cfg_write!(f, cfg, buf, "while", LocHint(&locs[0], " "), e, LocHint(&locs[1], " "),
                                                  "do", LocHint(&locs[2], " "), n, LocHint(&locs[3], " "), "end"),
            RepeatUntil(_, locs, e) => cfg_write!(f, cfg, buf, "repeat", LocHint(&locs[0], " "), "until",
                                                  LocHint(&locs[1], " "), e),
            RepeatBUntil(_, locs, b, e) => cfg_write!(f, cfg, buf, "repeat", LocHint(&locs[0], " "), b,
                                                      LocHint(&locs[1], " "), "until", LocHint(&locs[2], " "), e),

            ForInt(_, locs, n, e1, e2) => {
                cfg_write!(f, cfg, buf, "for", LocHint(&locs[0], " "), n, LocHint(&locs[1], " "), "=", LocHint(&locs[2], " "),
                           e1, LocHint(&locs[3], ""), ",", LocHint(&locs[4], " "), e2, LocHint(&locs[5], " "), "do",
                           LocHint(&locs[6], " "), "end")
            }
            ForIntB(_, locs, n, e1, e2, b) => {
                cfg_write!(f, cfg, buf, "for", LocHint(&locs[0], " "), n, LocHint(&locs[1], " "), "=", LocHint(&locs[2], " "),
                           e1, LocHint(&locs[3], ""), ",", LocHint(&locs[4], " "), e2, LocHint(&locs[5], " "), "do",
                           LocHint(&locs[6], " "), b, LocHint(&locs[7], " "), "end")
            }
            ForIntStep(_, locs, n, e1, e2, e3) => {
                cfg_write!(f, cfg, buf, "for", LocHint(&locs[0], " "), n, LocHint(&locs[1], " "), "=", LocHint(&locs[2], " "),
                           e1, LocHint(&locs[3], ""), ",", LocHint(&locs[4], " "), e2, LocHint(&locs[5], ""), ",",
                           LocHint(&locs[6], " "), e3, LocHint(&locs[7], " "), "do", LocHint(&locs[8], " "), "end")
            },
            ForIntStepB(_, locs, n, e1, e2, e3, b) => {
                cfg_write!(f, cfg, buf, "for", LocHint(&locs[0], " "), n, LocHint(&locs[1], " "), "=", LocHint(&locs[2], " "),
                           e1, LocHint(&locs[3], ""), ",", LocHint(&locs[4], " "), e2, LocHint(&locs[5], ""), ",",
                           LocHint(&locs[6], " "), e3, LocHint(&locs[7], " "), "do", LocHint(&locs[8], " "), b,
                           LocHint(&locs[9], " "), "end")
            },
            ForRange(_, locs, n, e) => {
                cfg_write!(f, cfg, buf, "for", LocHint(&locs[0], " "), n, LocHint(&locs[1], " "), "in",
                           LocHint(&locs[2], " "), e, LocHint(&locs[3], " "), "do", LocHint(&locs[4], " "), "end")
            }
            ForRangeB(_, locs, n, e, b) => {
                cfg_write!(f, cfg, buf, "for", LocHint(&locs[0], " "), n, LocHint(&locs[1], " "), "in",
                           LocHint(&locs[2], " "), e, LocHint(&locs[3], " "), "do", LocHint(&locs[4], " "), b,
                           LocHint(&locs[5], " "), "end")
            }

            RetStatNone(_) => write!(f, "return"),
            RetStatExpr(_, locs, n) => cfg_write!(f, cfg, buf, "return", LocHint(&locs[0], " "), n),
            RetStatNoneComma(_, locs) => cfg_write!(f, cfg, buf, "return", LocHint(&locs[0], ""), ";"),
            RetStatExprComma(_, locs, n) => {
                cfg_write!(f, cfg, buf, "return", LocHint(&locs[0], " "), n, LocHint(&locs[1], ""), ";")
            }
            StatsRetStat(_, locs, n1, n2) => cfg_write!(f, cfg, buf, n1, LocHint(&locs[0], " "), n2),
            Chunk(locl, n, locr) => cfg_write!(f, cfg, buf, LocHint(&locl, ""), n, LocHint(&locr, "")),

            Semicolon(_) => write!(f, ";"),
        }
    }
}