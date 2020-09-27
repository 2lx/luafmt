use super::lexer::Token;
use std::fmt;

#[derive(Debug)]
pub struct Loc(pub usize, pub usize);

#[derive(Debug)]
pub struct Statements(pub Vec<Statement>);

impl fmt::Display for Statements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        let Statements(stts) = self;
        if !stts.is_empty() {
            for stt in &stts[0..stts.len() - 1] {
                fmt::write(f, format_args!("{} ", stt))?
            }
            fmt::write(f, format_args!("{}", stts.last().unwrap()))?
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Statement {
    NodeTree(Node),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Statement::*;
        match self {
            NodeTree(exprs) => write!(f, "{}{}", exprs, Token::Semicolon),
        }
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

    Var(Loc, std::string::String),
    Numeral(Loc, f64),
    RoundBrackets(Loc, Box<Node>),

    Nil(Loc),
    False(Loc),
    True(Loc),
    VarArg(Loc),

    TableConstructor(Loc, Box<Node>),
    Fields(Loc, Vec<Node>),
    FieldNamedBracket(Loc, Box<Node>, Box<Node>),
    FieldNamed(Loc, Box<Node>, Box<Node>),
    FieldSequential(Loc, Box<Node>),

    TableIndex(Loc, Box<Node>, Box<Node>),
    TableMember(Loc, Box<Node>, std::string::String),
    ExpList(Loc, Vec<Node>),
    FnStaticCall(Loc, Box<Node>, Box<Node>),
    FnMethodCall(Loc, Box<Node>, std::string::String, Box<Node>),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Node::*;
        match self {
            Exponentiation(_, l, r) => write!(f, "{} ^ {}", l, r),
            UnaryNot(_, r) => write!(f, "not {}", r),
            UnaryMinus(_, r) => write!(f, "-{}", r),
            UnaryLength(_, r) => write!(f, "#{}", r),
            UnaryBitwiseXor(_, r) => write!(f, "~{}", r),

            Multiplication(_, l, r) => write!(f, "{} * {}", l, r),
            Division(_, l, r) => write!(f, "{} / {}", l, r),
            FloorDivision(_, l, r) => write!(f, "{} // {}", l, r),
            Modulo(_, l, r) => write!(f, "{} % {}", l, r),

            Addition(_, l, r) => write!(f, "{} + {}", l, r),
            Subtraction(_, l, r) => write!(f, "{} - {}", l, r),
            Concatenation(_, l, r) => write!(f, "{} .. {}", l, r),
            LeftShift(_, l, r) => write!(f, "{} << {}", l, r),
            RightShift(_, l, r) => write!(f, "{} >> {}", l, r),

            BitwiseAnd(_, l, r) => write!(f, "{} & {}", l, r),
            BitwiseXor(_, l, r) => write!(f, "{} ~ {}", l, r),
            BitwiseOr(_, l, r) => write!(f, "{} | {}", l, r),

            Equality(_, l, r) => write!(f, "{} == {}", l, r),
            Inequality(_, l, r) => write!(f, "{} ~= {}", l, r),
            LessThan(_, l, r) => write!(f, "{} < {}", l, r),
            GreaterThan(_, l, r) => write!(f, "{} > {}", l, r),
            LessOrEqual(_, l, r) => write!(f, "{} <= {}", l, r),
            GreaterOrEqual(_, l, r) => write!(f, "{} >= {}", l, r),

            LogicalAnd(_, l, r) => write!(f, "{} and {}", l, r),
            LogicalOr(_, l, r) => write!(f, "{} or {}", l, r),

            Var(_, s) => write!(f, "{}", s),
            Numeral(_, n) => write!(f, "{}", n),
            RoundBrackets(_, r) => write!(f, "({})", r),

            Nil(_) => write!(f, "nil"),
            False(_) => write!(f, "false"),
            True(_) => write!(f, "true"),
            VarArg(_) => write!(f, "..."),

            TableConstructor(_, r) => write!(f, "{{{}}}", r),
            Fields(_, fields) => {
                if !fields.is_empty() {
                    write!(f, " ")?;
                    for field in &fields[0..fields.len() - 1] {
                        write!(f, "{}, ", field)?;
                    }
                    write!(f, "{} ", fields.last().unwrap())?;
                }
                Ok(())
            },
            FieldNamedBracket(_, e1, e2) => write!(f, "[{}] = {}", e1, e2),
            FieldNamed(_, e1, e2) => write!(f, "{} = {}", e1, e2),
            FieldSequential(_, e) => write!(f, "{}", e),

            TableIndex(_, e1, e2) => write!(f, "{}[{}]", e1, e2),
            TableMember(_, e1, s) => write!(f, "{}.{}", e1, s),
            ExpList(_, exps) => {
                if !exps.is_empty() {
                    for exp in &exps[0..exps.len() - 1] {
                        write!(f, "{}, ", exp)?;
                    }
                    write!(f, "{}", exps.last().unwrap())?;
                }
                Ok(())
            },
            FnStaticCall(_, n1, n2) => write!(f, "{}{}", n1, n2),
            FnMethodCall(_, n1, s, n2) => write!(f, "{}:{}{}", n1, s, n2),
        }
    }
}
