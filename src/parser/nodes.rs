use std::fmt;

use crate::config::{Config, ConfiguredWrite};
use crate::{cfg_write, cfg_write_helper};

#[derive(Debug)]
pub struct Loc(pub usize, pub usize);

#[derive(Debug)]
pub struct Str(pub &'static str);

impl ConfiguredWrite for Str {
    fn configured_write(&self, f: &mut dyn fmt::Write, _cfg: &Config, _buf: &str) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct LocOpt<'a>(pub &'a Loc, pub &'static str);

impl ConfiguredWrite for LocOpt<'_> {
    fn configured_write(&self, f: &mut dyn fmt::Write, cfg: &Config, buf: &str) -> fmt::Result {
        if cfg.remove_comments == Some(true) {
            return write!(f, "{}", self.1)
        }

        if cfg.normalize_ws != Some(true) {
            return write!(f, "{}", &buf[self.0.0..self.0.1])
        }

        let trimmed = &buf[self.0.0..self.0.1].trim_matches(' ');
        if trimmed.len() > 0 {
            let prefix = match trimmed.chars().next().unwrap() {
                '-' => " ",
                _ => "",
            };
            let suffix = match trimmed.chars().last().unwrap() {
                '\n' => "",
                _ => " ",
            };

            write!(f, "{}{}{}", prefix, trimmed, suffix)?;
        } else {
            write!(f, "{}", self.1)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Node {
    BinaryOp(Loc, [Loc; 2], Str, Box<Node>, Box<Node>),
    UnaryOp(Loc, [Loc; 1], Str, Box<Node>),
    UnaryNot(Loc, [Loc; 1], Box<Node>),

    Var(Loc, [Loc; 1], Box<Node>, Box<Node>),
    RoundBrackets(Loc, [Loc; 2], Box<Node>),
    RoundBracketsEmpty(Loc, [Loc; 1]),

    Nil(Loc),
    False(Loc),
    True(Loc),
    VarArg(Loc),
    Break(Loc),
    Numeral(Loc, String),
    NormalStringLiteral(Loc, String),
    CharStringLiteral(Loc, String),
    MultilineStringLiteral(Loc, usize, String),

    TableConstructor(Loc, [Loc; 2], Box<Node>),
    TableConstructorEmpty(Loc, [Loc; 1]),
    Fields(Loc, Vec<(Loc, Node, Loc)>),
    FieldNamedBracket(Loc, [Loc; 4], Box<Node>, Box<Node>),
    FieldNamed(Loc, [Loc; 2], Box<Node>, Box<Node>),
    FieldSequential(Loc, Box<Node>),

    TableIndex(Loc, [Loc; 2], Box<Node>),
    TableMember(Loc, [Loc; 1], Box<Node>),
    ExpList(Loc, Vec<(Loc, Node, Loc)>),
    NameList(Loc, Vec<(Loc, Node, Loc)>),
    ParList(Loc, Vec<(Loc, Node, Loc)>),
    VarList(Loc, Vec<(Loc, Node, Loc)>),
    VarRoundSuffix(Loc, [Loc; 3], Box<Node>, Box<Node>),
    VarSuffixList(Loc, Vec<(Loc, Node)>),
    FnMethodCall(Loc, [Loc; 2], Box<Node>, Box<Node>),
    FunctionDef(Loc, [Loc; 1], Box<Node>),
    FuncBody(Loc, [Loc; 3], Box<Node>),
    FuncBodyB(Loc, [Loc; 4], Box<Node>, Box<Node>),
    FuncName(Loc, Vec<(Loc, Node, Loc)>),
    FuncNameSelf(Loc, [Loc; 2], Vec<(Loc, Node, Loc)>, Box<Node>),
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

    Empty(Loc),
}

fn cfg_write_node_vec_locs_sep(
    f: &mut dyn fmt::Write,
    cfg: &Config,
    buf: &str,
    elems: &Vec<(Loc, Node, Loc)>,
    sep: &str,
    ws: &'static str,
) -> Result<(), core::fmt::Error> {
    if !elems.is_empty() {
        let first = &elems[0];
        cfg_write!(f, cfg, buf, LocOpt(&first.0, ""), first.1, LocOpt(&first.2, ""))?;

        for elem in &elems[1..elems.len()] {
            if let Node::Empty(_) = elem.1 {
                continue;
            }
            write!(f, "{}", sep)?;
            cfg_write!(f, cfg, buf, LocOpt(&elem.0, ws), elem.1, LocOpt(&elem.2, ""))?;
        }
    }
    Ok(())
}

fn cfg_write_node_vec_locs(
    f: &mut dyn fmt::Write,
    cfg: &Config,
    buf: &str,
    elems: &Vec<(Loc, Node)>,
    ws: &'static str,
) -> Result<(), core::fmt::Error> {
    if !elems.is_empty() {
        let first = &elems[0];
        cfg_write!(f, cfg, buf, LocOpt(&first.0, ""), first.1)?;

        for elem in &elems[1..elems.len()] {
            if let Node::Empty(_) = elem.1 {
                continue;
            }
            cfg_write!(f, cfg, buf, LocOpt(&elem.0, ws), elem.1)?;
        }
    }
    Ok(())
}

impl ConfiguredWrite for Node {
    fn configured_write(&self, f: &mut dyn fmt::Write, cfg: &Config, buf: &str) -> fmt::Result {
        use Node::*;

        match self {
            BinaryOp(_, locs, tok, l, r) => {
                cfg_write!(f, cfg, buf, l, LocOpt(&locs[0], " "), tok, LocOpt(&locs[1], " "), r)
            }
            UnaryOp(_, locs, tok, r) => cfg_write!(f, cfg, buf, tok, LocOpt(&locs[0], ""), r),
            UnaryNot(_, locs, r) => cfg_write!(f, cfg, buf, "not", LocOpt(&locs[0], " "), r),

            Var(_, locs, n1, n2) => cfg_write!(f, cfg, buf, n1, LocOpt(&locs[0], ""), n2),
            RoundBrackets(_, locs, r) => cfg_write!(f, cfg, buf, "(", LocOpt(&locs[0], ""), r, LocOpt(&locs[1], ""), ")"),
            RoundBracketsEmpty(_, locs) => cfg_write!(f, cfg, buf, "(", LocOpt(&locs[0], ""), ")"),

            Nil(_) => write!(f, "nil"),
            False(_) => write!(f, "false"),
            True(_) => write!(f, "true"),
            VarArg(_) => write!(f, "..."),
            Break(_) => write!(f, "break"),

            // literals
            Numeral(_, s) => write!(f, "{}", s),
            NormalStringLiteral(_, s) => write!(f, "\"{}\"", s),
            CharStringLiteral(_, s) => write!(f, "'{}'", s),
            MultilineStringLiteral(_, level, s) => {
                let level_str = (0..*level).map(|_| "=").collect::<String>();
                write!(f, "[{}[{}]{}]", level_str, s, level_str)
            }

            TableConstructor(_, locs, r) => {
                cfg_write!(f, cfg, buf, "{{", LocOpt(&locs[0], " "), r, LocOpt(&locs[1], " "), "}}")
            }
            TableConstructorEmpty(_, locs) => cfg_write!(f, cfg, buf, "{{", LocOpt(&locs[0], ""), "}}"),
            Fields(_, fields) => cfg_write_node_vec_locs_sep(f, cfg, buf, fields, ",", " "),
            FieldNamedBracket(_, locs, e1, e2) => {
                cfg_write!(f, cfg, buf, "[", LocOpt(&locs[0], ""), e1, LocOpt(&locs[1], ""), "]", LocOpt(&locs[2], " "),
                           "=", LocOpt(&locs[3], " "), e2)
            }
            FieldNamed(_, locs, e1, e2) => {
                cfg_write!(f, cfg, buf, e1, LocOpt(&locs[0], " "), "=", LocOpt(&locs[1], " "), e2)
            }
            FieldSequential(_, e) => cfg_write!(f, cfg, buf, e),

            TableIndex(_, locs, e) => cfg_write!(f, cfg, buf, "[", LocOpt(&locs[0], ""), e, LocOpt(&locs[1], ""), "]"),
            TableMember(_, locs, n) => cfg_write!(f, cfg, buf, ".", LocOpt(&locs[0], ""), n),
            ExpList(_, exps) => cfg_write_node_vec_locs_sep(f, cfg, buf, exps, ",", " "),
            NameList(_, names) => cfg_write_node_vec_locs_sep(f, cfg, buf, names, ",", " "),
            VarList(_, vars) => cfg_write_node_vec_locs_sep(f, cfg, buf, vars, ",", " "),
            StatementList(_, stts) => cfg_write_node_vec_locs(f, cfg, buf, stts, " "),
            DoEnd(_, locs) => cfg_write!(f, cfg, buf, "do", LocOpt(&locs[0], " "), "end"),
            DoBEnd(_, locs, b) => cfg_write!(f, cfg, buf, "do", LocOpt(&locs[0], " "), b, LocOpt(&locs[1], " "), "end"),
            VarsExprs(_, locs, n1, n2) => cfg_write!(f, cfg, buf, n1, LocOpt(&locs[0], " "), "=", LocOpt(&locs[1], " "), n2),

            VarRoundSuffix(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, "(", LocOpt(&locs[0], ""), n1, LocOpt(&locs[1], ""), ")", LocOpt(&locs[2], ""), n2)
            }
            VarSuffixList(_, suffs) => cfg_write_node_vec_locs(f, cfg, buf, suffs, ""),
            FnMethodCall(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, ":", LocOpt(&locs[0], ""), n1, LocOpt(&locs[1], ""), n2)
            }
            ParList(_, pars) => cfg_write_node_vec_locs_sep(f, cfg, buf, pars, ",", " "),
            FunctionDef(_, locs, n) => cfg_write!(f, cfg, buf, "function", LocOpt(&locs[0], ""), n),
            FuncBody(_, locs, n1) => {
                cfg_write!(f, cfg, buf, "(", LocOpt(&locs[0], ""), n1, LocOpt(&locs[1], ""), ")", LocOpt(&locs[2], " "),
                           "end")
            }
            FuncBodyB(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, "(", LocOpt(&locs[0], ""), n1, LocOpt(&locs[1], ""), ")", LocOpt(&locs[2], " "),
                           n2, LocOpt(&locs[3], " "), "end")
            }
            FuncName(_, names) => cfg_write_node_vec_locs_sep(f, cfg, buf, names, ".", ""),
            FuncNameSelf(_, locs, names, n) => {
                cfg_write_node_vec_locs_sep(f, cfg, buf, names, ".", "")?;
                cfg_write!(f, cfg, buf, LocOpt(&locs[0], ""), ":", LocOpt(&locs[1], ""), n)
            }
            FuncDecl(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, "function", LocOpt(&locs[0], " "), n1, LocOpt(&locs[1], ""), n2)
            }
            LocalFuncDecl(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, "local", LocOpt(&locs[0], " "), "function", LocOpt(&locs[1], " "), n1,
                           LocOpt(&locs[2], ""), n2)
            }

            LocalNames(_, locs, n) => cfg_write!(f, cfg, buf, "local", LocOpt(&locs[0], " "), n),
            LocalNamesExprs(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, "local", LocOpt(&locs[0], " "), n1, LocOpt(&locs[1], " "), "=",
                           LocOpt(&locs[2], " "), n2)
            }

            // if
            IfThen(_, locs, e1) => {
                cfg_write!(f, cfg, buf, "if", LocOpt(&locs[0], " "), e1, LocOpt(&locs[1], " "), "then",
                           LocOpt(&locs[2], " "), "end")
            }
            IfThenB(_, locs, e1, b1) => {
                cfg_write!(f, cfg, buf, "if", LocOpt(&locs[0], " "), e1, LocOpt(&locs[1], " "), "then",
                           LocOpt(&locs[2], " "), b1, LocOpt(&locs[3], " "), "end")
            }
            IfThenElse(_, locs, e1) => {
                cfg_write!(f, cfg, buf, "if", LocOpt(&locs[0], " "), e1, LocOpt(&locs[1], " "), "then",
                           LocOpt(&locs[2], " "), "else", LocOpt(&locs[3], " "), "end")
            }
            IfThenBElse(_, locs, e1, b1) => {
                cfg_write!(f, cfg, buf, "if", LocOpt(&locs[0], " "), e1, LocOpt(&locs[1], " "), "then",
                           LocOpt(&locs[2], " "), b1, LocOpt(&locs[3], " "), "else", LocOpt(&locs[4], " "), "end")
            }
            IfThenElseB(_, locs, e1, b2) => {
                cfg_write!(f, cfg, buf, "if", LocOpt(&locs[0], " "), e1, LocOpt(&locs[1], " "), "then",
                           LocOpt(&locs[2], " "), "else", LocOpt(&locs[3], " "), b2, LocOpt(&locs[4], " "), "end")
            }
            IfThenBElseB(_, locs, e1, b1, b2) => {
                cfg_write!(f, cfg, buf, "if", LocOpt(&locs[0], " "), e1, LocOpt(&locs[1], " "), "then",
                           LocOpt(&locs[2], " "), b1, LocOpt(&locs[3], " "), "else", LocOpt(&locs[4], " "), b2,
                           LocOpt(&locs[5], " "), "end")
            }
            IfThenElseIf(_, locs, e1, n) => {
                cfg_write!(f, cfg, buf, "if", LocOpt(&locs[0], " "), e1, LocOpt(&locs[1], " "), "then",
                           LocOpt(&locs[2], " "), n, LocOpt(&locs[3], " "), "end")
            }
            IfThenBElseIf(_, locs, e1, b1, n) => {
                cfg_write!(f, cfg, buf, "if", LocOpt(&locs[0], " "), e1, LocOpt(&locs[1], " "), "then",
                           LocOpt(&locs[2], " "), b1, LocOpt(&locs[3], " "), n, LocOpt(&locs[4], " "), "end")
            }
            IfThenElseIfElse(_, locs, e1, n) => {
                cfg_write!(f, cfg, buf, "if", LocOpt(&locs[0], " "), e1, LocOpt(&locs[1], " "), "then",
                           LocOpt(&locs[2], " "), n, LocOpt(&locs[3], " "), "else", LocOpt(&locs[4], " "), "end")
            }
            IfThenBElseIfElse(_, locs, e1, b1, n) => {
                cfg_write!(f, cfg, buf, "if", LocOpt(&locs[0], " "), e1, LocOpt(&locs[1], " "), "then",
                           LocOpt(&locs[2], " "), b1, LocOpt(&locs[3], " "), n, LocOpt(&locs[4], " "), "else",
                           LocOpt(&locs[5], " "), "end")
            }
            IfThenElseIfElseB(_, locs, e1, n, b2) => {
                cfg_write!(f, cfg, buf, "if", LocOpt(&locs[0], " "), e1, LocOpt(&locs[1], " "), "then",
                           LocOpt(&locs[2], " "), n, LocOpt(&locs[3], " "), "else", LocOpt(&locs[4], " "), b2,
                           LocOpt(&locs[5], " "), "end")
            }
            IfThenBElseIfElseB(_, locs, e1, b1, n, b2) => {
                cfg_write!(f, cfg, buf, "if", LocOpt(&locs[0], " "), e1, LocOpt(&locs[1], " "), "then",
                           LocOpt(&locs[2], " "), b1, LocOpt(&locs[3], " "), n, LocOpt(&locs[4], " "), "else",
                           LocOpt(&locs[5], " "), b2, LocOpt(&locs[6], " "), "end")
            }
            ElseIfThenVec(_, elems) => cfg_write_node_vec_locs(f, cfg, buf, elems, " "),
            ElseIfThen(_, locs, e) => {
                cfg_write!(f, cfg, buf, "elseif", LocOpt(&locs[0], " "), e, LocOpt(&locs[1], " "), "then")
            }
            ElseIfThenB(_, locs, e, b) => {
                cfg_write!(f, cfg, buf, "elseif", LocOpt(&locs[0], " "), e, LocOpt(&locs[1], " "), "then",
                           LocOpt(&locs[2], " "), b)
            }

            Name(_, s) => write!(f, "{}", s),
            Label(_, locs, n) => cfg_write!(f, cfg, buf, "::", LocOpt(&locs[0], ""), n, LocOpt(&locs[1], ""), "::"),
            GoTo(_, locs, n) => cfg_write!(f, cfg, buf, "goto", LocOpt(&locs[0], " "), n),
            WhileDo(_, locs, e) => cfg_write!(f, cfg, buf, "while", LocOpt(&locs[0], " "), e, LocOpt(&locs[1], " "),
                                              "do", LocOpt(&locs[2], " "), "end"),
            WhileDoB(_, locs, e, n) => cfg_write!(f, cfg, buf, "while", LocOpt(&locs[0], " "), e, LocOpt(&locs[1], " "),
                                                  "do", LocOpt(&locs[2], " "), n, LocOpt(&locs[3], " "), "end"),
            RepeatUntil(_, locs, e) => cfg_write!(f, cfg, buf, "repeat", LocOpt(&locs[0], " "), "until",
                                                  LocOpt(&locs[1], " "), e),
            RepeatBUntil(_, locs, b, e) => cfg_write!(f, cfg, buf, "repeat", LocOpt(&locs[0], " "), b,
                                                      LocOpt(&locs[1], " "), "until", LocOpt(&locs[2], " "), e),

            ForInt(_, locs, n, e1, e2) => {
                cfg_write!(f, cfg, buf, "for", LocOpt(&locs[0], " "), n, LocOpt(&locs[1], " "), "=", LocOpt(&locs[2], " "),
                           e1, LocOpt(&locs[3], ""), ",", LocOpt(&locs[4], " "), e2, LocOpt(&locs[5], " "), "do",
                           LocOpt(&locs[6], " "), "end")
            }
            ForIntB(_, locs, n, e1, e2, b) => {
                cfg_write!(f, cfg, buf, "for", LocOpt(&locs[0], " "), n, LocOpt(&locs[1], " "), "=", LocOpt(&locs[2], " "),
                           e1, LocOpt(&locs[3], ""), ",", LocOpt(&locs[4], " "), e2, LocOpt(&locs[5], " "), "do",
                           LocOpt(&locs[6], " "), b, LocOpt(&locs[7], " "), "end")
            }
            ForIntStep(_, locs, n, e1, e2, e3) => {
                cfg_write!(f, cfg, buf, "for", LocOpt(&locs[0], " "), n, LocOpt(&locs[1], " "), "=", LocOpt(&locs[2], " "),
                           e1, LocOpt(&locs[3], ""), ",", LocOpt(&locs[4], " "), e2, LocOpt(&locs[5], ""), ",",
                           LocOpt(&locs[6], " "), e3, LocOpt(&locs[7], " "), "do", LocOpt(&locs[8], " "), "end")
            },
            ForIntStepB(_, locs, n, e1, e2, e3, b) => {
                cfg_write!(f, cfg, buf, "for", LocOpt(&locs[0], " "), n, LocOpt(&locs[1], " "), "=", LocOpt(&locs[2], " "),
                           e1, LocOpt(&locs[3], ""), ",", LocOpt(&locs[4], " "), e2, LocOpt(&locs[5], ""), ",",
                           LocOpt(&locs[6], " "), e3, LocOpt(&locs[7], " "), "do", LocOpt(&locs[8], " "), b,
                           LocOpt(&locs[9], " "), "end")
            },
            ForRange(_, locs, n, e) => {
                cfg_write!(f, cfg, buf, "for", LocOpt(&locs[0], " "), n, LocOpt(&locs[1], " "), "in",
                           LocOpt(&locs[2], " "), e, LocOpt(&locs[3], " "), "do", LocOpt(&locs[4], " "), "end")
            }
            ForRangeB(_, locs, n, e, b) => {
                cfg_write!(f, cfg, buf, "for", LocOpt(&locs[0], " "), n, LocOpt(&locs[1], " "), "in",
                           LocOpt(&locs[2], " "), e, LocOpt(&locs[3], " "), "do", LocOpt(&locs[4], " "), b,
                           LocOpt(&locs[5], " "), "end")
            }

            RetStatNone(_) => write!(f, "return"),
            RetStatExpr(_, locs, n) => cfg_write!(f, cfg, buf, "return", LocOpt(&locs[0], " "), n),
            RetStatNoneComma(_, locs) => cfg_write!(f, cfg, buf, "return", LocOpt(&locs[0], ""), ";"),
            RetStatExprComma(_, locs, n) => {
                cfg_write!(f, cfg, buf, "return", LocOpt(&locs[0], " "), n, LocOpt(&locs[1], ""), ";")
            }
            StatsRetStat(_, locs, n1, n2) => cfg_write!(f, cfg, buf, n1, LocOpt(&locs[0], " "), n2),
            Chunk(locl, n, locr) => cfg_write!(f, cfg, buf, LocOpt(&locl, ""), n, LocOpt(&locr, "")),

            Empty(_) => Ok(()),
        }
    }
}
