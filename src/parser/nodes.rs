use std::fmt;

use crate::config::{Config, ConfiguredWrite};
use crate::{cfg_write, cfg_write_helper};

#[derive(Debug)]
pub struct Loc(pub usize, pub usize);

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

    Addition(Loc, Box<Node>, Box<Node>),
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

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let config = Config { indent_width: 0 };
        cfg_write!(f, &config, self)
    }
}

fn cfg_write_node_vec(
    f: &mut dyn fmt::Write,
    cfg: &Config,
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
            cfg_write!(f, cfg, elem)?;
            write!(f, "{}{}", sep, ws)?;
        }
        cfg_write!(f, cfg, elems.last().unwrap())?;
        write!(f, "{}", padding)?;
    }
    Ok(())
}

impl ConfiguredWrite for Node {
    fn configured_write(&self, f: &mut dyn fmt::Write, cfg: &Config) -> fmt::Result {
        use Node::*;

        match self {
            Exponentiation(_, l, r) => cfg_write!(f, cfg, l, " ^ ", r),
            UnaryNot(_, r) => cfg_write!(f, cfg, "not ", r),
            UnaryMinus(_, r) => cfg_write!(f, cfg, "-", r),
            UnaryLength(_, r) => cfg_write!(f, cfg, "#", r),
            UnaryBitwiseXor(_, r) => cfg_write!(f, cfg, "~", r),

            Multiplication(_, l, r) => cfg_write!(f, cfg, l, " * ", r),
            Division(_, l, r) => cfg_write!(f, cfg, l, " / ", r),
            FloorDivision(_, l, r) => cfg_write!(f, cfg, l, " // ", r),
            Modulo(_, l, r) => cfg_write!(f, cfg, l, " % ", r),

            Addition(_, l, r) => cfg_write!(f, cfg, l, " + ", r),
            Subtraction(_, l, r) => cfg_write!(f, cfg, l, " - ", r),
            Concatenation(_, l, r) => cfg_write!(f, cfg, l, " .. ", r),
            LeftShift(_, l, r) => cfg_write!(f, cfg, l, " << ", r),
            RightShift(_, l, r) => cfg_write!(f, cfg, l, " >> ", r),

            BitwiseAnd(_, l, r) => cfg_write!(f, cfg, l, " & ", r),
            BitwiseXor(_, l, r) => cfg_write!(f, cfg, l, " ~ ", r),
            BitwiseOr(_, l, r) => cfg_write!(f, cfg, l, " | ", r),

            Equality(_, l, r) => cfg_write!(f, cfg, l, " == ", r),
            Inequality(_, l, r) => cfg_write!(f, cfg, l, " ~= ", r),
            LessThan(_, l, r) => cfg_write!(f, cfg, l, " < ", r),
            GreaterThan(_, l, r) => cfg_write!(f, cfg, l, " > ", r),
            LessOrEqual(_, l, r) => cfg_write!(f, cfg, l, " <= ", r),
            GreaterOrEqual(_, l, r) => cfg_write!(f, cfg, l, " >= ", r),

            LogicalAnd(_, l, r) => cfg_write!(f, cfg, l, " and ", r),
            LogicalOr(_, l, r) => cfg_write!(f, cfg, l, " or ", r),

            Var(_, n1, n2) => cfg_write!(f, cfg, n1, n2),
            RoundBrackets(_, r) => cfg_write!(f, cfg, "(", r, ")"),

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

            TableConstructor(_, r) => cfg_write!(f, cfg, "{{", r, "}}"),
            Fields(_, fields) => cfg_write_node_vec(f, cfg, fields, " ", ",", " "),
            FieldNamedBracket(_, e1, e2) => cfg_write!(f, cfg, "[", e1, "] = ", e2),
            FieldNamed(_, e1, e2) => cfg_write!(f, cfg, e1, " = ", e2),
            FieldSequential(_, e) => cfg_write!(f, cfg, e),

            TableIndex(_, e) => cfg_write!(f, cfg, "[", e, "]"),
            TableMember(_, n) => cfg_write!(f, cfg, ".", n),
            ExpList(_, exps) => cfg_write_node_vec(f, cfg, exps, "", ",", " "),
            NameList(_, names) => cfg_write_node_vec(f, cfg, names, "", ",", " "),
            VarList(_, vars) => cfg_write_node_vec(f, cfg, vars, "", ",", " "),
            StatementList(_, stts) => cfg_write_node_vec(f, cfg, stts, "", ";", " "),
            DoEnd(_, n) => cfg_write!(f, cfg, "do ", n, " end"),
            VarsExprs(_, n1, n2) => cfg_write!(f, cfg, n1, " = ", n2),

            VarRoundSuffix(_, n1, n2) => cfg_write!(f, cfg, "(", n1, ")", n2),
            VarSuffixList(_, suffs) => cfg_write_node_vec(f, cfg, suffs, "", "", ""),
            FnMethodCall(_, n1, n2) => cfg_write!(f, cfg, ":", n1, n2),
            ParList(_, pars) => cfg_write_node_vec(f, cfg, pars, "", ",", " "),
            FunctionDef(_, n) => cfg_write!(f, cfg, "function", n),
            FuncBody(_, n1, n2) => match &**n2 {
                Node::StatementList(_, v2) if v2.is_empty() => cfg_write!(f, cfg, "(", n1, ") end"),
                _ => cfg_write!(f, cfg, "(", n1, ") ", n2, " end"),
            },
            FuncName(_, names, n) => {
                cfg_write_node_vec(f, cfg, names, "", ".", "")?;
                match &**n {
                    Node::Empty(_) => Ok(()),
                    _ => cfg_write!(f, cfg, ":", n),
                }
            }
            FuncDecl(_, n1, n2) => cfg_write!(f, cfg, "function ", n1, n2),
            LocalNamesExprs(_, n1, n2) => match &**n2 {
                Node::Empty(_) => cfg_write!(f, cfg, "local ", n1),
                _ => cfg_write!(f, cfg, "local ", n1, " = ", n2),
            },
            IfThenElse(_, e1, b1, n, b2) => match (&**n, &**b2) {
                (Node::ElseIfThenVec(_, v), Node::Empty(_)) if v.is_empty() => {
                    cfg_write!(f, cfg, "if ", e1, " then ", b1, " end")
                }
                (Node::ElseIfThenVec(_, v), _) if v.is_empty() => {
                    cfg_write!(f, cfg, "if ", e1, " then ", b1, " else ", b2, " end")
                }
                (_, Node::Empty(_)) => cfg_write!(f, cfg, "if ", e1, " then ", b1, " ", n, " end"),
                _ => cfg_write!(f, cfg, "if ", e1, " then ", b1, " ", n, " else ", b2, " end"),
            },
            ElseIfThenVec(_, elems) => cfg_write_node_vec(f, cfg, elems, "", "", " "),
            ElseIfThen(_, e, n) => cfg_write!(f, cfg, "elseif ", e, " then ", n),

            Name(_, s) => write!(f, "{}", s),
            Label(_, n) => cfg_write!(f, cfg, "::", n, "::"),
            GoTo(_, n) => cfg_write!(f, cfg, "goto ", n),
            While(_, e, n) => cfg_write!(f, cfg, "while ", e, " do ", n, " end"),
            Repeat(_, n, e) => cfg_write!(f, cfg, "repeat ", n, " until ", e),
            ForRange(_, n, e, b) => cfg_write!(f, cfg, "for ", n, " in ", e, " do ", b, " end"),
            ForInt(_, n, e1, e2, e3, b) => match &**e3 {
                Node::Empty(_) => {
                    cfg_write!(f, cfg, "for ", n, " = ", e1, ", ", e2, " do ", b, " end")
                }
                _ => {
                    cfg_write!(f, cfg, "for ", n, " = ", e1, ", ", e2, ", ", e3, " do ", b, " end")
                }
            },
            RetStat(_, n) => match &**n {
                Node::Empty(_) => write!(f, "return"),
                _ => cfg_write!(f, cfg, "return ", n),
            },
            StatsRetStat(_, n1, n2) => match &**n1 {
                Node::StatementList(_, ref v) if v.is_empty() => cfg_write!(f, cfg, n2),
                _ => cfg_write!(f, cfg, n1, " ", n2),
            },

            Empty(_) => Ok(()),
        }
    }
}
