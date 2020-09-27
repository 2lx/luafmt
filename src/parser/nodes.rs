use std::fmt;

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

    Var(Loc, Box<Node>),
    Numeral(Loc, f64),
    RoundBrackets(Loc, Box<Node>),

    Nil(Loc),
    False(Loc),
    True(Loc),
    VarArg(Loc),
    Break(Loc),
    NormalStringLiteral(Loc, String),
    CharStringLiteral(Loc, String),

    TableConstructor(Loc, Box<Node>),
    Fields(Loc, Vec<Node>),
    FieldNamedBracket(Loc, Box<Node>, Box<Node>),
    FieldNamed(Loc, Box<Node>, Box<Node>),
    FieldSequential(Loc, Box<Node>),

    TableIndex(Loc, Box<Node>, Box<Node>),
    TableMember(Loc, Box<Node>, Box<Node>),
    ExpList(Loc, Vec<Node>),
    NameList(Loc, Vec<Node>),
    ParList(Loc, Box<Node>, Box<Node>),
    VarList(Loc, Vec<Node>),
    FnStaticCall(Loc, Box<Node>, Box<Node>),
    FnMethodCall(Loc, Box<Node>, Box<Node>, Box<Node>),
    FunctionDef(Loc, Box<Node>),
    FuncBody(Loc, Box<Node>, Box<Node>),
    FuncName(Loc, Vec<Node>, Box<Node>),
    FuncDecl(Loc, Box<Node>, Box<Node>),

    StatementList(Loc, Vec<Node>),
    StatementEmpty(Loc),
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
    RetStat(Loc, Box<Node>),
    StatsRetStat(Loc, Box<Node>, Box<Node>),

    Empty,
}

fn print_node_vec(
    f: &mut fmt::Formatter,
    elems: &Vec<Node>,
    padding: &str,
    sep: &str,
    ws: &str,
) -> Result<(), core::fmt::Error> {
    if !elems.is_empty() {
        write!(f, "{}", padding)?;
        for elem in &elems[0..elems.len() - 1] {
            write!(f, "{}{}{}", elem, sep, ws)?;
        }
        write!(f, "{}{}", elems.last().unwrap(), padding)?;
    }
    Ok(())
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

            Var(_, n) => write!(f, "{}", n),
            Numeral(_, n) => write!(f, "{}", n),
            RoundBrackets(_, r) => write!(f, "({})", r),

            Nil(_) => write!(f, "nil"),
            False(_) => write!(f, "false"),
            True(_) => write!(f, "true"),
            VarArg(_) => write!(f, "..."),
            Break(_) => write!(f, "break"),
            NormalStringLiteral(_, s) => write!(f, "\"{}\"", s),
            CharStringLiteral(_, s) => write!(f, "'{}'", s),

            TableConstructor(_, r) => write!(f, "{{{}}}", r),
            Fields(_, fields) => print_node_vec(f, fields, " ", ",", " "),
            FieldNamedBracket(_, e1, e2) => write!(f, "[{}] = {}", e1, e2),
            FieldNamed(_, e1, e2) => write!(f, "{} = {}", e1, e2),
            FieldSequential(_, e) => write!(f, "{}", e),

            TableIndex(_, e1, e2) => write!(f, "{}[{}]", e1, e2),
            TableMember(_, e1, s) => write!(f, "{}.{}", e1, s),
            ExpList(_, exps) => print_node_vec(f, exps, "", ",", " "),
            NameList(_, names) => print_node_vec(f, names, "", ",", " "),
            VarList(_, vars) => print_node_vec(f, vars, "", ",", " "),
            StatementList(_, stts) => print_node_vec(f, stts, "", ",", " "),
            StatementEmpty(_) => Ok(()),
            DoEnd(_, n) => write!(f, "do {} end", n),
            VarsExprs(_, n1, n2) => write!(f, "{} = {}", n1, n2),

            FnStaticCall(_, n1, n2) => write!(f, "{}{}", n1, n2),
            FnMethodCall(_, n1, s, n2) => write!(f, "{}:{}{}", n1, s, n2),
            ParList(_, n1, n2) => write!(f, "{}, {}", n1, n2),
            FunctionDef(_, n) => write!(f, "function {}", n),
            FuncBody(_, n1, n2) => write!(f, "({}) {} end", n1, n2),
            FuncName(_, names, n) => {
                print_node_vec(f, names, "", ".", "")?;
                write!(f, "{}", n)
            }
            FuncDecl(_, n1, n2) => write!(f, "function {} {}", n1, n2),
            LocalNamesExprs(_, n1, n2) => write!(f, "local {} = {}", n1, n2),

            Name(_, s) => write!(f, "{}", s),
            Label(_, n) => write!(f, "::{}::", n),
            GoTo(_, n) => write!(f, "goto {}", n),
            While(_, e, n) => write!(f, "while {} do {} end", e, n),
            Repeat(_, n, e) => write!(f, "repeat {} until {}", n, e),
            ForRange(_, n, e, b) => write!(f, "for {} in {} do {} end", n, e, b),
            ForInt(_, n, e1, e2, e3, b) => write!(f, "for {} = {}, {}, {} do {} end", n, e1, e2, e3, b),
            RetStat(_, n) => write!(f, "return {}", n),
            StatsRetStat(_, n1, n2) => write!(f, "{} {}", n1, n2),

            Empty => Ok(()),
        }
    }
}
