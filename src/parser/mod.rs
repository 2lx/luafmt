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
fn test_errors() {
    let result = parse("a = 3 + 22 * ? + 65");
    assert!(result.is_err());
    match result.unwrap_err() {
        #[allow(unused_variables)]
        ParseError::User { error } => (),
        _ => assert!(false, "wrong error type"),
    };

    let result = parse("local a = 1--2");
    assert!(result.is_ok());

    let result = parse("local a = 1---2");
    assert!(result.is_err());

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
            token: (_, _, _),
            expected,
        } => (),
        _ => assert!(false, "wrong error type"),
    };

    let result = parse("1+");
    assert!(result.is_err());
    match result.unwrap_err() {
        #[allow(unused_variables)]
        ParseError::UnrecognizedToken {
            token: (_, _, _),
            expected,
        } => (),
        _ => assert!(false, "wrong error type"),
    };
}

#[test]
fn test_operation() {
    let result = parse("local a = 3 + 22 * -(11 + 65 // 12 << 3 ^ 1) and true or false");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "local a = 3 + 22 * -(11 + 65 // 12 << 3 ^ 1) and true or false"
    );

    let result = parse("b, c = -3+-22*-11^d*1+ 65, ((true and 43) or 43) %3 ^2");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "b, c = -3 + -22 * -11 ^ d * 1 + 65, ((true and 43) or 43) % 3 ^ 2"
    );
}

#[test]
fn test_table() {
    let result = parse("a = { }");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "a = {}");

    let result = parse("a = { 1 }");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "a = { 1 }");

    let result = parse("a = { a, }");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "a = { a }");

    let result = parse("a = { a, b }");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "a = { a, b }");

    let result = parse("a = { a, b, }");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "a = { a, b }");

    let result = parse("a = { a = (1 + 3), }");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "a = { a = (1 + 3) }");

    // let result = parse("a = ({a=3}).a");
    // assert!(result.is_ok(), "{:?}", result);
    // assert_eq!(&format!("{}", result.unwrap()), "a = { a = (1 + 3) }");

    // let result = parse("(({a={b=2}}).a).b = 4");
    // assert!(result.is_ok());
    // assert_eq!(&format!("{}", result.unwrap()), "a = { a = (1 + 3) }");

    let result = parse("a={1,2,  3; 4, 5;6;7 }");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "a = { 1, 2, 3, 4, 5, 6, 7 }"
    );

    let result = parse("a = { a = 1, 2, b = 4, 4 }");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "a = { a = 1, 2, b = 4, 4 }"
    );

    let result = parse("a = { a = { b = true }, [b] = { a = true } }");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "a = { a = { b = true }, [b] = { a = true } }"
    );

    let result = parse("a = tbl[4 + b]");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "a = tbl[4 + b]");

    let result = parse("a = tbl.field.field2");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "a = tbl.field.field2");
}

#[test]
fn test_function() {
    let result = parse("fn_name(a1, a2)");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "fn_name(a1, a2)");

    let result = parse("fn_name{a1, a2}");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "fn_name{ a1, a2 }");

    let result = parse("fn_name:method{a1, a2}");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "fn_name:method{ a1, a2 }");

    let result = parse("fn_name.field:method{a1, a2}");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "fn_name.field:method{ a1, a2 }"
    );

    let result = parse("fn_name(a1, a2).field = 3");
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!( &format!("{}", result.unwrap()), "fn_name(a1, a2).field = 3");

    let result = parse("fn_name{a1, a2}.field = fn().f4");
    assert!(result.is_ok());
    assert_eq!( &format!("{}", result.unwrap()), "fn_name{ a1, a2 }.field = fn().f4");

    let result = parse("fn()()(fn2('abc'))(1, 2)()");
    assert!(result.is_ok());
    assert_eq!( &format!("{}", result.unwrap()), "fn()()(fn2('abc'))(1, 2)()");

    let result = parse("a = fn()().field");
    assert!(result.is_ok());
    assert_eq!( &format!("{}", result.unwrap()), "a = fn()().field");

    let result = parse("a = fn{fn}{fn}(2){fn}.field(3).b");
    assert!(result.is_ok());
    assert_eq!( &format!("{}", result.unwrap()), "a = fn{ fn }{ fn }(2){ fn }.field(3).b");

    let result = parse("fn = function(a) return a end");
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!(
        &format!("{}", result.unwrap()),
        "fn = function(a) return a end"
    );

    let result = parse("fn = function() return end");
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!(
        &format!("{}", result.unwrap()),
        "fn = function() return end"
    );

    let result = parse("fn = function(a, b) return a, b end");
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!(
        &format!("{}", result.unwrap()),
        "fn = function(a, b) return a, b end"
    );

    // no RetStat
    let result = parse("fn = function(a, b) end");
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!(&format!("{}", result.unwrap()), "fn = function(a, b) end");

    let result = parse("fn = function() end");
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!(&format!("{}", result.unwrap()), "fn = function() end");
}

#[test]
fn test_stat() {
    let result = parse(";");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "");

    let result = parse(";;;;;");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "");

    let result = parse("a = 32;;;;;");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "a = 32; ");

    let result = parse(r#"a = "32";;;;b = {3, 4};;;;;c = 45"#);
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "a = \"32\"; b = { 3, 4 }; c = 45"
    );

    let result = parse("a = 3+2; b =12-3; c=-42;");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "a = 3 + 2; b = 12 - 3; c = -42; "
    );
}

#[test]
fn test_for() {
    let result = parse(
        r#"for a in pairs(tbl) do
                            x.fn(a)
                        end"#,
    );
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "for a in pairs(tbl) do x.fn(a) end"
    );

    let result = parse("for a = 5, 1, -1 do x.fn(a) end");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "for a = 5, 1, -1 do x.fn(a) end"
    );

    let result = parse("for a = 1, 5 do x.fn(a) fn(b + 3) end");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "for a = 1, 5 do x.fn(a); fn(b + 3) end"
    );

    let result = parse("while a < 4 do fn(a) fn(b) break end");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "while a < 4 do fn(a); fn(b); break end"
    );

    let result = parse("local a, b repeat fn(a) fn(b) until a > b print(a, b)");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "local a, b; repeat fn(a); fn(b) until a > b; print(a, b)"
    );

    let result = parse(
        r#"local a, b
                          for i in ipairs(tbl) do
                            print(i, a)
                            break
                            return;
                          end
                          a, b = b, a
                          a.b = b
                          b.a = a
                          ::lab1::
                          repeat
                            fn(a)
                            fn(b)
                            return
                          until a > b
                          print(a, b)
                          goto lab1
                          return 4, 6"#,
    );
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()),
               "local a, b; for i in ipairs(tbl) do print(i, a); break return end; a, b = b, a; \
               a.b = b; b.a = a; ::lab1::; repeat fn(a); fn(b) return until a > b; print(a, b); goto lab1 return 4, 6");
}

#[test]
fn test_var() {
    let result = parse("local a, b");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "local a, b");

    let result = parse("local a, b = 4, 4 & 1");
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "local a, b = 4, 4 & 1");

    let result = parse("a, b");
    assert!(result.is_err());

    let result = parse("local a, b, ");
    assert!(result.is_err());

    let result = parse("local a, b = ");
    assert!(result.is_err());

    let result = parse("local , a, b");
    assert!(result.is_err());

    let result = parse("a,b,c = 4, 4 & 1, func(42)");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "a, b, c = 4, 4 & 1, func(42)"
    );
}

#[test]
fn test_literal() {
    let result = parse(r#"a = 123"#);
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "a = 123");

    let result = parse(r#"a = 123.124"#);
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), "a = 123.124");

    let result = parse(r#"a = "123""#);
    assert!(result.is_ok());
    assert_eq!(&format!("{}", result.unwrap()), r#"a = "123""#);

    let result = parse(r#"a = "\"12'3'\"344\"""#);
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "a = \"\\\"12'3'\\\"344\\\"\""
    );

    let result = parse(r#"a = '"12\'3\'"344"\\\''"#);
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        r#"a = '"12\'3\'"344"\\\''"#
    );
}

#[test]
fn test_if() {
    let result = parse("if a + b > 4 then print(a) end");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "if a + b > 4 then print(a) end"
    );

    let result = parse("if a + b > 4 then print(a) else print(b) end");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "if a + b > 4 then print(a) else print(b) end"
    );

    let result = parse("if a + b > 4 then print(a) elseif a+b<-4 then print(b) end");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "if a + b > 4 then print(a) elseif a + b < -4 then print(b) end"
    );

    let result =
        parse("if a + b > 4 then print(a) elseif a+b<-4 then print(b) else print(a+b) end");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "if a + b > 4 then print(a) elseif a + b < -4 then print(b) else print(a + b) end"
    );

    let result =
        parse("if a + b > 4 then print(a) elseif a+b<-4 then print(b) elseif a + b == 0 then print(0) else print(a+b) end");
    assert!(result.is_ok());
    assert_eq!(
        &format!("{}", result.unwrap()),
        "if a + b > 4 then print(a) elseif a + b < -4 then print(b) elseif a + b == 0 then print(0) else print(a + b) end"
    );

    let result = parse("if a + b > 4 then print(a)");
    assert!(result.is_err());

    let result = parse("if a + b > 4 print(a) end");
    assert!(result.is_err());
}
