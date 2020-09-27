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

    StatementList(Loc, Vec<Node>),
    StatementEmpty(Loc),
    StatementDoEnd(Loc, Box<Node>),
    StatementVarExp(Loc, Box<Node>, Box<Node>),
    Name(Loc, std::string::String),
    Empty,
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
            NameList(_, names) => {
                if !names.is_empty() {
                    for name in &names[0..names.len() - 1] {
                        write!(f, "{}, ", name)?;
                    }
                    write!(f, "{}", names.last().unwrap())?;
                }
                Ok(())
            },
            VarList(_, vars) => {
                if !vars.is_empty() {
                    for var in &vars[0..vars.len() - 1] {
                        write!(f, "{}, ", var)?;
                    }
                    write!(f, "{}", vars.last().unwrap())?;
                }
                Ok(())
            },
            StatementList(_, stts) => {
                if !stts.is_empty() {
                    for stt in &stts[0..stts.len() - 1] {
                        write!(f, "{}; ", stt)?;
                    }
                    write!(f, "{};", stts.last().unwrap())?;
                }
                Ok(())
            },
            StatementEmpty(_) => Ok(()),
            StatementDoEnd(_, n) => write!(f, "do {} end", n),
            StatementVarExp(_, n1, n2) => write!(f, "{} = {}", n1, n2),

            FnStaticCall(_, n1, n2) => write!(f, "{}{}", n1, n2),
            FnMethodCall(_, n1, s, n2) => write!(f, "{}:{}{}", n1, s, n2),
            ParList(_, n1, n2) => write!(f, "{}, {}", n1, n2),
            FunctionDef(_, n) =>  write!(f, "function {}", n),
            FuncBody(_, n1, n2) => write!(f, "({}) {} end", n1, n2),

            Empty => Ok(()),
            Name(_, s) => write!(f, "{}", s),
        }
    }
}
