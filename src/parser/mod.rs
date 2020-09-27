#[allow(clippy::all)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod syntax;
mod lexer;
pub mod nodes;

use lalrpop_util::ParseError;

pub fn parse(
    src: &str,
) -> Result<nodes::Node, ParseError<usize, lexer::Token, lexer::LexicalError>> {
    let lexer = lexer::Lexer::new(src);
    syntax::ChunkParser::new().parse(src, lexer)
}

#[test]
fn test_expressions_ok() {
    let result = parse("3 + 22 * 11 + 65");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "3 + 22 * 11 + 65;");

    let result = parse("(3 + 22) * 11 + 65;");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "(3 + 22) * 11 + 65;");

    let result = parse("3 + 22 * (11 + 65)");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "3 + 22 * (11 + 65);");

    let result = parse("(3 + 22) * (11 + 65)");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "(3 + 22) * (11 + 65);");

    let result = parse("-(3 + - 22) * (-11 + 65)");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "-(3 + -22) * (-11 + 65);");

    let result = parse("-3+-22*-11^3*1+ 65");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "-3 + -22 * -11 ^ 3 * 1 + 65;"
    );

    let result = parse("-3+-22--11^3*1+ 65");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "-3 + -22 - -11 ^ 3 * 1 + 65;"
    );

    let result = parse("- PI* (10)");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "-PI * (10);");
}

#[test]
fn test_errors() {
    let result = parse("3 + 22 * ? + 65");
    assert!(result.is_err());
    match result.unwrap_err() {
        #[allow(unused_variables)]
        ParseError::User { error } => (),
        _ => assert!(false, "wrong error type"),
    };

    let result = parse("1---2");
    assert!(result.is_ok());

    let result = parse("1++2");
    assert!(result.is_err());
    match result.unwrap_err() {
        #[allow(unused_variables)]
        ParseError::UnrecognizedToken {
            token: (l, token, r),
            expected,
        } => (),
        _ => assert!(false, "wrong error type"),
    };

    let result = parse("1 2");
    assert!(result.is_err());
    match result.unwrap_err() {
        #[allow(unused_variables)]
        ParseError::UnrecognizedToken {
            token: (l, token, r),
            expected,
        } => (),
        _ => assert!(false, "wrong error type"),
    };

    let result = parse("1+");
    assert!(result.is_err());
    match result.unwrap_err() {
        #[allow(unused_variables)]
        ParseError::UnrecognizedEOF { location, expected } => (),
        _ => assert!(false, "wrong error type"),
    };
}

#[test]
fn test_table() {
    let result = parse("{ }");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "{};");

    let result = parse("{ 1 }");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "{ 1 };");

    let result = parse("{ 1, 2, 3; 4, 5; 6; 7 }");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "{ 1, 2, 3, 4, 5, 6, 7 };");

    let result = parse("{ a = 1, 2, b = 4, 4 }");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "{ a = 1, 2, b = 4, 4 };");

    let result = parse("{ a = { b = true }, [b] = { a = true } }");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "{ a = { b = true }, [b] = { a = true } };");

    let result = parse("tbl[4 + b]");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "tbl[4 + b];");

    let result = parse("tbl.field");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "tbl.field;");
}

#[test]
fn test_functions() {
    let result = parse("fn_name(a1, a2)");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "fn_name(a1, a2);");

    let result = parse("fn_name{a1, a2}");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "fn_name{ a1, a2 };");

    let result = parse("fn_name:method{a1, a2}");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "fn_name:method{ a1, a2 };");
}

#[test]
fn test_statements() {
    let result = parse(";;;;;");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "");

    let result = parse(";;;;");
    assert!(result.is_ok());

    let result = parse("32;;;;;");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "32;");

    let result = parse("32;;;;12;;;;;45");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "32; 12; 45;");

    let result = parse("3+2;12-3;-42;");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "3 + 2; 12 - 3; -42;");

    let result = parse("3+2;12-3;-42;");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "3 + 2; 12 - 3; -42;");

    // let result = parse("  a=3  ");
    // assert!(result.is_ok());
    // assert_eq!(&format!("{}", result.unwrap()), "a = 3;");

    // let result = parse(";;; a= 3 ;;;vars;;quit");
    // assert!(result.is_ok());
    // assert_eq!(&format!("{}", result.unwrap()), "a = 3; vars; quit;");
    //
    // let result = parse(";a=3;;;; \n ;b=5;;;a+b;;");
    // assert!(result.is_ok());
    // assert_eq!(&format!("{}", result.unwrap()), "a = 3; b = 5; a + b;");
}
