use super::parse_lua;
use crate::config::{Config, ConfiguredWrite};

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
enum TestError {
    ErrorWhileParsing,
    ErrorWhileWriting,
}

#[allow(dead_code)]
fn ts_base(source: &str, cfg: &Config) -> Result<String, TestError> {
    match parse_lua(source) {
        Err(_) => Err(TestError::ErrorWhileParsing),
        Ok(result) => {
            let mut output = String::new();

            match result.configured_write(&mut output, cfg, source) {
                Ok(_) => Ok(output),
                _ => Err(TestError::ErrorWhileWriting),
            }
        }
    }
}

#[allow(dead_code)]
fn tscln(source: &'static str) -> Result<String, TestError> {
    let cfg = Config {
        remove_comments: Some(true),
        remove_newlines: Some(true),
        normalize_ws: Some(true),
        ..Config::default()
    };
    ts_base(source, &cfg)
}

#[allow(dead_code)]
fn tsdef(source: &str) -> Result<String, TestError> {
    let cfg = Config::default();
    ts_base(source, &cfg)
}

#[test]
fn test_errors() {
    use lalrpop_util::ParseError;

    let result = parse_lua("a = 3 + 22 * ? + 65");
    assert!(result.is_err(), "{:?}", result);
    match result.unwrap_err() {
        #[allow(unused_variables)]
        ParseError::User { error } => (),
        _ => assert!(false, "wrong error type"),
    };

    let result = parse_lua("1++2");
    assert!(result.is_err(), "{:?}", result);
    match result.unwrap_err() {
        #[allow(unused_variables)]
        ParseError::UnrecognizedToken { token: (l, token, r), expected } => (),
        _ => assert!(false, "wrong error type"),
    };

    let result = parse_lua("1 2");
    assert!(result.is_err(), "{:?}", result);
    match result.unwrap_err() {
        #[allow(unused_variables)]
        ParseError::UnrecognizedToken { token: (_, _, _), expected } => (),
        _ => assert!(false, "wrong error type"),
    };

    let result = parse_lua("1+");
    assert!(result.is_err(), "{:?}", result);
    match result.unwrap_err() {
        #[allow(unused_variables)]
        ParseError::UnrecognizedToken { token: (_, _, _), expected } => (),
        _ => assert!(false, "wrong error type"),
    };
}

#[test]
fn test_operation() {
    assert_eq!(
        tscln("local a = 3 + 22 * -(11 + 65 // 12 << 3 ^ 1) and true or false"),
        Ok("local a = 3 + 22 * -(11 + 65 // 12 << 3 ^ 1) and true or false".to_string())
    );
    assert_eq!(
        tscln("b, c = -3+-22*-11^d*1+ 65, ((true and 43) or 43) %3 ^2"),
        Ok("b, c = -3 + -22 * -11 ^ d * 1 + 65, ((true and 43) or 43) % 3 ^ 2".to_string())
    );
    assert_eq!(tscln("local a = 1-2"), Ok("local a = 1 - 2".to_string()));
    assert_eq!(tscln("local a=1- -2"), Ok("local a = 1 - -2".to_string()));
    assert_eq!(tscln("local a=1--2"), Ok("local a = 1".to_string()));
    assert_eq!(tscln("local a=1- - -2"), Err(TestError::ErrorWhileParsing));
    assert_eq!(tscln("local a=1++2"), Err(TestError::ErrorWhileParsing));
}

#[test]
fn test_table() {
    assert_eq!(tscln("a = { }"), Ok("a = {}".to_string()));
    assert_eq!(tscln("a = { 1 }"), Ok("a = { 1 }".to_string()));
    assert_eq!(tscln("a = { a, }"), Ok("a = { a, }".to_string()));
    assert_eq!(tscln("a = { a, b }"), Ok("a = { a, b }".to_string()));
    assert_eq!(tscln("a = { a, b, }"), Ok("a = { a, b, }".to_string()));
    assert_eq!(tscln("a = { a, b, c, d }"), Ok("a = { a, b, c, d }".to_string()));
    assert_eq!(tscln("a = { a = (1 + 3), }"), Ok("a = { a = (1 + 3), }".to_string()));
    assert_eq!(tscln("a = ({a=3}).a"), Ok("a = ({ a = 3 }).a".to_string()));
    assert_eq!(tscln("a = (({a={b=2}}).a).b"), Ok("a = (({ a = { b = 2 } }).a).b".to_string()));
    assert_eq!(tscln("a = ({a={b=2}})[\"a\"][\"b\"]"), Ok("a = ({ a = { b = 2 } })[\"a\"][\"b\"]".to_string()));
    assert_eq!(tscln("a={1,2,  3; 4, 5;6;7 }"), Ok("a = { 1, 2, 3; 4, 5; 6; 7 }".to_string()));
    assert_eq!(tscln("a = { a = 1, 2, b = 4, 4 }"), Ok("a = { a = 1, 2, b = 4, 4 }".to_string()));
    assert_eq!(
        tscln("a = { a = { b = true }, [b] = { a = true } }"),
        Ok("a = { a = { b = true }, [b] = { a = true } }".to_string())
    );
    assert_eq!(tscln("a = tbl[4 + b]"), Ok("a = tbl[4 + b]".to_string()));
    assert_eq!(tscln("a = tbl.field.field2"), Ok("a = tbl.field.field2".to_string()));
}

#[test]
fn test_function() {
    assert_eq!(tscln("fn_name(a1, a2)"), Ok("fn_name(a1, a2)".to_string()));
    assert_eq!(tscln("fn_name{a1, a2}"), Ok("fn_name{ a1, a2 }".to_string()));
    assert_eq!(tscln("fn_name:method{a1, a2}"), Ok("fn_name:method{ a1, a2 }".to_string()));
    assert_eq!(tscln("fn_name.field:method{a1, a2}"), Ok("fn_name.field:method{ a1, a2 }".to_string()));
    assert_eq!(tscln("fn_name(a1, a2).field = 3"), Ok("fn_name(a1, a2).field = 3".to_string()));
    assert_eq!(tscln("fn_name{a1, a2}.field = fn().f4"), Ok("fn_name{ a1, a2 }.field = fn().f4".to_string()));
    assert_eq!(tscln("fn()()(fn2('abc'))(1, 2)()"), Ok("fn()()(fn2('abc'))(1, 2)()".to_string()));
    assert_eq!(tscln("a = fn()().field"), Ok("a = fn()().field".to_string()));
    assert_eq!(tscln("a = fn{fn}{fn}{}(2){fn}.field(3).b"), Ok("a = fn{ fn }{ fn }{}(2){ fn }.field(3).b".to_string()));

    // RetStat
    assert_eq!(tscln("fn = function(a) return a end"), Ok("fn = function(a) return a end".to_string()));
    assert_eq!(tscln("fn = function() return end"), Ok("fn = function() return end".to_string()));
    assert_eq!(tscln("fn = function(a, b) return a, b end"), Ok("fn = function(a, b) return a, b end".to_string()));
    assert_eq!(tscln("function fn() return end"), Ok("function fn() return end".to_string()));
    assert_eq!(tscln("function fn() return; end"), Ok("function fn() return; end".to_string()));
    assert_eq!(tscln("function fn() return;; end"), Err(TestError::ErrorWhileParsing));

    // no RetStat
    assert_eq!(tscln("fn = function(a, b) end"), Ok("fn = function(a, b) end".to_string()));
    assert_eq!(tscln("fn = function() end"), Ok("fn = function() end".to_string()));
    assert_eq!(tscln("fn_name():method():fn{a1, a2}"), Ok("fn_name():method():fn{ a1, a2 }".to_string()));
    assert_eq!(tscln("function Obj:type() print(str) end"), Ok("function Obj:type() print(str) end".to_string()));
    assert_eq!(tscln("local function Obj:type() print(str) end"), Err(TestError::ErrorWhileParsing));
    assert_eq!(
        tscln("local function obj_type() print(str) end"),
        Ok("local function obj_type() print(str) end".to_string())
    );
}

#[test]
fn test_stat() {
    assert_eq!(tscln(";"), Ok(";".to_string()));
    assert_eq!(tscln(";;;;;;;"), Ok(";;;;;;;".to_string()));
    assert_eq!(tscln("a = 32;;;  ;; ;;"), Ok("a = 32;;;;;;;".to_string()));
    assert_eq!(
        tscln(r#"a = "32";;;;b = {3, 4};;;;;c = 45"#),
        Ok("a = \"32\";;;; b = { 3, 4 };;;;; c = 45".to_string())
    );
    assert_eq!(tscln("a = 3+2; b =12-3; c=-42;"), Ok("a = 3 + 2; b = 12 - 3; c = -42;".to_string()));
}

#[test]
fn test_for() {
    assert_eq!(tscln("for a in pairs(tbl) do x.fn(a) end"), Ok("for a in pairs(tbl) do x.fn(a) end".to_string()));
    assert_eq!(tscln("for a = 5, 1, -1 do x.fn(a) end"), Ok("for a = 5, 1, -1 do x.fn(a) end".to_string()));
    assert_eq!(tscln("for a = 1, 5 do x.fn(a) fn(b + 3) end"), Ok("for a = 1, 5 do x.fn(a) fn(b + 3) end".to_string()));
    assert_eq!(tscln("while a < 4 do fn(a) fn(b); break end"), Ok("while a < 4 do fn(a) fn(b); break end".to_string()));
    assert_eq!(
        tscln("local a, b repeat fn(a) fn(b) until a > b print(a, b)"),
        Ok("local a, b repeat fn(a) fn(b) until a > b print(a, b)".to_string())
    );
    assert_eq!(
        tscln(
            r#"  local a, b
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
                return 4, 6"#
        ),
        Ok("local a, b for i in ipairs(tbl) do print(i, a) break return; end a, b = b, a \
            a.b = b b.a = a ::lab1:: repeat fn(a) fn(b) return until a > b print(a, b) goto lab1 return 4, 6"
            .to_string())
    );
}

#[test]
fn test_var() {
    assert_eq!(tscln("local a, b"), Ok("local a, b".to_string()));
    assert_eq!(tscln("local a, b = 4, 4 & 1"), Ok("local a, b = 4, 4 & 1".to_string()));
    assert_eq!(tscln("local a, b"), Ok("local a, b".to_string()));
    assert_eq!(tscln("local a, b = 3, 4"), Ok("local a, b = 3, 4".to_string()));
    assert_eq!(tscln("local a, b, "), Err(TestError::ErrorWhileParsing));
    assert_eq!(tscln("local , a, b"), Err(TestError::ErrorWhileParsing));
    assert_eq!(tscln("a,b,c = 4, 4 & 1, func(42)"), Ok("a, b, c = 4, 4 & 1, func(42)".to_string()));
}

#[test]
fn test_round_prefix() {
    assert_eq!(tscln("(fn2())()"), Ok("(fn2())()".to_string()));
    assert_eq!(tscln("((fn2()))()"), Ok("((fn2()))()".to_string()));
    assert_eq!(
        tscln("((fn2()))(fn2())() fn2().field (fn2())()"),
        Ok("((fn2()))(fn2())() fn2().field(fn2())()".to_string())
    );
    assert_eq!(tscln("a = (((fn2()))())"), Ok("a = (((fn2()))())".to_string()));
    assert_eq!(tscln("({ a = 2}).a = 3"), Ok("({ a = 2 }).a = 3".to_string()));
    assert_eq!(tscln("(fn()):fl().a = 3"), Ok("(fn()):fl().a = 3".to_string()));
    assert_eq!(tscln("(fn()):fl().a, ({}).f = 3, (3&2)"), Ok("(fn()):fl().a, ({}).f = 3, (3 & 2)".to_string()));
    assert_eq!(tscln("local str = ({ a = 3, b = 2 })[param]"), Ok("local str = ({ a = 3, b = 2 })[param]".to_string()));

    // assert_eq!(tscln("a = 3 (fn()):fl().a = 3"), Ok("a = 3 (fn()):fl().a = 3".to_string()));
    // assert_eq!(tscln("({ a = 2}).a = 3 (fn()):fl().a = 3"), Ok("({ a = 2}).a = 3 (fn()):fl().a = 3".to_string()));
    // assert_eq!(
    //     tscln("local p = 'a' ({ a = fn1, b = fn2 })[p]()"),
    //     Ok("local p = 'a' ({ a = fn1, b = fn2 })[p]()".to_string())
    // );
}

#[test]
fn test_literal() {
    assert_eq!(tscln(r#"a = 123"#), Ok("a = 123".to_string()));
    assert_eq!(tscln(r#"a = 123.124"#), Ok("a = 123.124".to_string()));
    assert_eq!(tscln(r#"a = "123""#), Ok(r#"a = "123""#.to_string()));
    assert_eq!(tscln(r#"a = "\"12'3'\"344\"""#), Ok("a = \"\\\"12'3'\\\"344\\\"\"".to_string()));
    assert_eq!(tscln(r#"a = '"12\'3\'"344"\\\''"#), Ok(r#"a = '"12\'3\'"344"\\\''"#.to_string()));
    assert_eq!(tscln("a = [[line]]"), Ok("a = [[line]]".to_string()));
    assert_eq!(tscln("a = [=[line]=]"), Ok("a = [=[line]=]".to_string()));
    assert_eq!(tscln("a = [===[]===]"), Ok("a = [===[]===]".to_string()));
    assert_eq!(tscln("a = [[]]"), Ok("a = [[]]".to_string()));
    assert_eq!(tscln("a = [[\"\n'\"]]"), Ok("a = [[\"\n'\"]]".to_string()));
    assert_eq!(
        tscln("a = [=[line\nnewline]]\n ]==] \n]===]\n]=]"),
        Ok("a = [=[line\nnewline]]\n ]==] \n]===]\n]=]".to_string())
    );
}

#[test]
fn test_if() {
    assert_eq!(tscln("if a + b > 4 then print(a) end"), Ok("if a + b > 4 then print(a) end".to_string()));
    assert_eq!(
        tscln("if a + b > 4 then print(a) else print(b) end"),
        Ok("if a + b > 4 then print(a) else print(b) end".to_string())
    );
    assert_eq!(
        tscln("if a + b > 4 then print(a) elseif a+b<-4 then print(b) end"),
        Ok("if a + b > 4 then print(a) elseif a + b < -4 then print(b) end".to_string())
    );
    assert_eq!(
        tscln("if a + b > 4 then print(a) elseif a+b<-4 then print(b) else print(a+b) end"),
        Ok("if a + b > 4 then print(a) elseif a + b < -4 then print(b) else print(a + b) end".to_string())
    );
    assert_eq!(
        tscln("if a + b > 4 then print(a) elseif a+b<-4 then print(b) elseif a + b == 0 then print(0) else print(a+b) end"),
        Ok("if a + b > 4 then print(a) elseif a + b < -4 then print(b) elseif a + b == 0 then print(0) else print(a + b) end".to_string())
    );
    assert_eq!(tscln("if a + b > 4 then print(a)"), Err(TestError::ErrorWhileParsing));
    assert_eq!(tscln("if a + b > 4 print(a) end"), Err(TestError::ErrorWhileParsing));
}

#[test]
fn test_cut_comment() {
    assert_eq!(tscln("if a + b > 4 then -- comment \n  print(a) -- comment 2 end "), Err(TestError::ErrorWhileParsing));
    assert_eq!(
        tscln("if a + b > 4 then -- comment\n--\n-- \n  print(a) -- comment 2 \nend "),
        Ok("if a + b > 4 then print(a) end".to_string())
    );
    assert_eq!(
        tscln("if a --[[test]]+ b > --[[\nt\ne\tscln\tt\n]] 4 then print(a) end "),
        Ok("if a + b > 4 then print(a) end".to_string())
    );
    assert_eq!(
        tscln("if a --[=[test]=]+ b > --[===[\ntest\n]]===]4 then print(a) end "),
        Ok("if a + b > 4 then print(a) end".to_string())
    );
    assert_eq!(
        tscln("if a --[[test\ntest]]+ --[=[sdf]=] --test\n b > --[===[]===]--[[]]--\n--[[]]4--[[]]--\n then print(a) end "),
        Ok("if a + b > 4 then print(a) end".to_string())
    );
}

#[test]
fn test_numeral() {
    assert_eq!(tscln("local a = 0"), Ok("local a = 0".to_string()));
    assert_eq!(tscln("local a = -12414341423123"), Ok("local a = -12414341423123".to_string()));
    assert_eq!(
        tscln("local a = -124432423412412432142424124.12423"),
        Ok("local a = -124432423412412432142424124.12423".to_string())
    );
    assert_eq!(tscln("local a = -124.12423e0"), Ok("local a = -124.12423e0".to_string()));
    assert_eq!(tscln("local a = -124.12423E-3 e = 4"), Ok("local a = -124.12423E-3 e = 4".to_string()));
    assert_eq!(tscln("local a = .12423E-3 e = 4"), Ok("local a = .12423E-3 e = 4".to_string()));
    assert_eq!(tscln("local a = .0 e = 4"), Ok("local a = .0 e = 4".to_string()));
    assert_eq!(tscln("local a = 0. e = 4"), Ok("local a = 0. e = 4".to_string()));
    assert_eq!(tscln("local a = 0x123 e = 4"), Ok("local a = 0x123 e = 4".to_string()));
    assert_eq!(tscln("local a = 0x123abcdef e = 4"), Ok("local a = 0x123abcdef e = 4".to_string()));
    assert_eq!(tscln("local a = 0x12.4 e = 4"), Err(TestError::ErrorWhileParsing));
    assert_eq!(tscln("local a = 0x12 e = 4"), Ok("local a = 0x12 e = 4".to_string()));
    assert_eq!(tscln("local a = 0x12g e = 4"), Err(TestError::ErrorWhileParsing));
    assert_eq!(tscln("local a = 0x12e-4 e = 4"), Ok("local a = 0x12e - 4 e = 4".to_string()));
}

#[test]
fn test_keep_comments_ops() {
    // binary ops
    for op in vec![
        "+", "-", "or", "and", "==", "~=", ">=", "<=", "<", ">", "|", "~", "&", ">>", "<<", "..", "*", "/", "//", "%",
        "^",
    ] {
        let str = format!("c   --1\n  =  --[=[2]=]   a  --3\n  {}   --[[4]]   b", op);
        assert_eq!(tsdef(&str), Ok(str.to_string()));

        let str = format!("c = a--\n{} --[[342]]b", op);
        assert_eq!(tsdef(&str), Ok(str.to_string()));
    }

    // unary ops
    for op in vec!["not", "-", "#", "~"] {
        let str = format!("c--[=[1]=]=--2\n{}--3\nb", op);
        assert_eq!(tsdef(&str), Ok(str.to_string()));

        let str = format!("c--[=[1]=]=--2\n{} --3\nb", op);
        assert_eq!(tsdef(&str), Ok(str.to_string()));

        let str = format!("c   --1\n  =  --[[2]]   {}  --3\n  b", op);
        assert_eq!(tsdef(&str), Ok(str.to_string()));
    }
}

#[test]
fn test_keep_comments_other() {
    // TableConstructor
    for str in vec![
        "t={--\n}",
        "t = { a --\n  =  --[[]]  3}",
        "t = { [ --c1\n a --[[c2]]] --c3\n= --c4\n 3}",
        "t = { [ --c1\n 'a' --[[c2]]] --c3\n= --c4\n 3}",
        "t = { [ --c1\n \"a\" --[[c2]]] --c3\n= --c4\n 3}",
        "t = { [ --c1\n [[a]] --[[c2]]] --c3\n= --c4\n 3}",
        "t = { --0\n a = 1 --1\n, --2\n b = 2 --3\n, --4\n c = 3 --5\n, --6\n d = 4 --7\n, --8\n e = 5 --9\n, --10\n }",

        // FunctionDef
        "fn = function --1\n( --[[2]]a --3\n  , --4\n b--[[5]] ,--6\n c --[[7]])--8\nend",
        "fn = function --1\n( --[==[2]==]a --3\n  , --4\n b--[[5]] ,--6\n c --[[7]])--[=[8]=]print(a) --[[9]]end",
        "local a = fn--[[1]](--[[2]])",

        // FunctionCall
        "local a = fn--[[1]](--2\na --[[3]],--[[4]]b--5\n, --6\nc--[[7]])",
        "local a = (--1\nfn--[[2]])(--[[3]])",
        "local a = (--1\nfn--[[2]])(  --3\n a --[[4]])",
        "local a = (--1\nfn--[[2]]  (--5\n  ))(  --3\n a --[[4]])",
        "local a = (--1\nfn--[[2]]  (--5\n  4 --[[6]]))(  --3\n a --[[4]])",
        "local a = (--1\nfn--[[2]].--[[7]]fld1--8\n.--9\nfld2--[[10]]:--11\nfnname (--5\n  4 --[[6]]))(  --3\n a --[[4]])",

        // PrefixExp
        "local a = (--1\n{--[[2]]}--[[3]])--4\n[--5\n'a'--[=[6]=]]",
        "(--1\n{--[[2]]}--[[3]])--4\n[--5\n'a'--[=[6]=]]--7\n(--8\n)",

        // Label
        "::--1\nlabel1--[[2]]:: goto--[[3]]label1",
        "::label1:: goto label1",

        // StatRetStat
        "a = b; return",
        "a = b; --[[1]] return",
        "a = b; --[[1]] return--2\n;",
        "a = b; --[[1]] return--2\n2--[[3]],--[[4]]3",
        "a = b; --[[1]] return--2\n2--[[3]],--[[4]]3--5\n;",
        "a = b return",
        "a = b --[[1]] return",
        "a = b --[[1]] return--2\n;",
        "a = b --[[1]] return--2\n2--[[3]],--[[4]]3",
        "a = b --[[1]] return--2\n2--[[3]],--[[4]]3--5\n;",

        // ParStats
        "a = ({})[a]--1\n() --2\n break --3\n ({})--4\n[1]",
        "({})[a]--1\n() --2\n break --3\n ({})--4\n[1]",

        // If Then ElseIf Else End
        r#"if --[[1]] a > 3 --[[2]] then --[[3]] print(4) --[[4]] elseif --[[5]]a<3--[[6]] then --[[7]]print(2)--[[8]]
elseif --[[9]]a == 3 --[[10]]then--[[11]] print(3)--[[12]] else--[[13]] print(0) --[[14]]end"#,
        r#"if --[[1]] a > 3 --[[2]] then --[[3]] elseif --[[5]]a<3--[[6]] then --[[7]]
elseif --[[9]]a == 3 --[[10]]then--[[11]] print(3)--[[12]] else--[[13]] print(0) --[[14]]end"#,
        r#"if --[[1]] a > 3 --[[2]] then --[[3]] print(4) --[[4]] elseif --[[5]]a<3--[[6]] then --[[7]]print(2)--[[8]]
elseif --[[9]]a == 3 --[[10]]then--[[11]] print(3)--[[12]] else--[[13]] end"#,
        r#"if --[[1]] a > 3 --[[2]] then --[[3]] elseif --[[5]]a<3--[[6]] then --[[7]]print(2)--[[8]]
elseif --[[9]]a == 3 --[[10]]then--[[11]] print(3)--[[12]] else--[[13]] end"#,
        r#"if --[[1]] a > 3 --[[2]] then --[[3]] print(4) --[[4]] else--[[13]] print(0) --[[14]]end"#,
        r#"if --[[1]] a > 3 --[[2]] then --[[3]] else--[[13]] print(0) --[[14]]end"#,
        r#"if --[[1]] a > 3 --[[2]] then --[[3]] print(4) --[[4]] else--[[13]] end"#,
        r#"if --[[1]] a > 3 --[[2]] then --[[3]] else--[[13]] end"#,
        r#"if --[[1]] a > 3 --[[2]] then --[[3]] print(4) --[[4]] elseif --[[5]]a<3--[[6]] then --[[7]]print(2)--[[8]]
elseif --[[9]]a == 3 --[[10]]then--[[11]] print(3)--[[12]] end"#,
        r#"if --[[1]] a > 3 --[[2]] then --[[3]] elseif --[[5]]a<3--[[6]] then --[[7]]print(2)--[[8]]
elseif --[[9]]a == 3 --[[10]]then--[[11]] print(3)--[[12]] end"#,
        r#"if --[[1]] a > 3 --[[2]] then --[[3]] print(4) --[[4]] end"#,
        r#"if --[[1]] a > 3 --[[2]] then --[[3]] end"#,

        // local names = exprs
        "local --[[1]]a",
        "local --[[1]]a--2\n,--[[7]] b",
        "local --[[1]]a--2\n=--[[7]] 1",
        "local --[[1]]a--2\n,--3\n b --[[4]],--[[5]] c--6\n =--[[7]] 1--8\n,--9\n 2--10\n, --11\n3",

        // FuncDecl
        "function  --[[1]] a--2\n.--3\nb--[[4]](--5\na --6\n,--[[7]] b--[[8]])--[[9]] end",
        "function  --[[1]] a--2\n.--3\nb--[[4]](--5\na --6\n,--[[7]] b--[[8]])--[[9]] print(a) --[[10]] end",
        "local --[[0]] function  --[[1]] b--[[4]](--5\na --6\n,--[[7]] b--[[8]])--[[9]] end",
        "local --[[0]] function  --[[1]] b--[[4]](--5\na --6\n,--[[7]] b--[[8]])--[[9]] print(a)--[[10]]end",
        "function fn(  --5\n   --[[6]] )end",

        // For
        "for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\n, --7\n1 --8\ndo--9\n print(a) --[[10]]end",
        "for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\n, --7\n1 --8\ndo--9\n --[[10]]end",
        "for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\ndo--9\n print(a) --[[10]]end",
        "for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\ndo--9\n --[[10]]end",
        "for --[[1]]a--[[2]] in--[[3]] ipairs(t)--4\ndo--9\n print(a) --[[10]]end",
        "for --[[1]]a--[[2]] in--[[3]] ipairs(t)--4\ndo--9\n --[[10]]end",

        // WhileDo
        "while --1\n a < 3 --[[2]]do --[[3]]a = a + 1 --4\nend",
        "while --1\n a < 3 --[[2]]do--5\n --[[3]]end",

        // DoEnd
        "do--1\n print(a)--[[2]]end",
        "do--1\n --[[2]] end",

        // RepeatUntil
        "repeat--[[1]] print(a)--2\nuntil --[[3]]  a < 4",
        "repeat--[[1]] --2\nuntil --[[3]]  a < 4",

        // VarsExprs
        "a--[[1]].--2\nt--[[3]],--[[4]] a--5\n.--6\nr--[[7]] =--8\n 2--[[9]], --10\n'as'",
        "(--[[0]]a--[[1]])--11\n.--2\nt--[[3]],--[[4]] a--5\n.--6\nr--[[7]] =--8\n 2--[[9]], --10\n'as'",
    ] {
        assert_eq!(tsdef(str), Ok(str.to_string()));
    }
}

#[test]
fn test_keep_comments_special() {
    assert_eq!(tsdef("   "), Ok("   ".to_string()));
    assert_eq!(tsdef(" ;   ; "), Ok(" ;   ; ".to_string()));
    assert_eq!(
        tsdef(" ;;; ;;;  ;;;;;; ;; ;; --[[]] ;;; ;; ;"),
        Ok(" ;;; ;;;  ;;;;;; ;; ;; --[[]] ;;; ;; ;".to_string())
    );
    assert_eq!(tsdef("--[[1]]"), Ok("--[[1]]".to_string()));
    assert_eq!(tsdef("--[[1]] ; --2\n "), Ok("--[[1]] ; --2\n ".to_string()));
    assert_eq!(tsdef("--[[1]]  --2\n "), Ok("--[[1]]  --2\n ".to_string()));
    assert_eq!(tsdef("--[[1]] print(a) --2\n "), Ok("--[[1]] print(a) --2\n ".to_string()));
    assert_eq!(tsdef("#!/usr/bin/lua\n  --[[]] "), Ok("#!/usr/bin/lua\n  --[[]] ".to_string()));
    assert_eq!(tsdef("#!/usr/bin/lua\n"), Ok("#!/usr/bin/lua\n".to_string()));
    assert_eq!(tsdef("#!/usr/bin/lua\n --[[]] local a = 32"), Ok("#!/usr/bin/lua\n --[[]] local a = 32".to_string()));
    assert_eq!(tsdef("\n\n#!/usr/bin/lua\n --[[]] local a = 32"), Ok("\n\n#!/usr/bin/lua\n --[[]] local a = 32".to_string()));
    assert_eq!(
        tsdef("#!/usr/bin/lua\n --[[]] local a = 32 --3\n"),
        Ok("#!/usr/bin/lua\n --[[]] local a = 32 --3\n".to_string())
    );

    // special
    assert_eq!(tsdef("#!/usr/bin/lua"), Ok("#!/usr/bin/lua\n".to_string()));
    assert_eq!(tsdef("#!"), Ok("#!\n".to_string()));
}
