use std::fmt;

use crate::config::{Config, ConfiguredWrite};
use crate::{cfg_write, cfg_write_helper};

#[derive(Debug)]
pub struct Loc(pub usize, pub usize);

impl ConfiguredWrite for Loc {
    fn configured_write(&self, f: &mut dyn fmt::Write, cfg: &Config, buf: &str) -> fmt::Result {
        if cfg.keep_comments && self.1 - self.0 > 2 {
            write!(f, "{}", &buf[self.0 .. self.1].trim_matches(' '))?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Node {
    Exponentiation(Loc, Box<Node>, Box<Node>),
    UnaryNot(Loc, Box<Node>),
    UnaryMinus(Loc, Box<Node>),
    UnaryLength(Loc, Box<Node>),
    UnaryBitwiseXor(Loc, Box<Node>),

    Multiplication(Loc, Box<Node>, Box<Node>),
    Division(Loc, Box<Node>, Box<Node>),
    FloorDivision(Loc, Box<Node>, Box<Node>),
    Modulo(Loc, Box<Node>, Box<Node>),

    Addition(Loc, [Loc; 2], Box<Node>, Box<Node>),
    Subtraction(Loc, Box<Node>, Box<Node>),
    Concatenation(Loc, Box<Node>, Box<Node>),
    LeftShift(Loc, Box<Node>, Box<Node>),
    RightShift(Loc, Box<Node>, Box<Node>),

    BitwiseAnd(Loc, Box<Node>, Box<Node>),
    BitwiseXor(Loc, Box<Node>, Box<Node>),
    BitwiseOr(Loc, Box<Node>, Box<Node>),

    Equality(Loc, Box<Node>, Box<Node>),
    Inequality(Loc, Box<Node>, Box<Node>),
    LessThan(Loc, Box<Node>, Box<Node>),
    GreaterThan(Loc, Box<Node>, Box<Node>),
    LessOrEqual(Loc, Box<Node>, Box<Node>),
    GreaterOrEqual(Loc, Box<Node>, Box<Node>),

    LogicalAnd(Loc, Box<Node>, Box<Node>),
    LogicalOr(Loc, Box<Node>, Box<Node>),

    Var(Loc, Box<Node>, Box<Node>),
    RoundBrackets(Loc, Box<Node>),

    Nil(Loc),
    False(Loc),
    True(Loc),
    VarArg(Loc),
    Break(Loc),
    Numeral(Loc, String),
    NormalStringLiteral(Loc, String),
    CharStringLiteral(Loc, String),
    MultilineStringLiteral(Loc, usize, String),

    TableConstructor(Loc, Box<Node>),
    Fields(Loc, Vec<Node>),
    FieldNamedBracket(Loc, Box<Node>, Box<Node>),
    FieldNamed(Loc, Box<Node>, Box<Node>),
    FieldSequential(Loc, Box<Node>),

    TableIndex(Loc, Box<Node>),
    TableMember(Loc, Box<Node>),
    ExpList(Loc, Vec<Node>),
    NameList(Loc, Vec<Node>),
    ParList(Loc, Vec<Node>),
    VarList(Loc, Vec<Node>),
    VarRoundSuffix(Loc, Box<Node>, Box<Node>),
    VarSuffixList(Loc, Vec<Node>),
    FnMethodCall(Loc, Box<Node>, Box<Node>),
    FunctionDef(Loc, Box<Node>),
    FuncBody(Loc, Box<Node>, Box<Node>),
    FuncName(Loc, Vec<Node>, Box<Node>),
    FuncDecl(Loc, Box<Node>, Box<Node>),

    StatementList(Loc, Vec<Node>),
    DoEnd(Loc, Box<Node>),
    VarsExprs(Loc, Box<Node>, Box<Node>),
    Name(Loc, String),
    Label(Loc, Box<Node>),
    GoTo(Loc, Box<Node>),
    While(Loc, Box<Node>, Box<Node>),
    Repeat(Loc, Box<Node>, Box<Node>),
    ForRange(Loc, Box<Node>, Box<Node>, Box<Node>),
    ForInt(Loc, Box<Node>, Box<Node>, Box<Node>, Box<Node>, Box<Node>),
    LocalNamesExprs(Loc, Box<Node>, Box<Node>),
    IfThenElse(Loc, Box<Node>, Box<Node>, Box<Node>, Box<Node>),
    ElseIfThenVec(Loc, Vec<Node>),
    ElseIfThen(Loc, Box<Node>, Box<Node>),

    RetStat(Loc, Box<Node>),
    StatsRetStat(Loc, Box<Node>, Box<Node>),

    Empty(Loc),
}

fn cfg_write_node_vec(
    f: &mut dyn fmt::Write,
    cfg: &Config,
    buf: &str,
    elems: &Vec<Node>,
    padding: &str,
    sep: &str,
    ws: &str,
) -> Result<(), core::fmt::Error> {
    if !elems.is_empty() {
        write!(f, "{}", padding)?;
        for elem in &elems[0..elems.len() - 1] {
            if let Node::Empty(_) = *elem {
                continue;
            }
            cfg_write!(f, cfg, buf, elem)?;
            write!(f, "{}{}", sep, ws)?;
        }
        cfg_write!(f, cfg, buf, (elems.last().unwrap()))?;
        write!(f, "{}", padding)?;
    }
    Ok(())
}

impl ConfiguredWrite for Node {
    fn configured_write(&self, f: &mut dyn fmt::Write, cfg: &Config, buf: &str) -> fmt::Result {
        use Node::*;

        match self {
            Exponentiation(_, l, r) => cfg_write!(f, cfg, buf, l, " ^ ", r),
            UnaryNot(_, r) => cfg_write!(f, cfg, buf, "not ", r),
            UnaryMinus(_, r) => cfg_write!(f, cfg, buf, "-", r),
            UnaryLength(_, r) => cfg_write!(f, cfg, buf, "#", r),
            UnaryBitwiseXor(_, r) => cfg_write!(f, cfg, buf, "~", r),

            Multiplication(_, l, r) => cfg_write!(f, cfg, buf, l, " * ", r),
            Division(_, l, r) => cfg_write!(f, cfg, buf, l, " / ", r),
            FloorDivision(_, l, r) => cfg_write!(f, cfg, buf, l, " // ", r),
            Modulo(_, l, r) => cfg_write!(f, cfg, buf, l, " % ", r),

            Addition(_, locs, l, r) => {
                cfg_write!(f, cfg, buf, l, (locs[0]), " + ", (locs[1]), r)
            },
            Subtraction(_, l, r) => cfg_write!(f, cfg, buf, l, " - ", r),
            Concatenation(_, l, r) => cfg_write!(f, cfg, buf, l, " .. ", r),
            LeftShift(_, l, r) => cfg_write!(f, cfg, buf, l, " << ", r),
            RightShift(_, l, r) => cfg_write!(f, cfg, buf, l, " >> ", r),

            BitwiseAnd(_, l, r) => cfg_write!(f, cfg, buf, l, " & ", r),
            BitwiseXor(_, l, r) => cfg_write!(f, cfg, buf, l, " ~ ", r),
            BitwiseOr(_, l, r) => cfg_write!(f, cfg, buf, l, " | ", r),

            Equality(_, l, r) => cfg_write!(f, cfg, buf, l, " == ", r),
            Inequality(_, l, r) => cfg_write!(f, cfg, buf, l, " ~= ", r),
            LessThan(_, l, r) => cfg_write!(f, cfg, buf, l, " < ", r),
            GreaterThan(_, l, r) => cfg_write!(f, cfg, buf, l, " > ", r),
            LessOrEqual(_, l, r) => cfg_write!(f, cfg, buf, l, " <= ", r),
            GreaterOrEqual(_, l, r) => cfg_write!(f, cfg, buf, l, " >= ", r),

            LogicalAnd(_, l, r) => cfg_write!(f, cfg, buf, l, " and ", r),
            LogicalOr(_, l, r) => cfg_write!(f, cfg, buf, l, " or ", r),

            Var(_, n1, n2) => cfg_write!(f, cfg, buf, n1, n2),
            RoundBrackets(_, r) => cfg_write!(f, cfg, buf, "(", r, ")"),

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

            TableConstructor(_, r) => cfg_write!(f, cfg, buf, "{{", r, "}}"),
            Fields(_, fields) => cfg_write_node_vec(f, cfg, buf, fields, " ", ",", " "),
            FieldNamedBracket(_, e1, e2) => cfg_write!(f, cfg, buf, "[", e1, "] = ", e2),
            FieldNamed(_, e1, e2) => cfg_write!(f, cfg, buf, e1, " = ", e2),
            FieldSequential(_, e) => cfg_write!(f, cfg, buf, e),

            TableIndex(_, e) => cfg_write!(f, cfg, buf, "[", e, "]"),
            TableMember(_, n) => cfg_write!(f, cfg, buf, ".", n),
            ExpList(_, exps) => cfg_write_node_vec(f, cfg, buf, exps, "", ",", " "),
            NameList(_, names) => cfg_write_node_vec(f, cfg, buf, names, "", ",", " "),
            VarList(_, vars) => cfg_write_node_vec(f, cfg, buf, vars, "", ",", " "),
            StatementList(_, stts) => cfg_write_node_vec(f, cfg, buf, stts, "", ";", " "),
            DoEnd(_, n) => cfg_write!(f, cfg, buf, "do ", n, " end"),
            VarsExprs(_, n1, n2) => cfg_write!(f, cfg, buf, n1, " = ", n2),

            VarRoundSuffix(_, n1, n2) => cfg_write!(f, cfg, buf, "(", n1, ")", n2),
            VarSuffixList(_, suffs) => cfg_write_node_vec(f, cfg, buf, suffs, "", "", ""),
            FnMethodCall(_, n1, n2) => cfg_write!(f, cfg, buf, ":", n1, n2),
            ParList(_, pars) => cfg_write_node_vec(f, cfg, buf, pars, "", ",", " "),
            FunctionDef(_, n) => cfg_write!(f, cfg, buf, "function", n),
            FuncBody(_, n1, n2) => match &**n2 {
                Node::StatementList(_, v2) if v2.is_empty() => cfg_write!(f, cfg, buf, "(", n1, ") end"),
                _ => cfg_write!(f, cfg, buf, "(", n1, ") ", n2, " end"),
            },
            FuncName(_, names, n) => {
                cfg_write_node_vec(f, cfg, buf, names, "", ".", "")?;
                match &**n {
                    Node::Empty(_) => Ok(()),
                    _ => cfg_write!(f, cfg, buf, ":", n),
                }
            }
            FuncDecl(_, n1, n2) => cfg_write!(f, cfg, buf, "function ", n1, n2),
            LocalNamesExprs(_, n1, n2) => match &**n2 {
                Node::Empty(_) => cfg_write!(f, cfg, buf, "local ", n1),
                _ => cfg_write!(f, cfg, buf, "local ", n1, " = ", n2),
            },
            IfThenElse(_, e1, b1, n, b2) => match (&**n, &**b2) {
                (Node::ElseIfThenVec(_, v), Node::Empty(_)) if v.is_empty() => {
                    cfg_write!(f, cfg, buf, "if ", e1, " then ", b1, " end")
                }
                (Node::ElseIfThenVec(_, v), _) if v.is_empty() => {
                    cfg_write!(f, cfg, buf, "if ", e1, " then ", b1, " else ", b2, " end")
                }
                (_, Node::Empty(_)) => cfg_write!(f, cfg, buf, "if ", e1, " then ", b1, " ", n, " end"),
                _ => cfg_write!(f, cfg, buf, "if ", e1, " then ", b1, " ", n, " else ", b2, " end"),
            },
            ElseIfThenVec(_, elems) => cfg_write_node_vec(f, cfg, buf, elems, "", "", " "),
            ElseIfThen(_, e, n) => cfg_write!(f, cfg, buf, "elseif ", e, " then ", n),

            Name(_, s) => write!(f, "{}", s),
            Label(_, n) => cfg_write!(f, cfg, buf, "::", n, "::"),
            GoTo(_, n) => cfg_write!(f, cfg, buf, "goto ", n),
            While(_, e, n) => cfg_write!(f, cfg, buf, "while ", e, " do ", n, " end"),
            Repeat(_, n, e) => cfg_write!(f, cfg, buf, "repeat ", n, " until ", e),
            ForRange(_, n, e, b) => cfg_write!(f, cfg, buf, "for ", n, " in ", e, " do ", b, " end"),
            ForInt(_, n, e1, e2, e3, b) => match &**e3 {
                Node::Empty(_) => {
                    cfg_write!(f, cfg, buf, "for ", n, " = ", e1, ", ", e2, " do ", b, " end")
                }
                _ => {
                    cfg_write!(f, cfg, buf, "for ", n, " = ", e1, ", ", e2, ", ", e3, " do ", b, " end")
                }
            },
            RetStat(_, n) => match &**n {
                Node::Empty(_) => write!(f, "return"),
                _ => cfg_write!(f, cfg, buf, "return ", n),
            },
            StatsRetStat(_, n1, n2) => match &**n1 {
                Node::StatementList(_, ref v) if v.is_empty() => cfg_write!(f, cfg, buf, n2),
                _ => cfg_write!(f, cfg, buf, n1, " ", n2),
            },

            Empty(_) => Ok(()),
        }
    }
}
