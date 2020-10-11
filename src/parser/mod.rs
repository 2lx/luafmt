#[allow(clippy::all)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod syntax;
mod lexer;
pub mod nodes;
use super::config::{Config, ConfiguredWrite};

use lalrpop_util::ParseError;

pub fn parse(src: &str) -> Result<nodes::Node, ParseError<usize, lexer::Token, lexer::LexicalError>> {
    let lexer = lexer::Lexer::new(src);
    syntax::ChunkParser::new().parse(src, lexer)
}

#[allow(dead_code)]
static TEST_CONFIG: Config = Config {
    indent_width: 4,
    keep_comments: false,
};

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
enum TestError {
    ErrorWhileParsing,
    ErrorWhileWriting,
}

#[allow(dead_code)]
fn ts_base(source: &'static str, cfg: &Config) -> Result<String, TestError> {
    match parse(source) {
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
fn ts(source: &'static str) -> Result<String, TestError> {
    ts_base(source, &TEST_CONFIG)
}

#[allow(dead_code)]
fn ts_comments(source: &'static str) -> Result<String, TestError> {
    let cfg = Config {
        indent_width: 4,
        keep_comments: true,
    };
    ts_base(source, &cfg)
}

#[test]
fn test_errors() {
    let result = parse("a = 3 + 22 * ? + 65");
    assert!(result.is_err(), "{:?}", result);
    match result.unwrap_err() {
        #[allow(unused_variables)]
        ParseError::User { error } => (),
        _ => assert!(false, "wrong error type"),
    };

    let result = parse("1++2");
    assert!(result.is_err(), "{:?}", result);
    match result.unwrap_err() {
        #[allow(unused_variables)]
        ParseError::UnrecognizedToken {
            token: (l, token, r),
            expected,
        } => (),
        _ => assert!(false, "wrong error type"),
    };

    let result = parse("1 2");
    assert!(result.is_err(), "{:?}", result);
    match result.unwrap_err() {
        #[allow(unused_variables)]
        ParseError::UnrecognizedToken {
            token: (_, _, _),
            expected,
        } => (),
        _ => assert!(false, "wrong error type"),
    };

    let result = parse("1+");
    assert!(result.is_err(), "{:?}", result);
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
    assert_eq!(
        ts("local a = 3 + 22 * -(11 + 65 // 12 << 3 ^ 1) and true or false"),
        Ok("local a = 3 + 22 * -(11 + 65 // 12 << 3 ^ 1) and true or false".to_string())
    );
    assert_eq!(
        ts("b, c = -3+-22*-11^d*1+ 65, ((true and 43) or 43) %3 ^2"),
        Ok("b, c = -3 + -22 * -11 ^ d * 1 + 65, ((true and 43) or 43) % 3 ^ 2".to_string())
    );
    assert_eq!(ts("local a = 1-2"), Ok("local a = 1 - 2".to_string()));
    assert_eq!(ts("local a=1- -2"), Ok("local a = 1 - -2".to_string()));
    assert_eq!(ts("local a=1--2"), Ok("local a = 1".to_string()));
    assert_eq!(ts("local a=1- - -2"), Err(TestError::ErrorWhileParsing));
    assert_eq!(ts("local a=1++2"), Err(TestError::ErrorWhileParsing));
}

#[test]
fn test_table() {
    assert_eq!(ts("a = { }"), Ok("a = {}".to_string()));
    assert_eq!(ts("a = { 1 }"), Ok("a = { 1 }".to_string()));
    assert_eq!(ts("a = { a, }"), Ok("a = { a }".to_string()));
    assert_eq!(ts("a = { a, b }"), Ok("a = { a, b }".to_string()));
    assert_eq!(ts("a = { a, b, }"), Ok("a = { a, b }".to_string()));
    assert_eq!(ts("a = { a = (1 + 3), }"), Ok("a = { a = (1 + 3) }".to_string()));
    assert_eq!(ts("a = ({a=3}).a"), Ok("a = ({ a = 3 }).a".to_string()));
    assert_eq!(
        ts("a = (({a={b=2}}).a).b"),
        Ok("a = (({ a = { b = 2 } }).a).b".to_string())
    );
    assert_eq!(
        ts("a = ({a={b=2}})[\"a\"][\"b\"]"),
        Ok("a = ({ a = { b = 2 } })[\"a\"][\"b\"]".to_string())
    );
    assert_eq!(
        ts("a={1,2,  3; 4, 5;6;7 }"),
        Ok("a = { 1, 2, 3, 4, 5, 6, 7 }".to_string())
    );
    assert_eq!(
        ts("a = { a = 1, 2, b = 4, 4 }"),
        Ok("a = { a = 1, 2, b = 4, 4 }".to_string())
    );
    assert_eq!(
        ts("a = { a = { b = true }, [b] = { a = true } }"),
        Ok("a = { a = { b = true }, [b] = { a = true } }".to_string())
    );
    assert_eq!(ts("a = tbl[4 + b]"), Ok("a = tbl[4 + b]".to_string()));
    assert_eq!(ts("a = tbl.field.field2"), Ok("a = tbl.field.field2".to_string()));
}

#[test]
fn test_function() {
    assert_eq!(ts("fn_name(a1, a2)"), Ok("fn_name(a1, a2)".to_string()));
    assert_eq!(ts("fn_name{a1, a2}"), Ok("fn_name{ a1, a2 }".to_string()));
    assert_eq!(ts("fn_name:method{a1, a2}"), Ok("fn_name:method{ a1, a2 }".to_string()));
    assert_eq!(
        ts("fn_name.field:method{a1, a2}"),
        Ok("fn_name.field:method{ a1, a2 }".to_string())
    );
    assert_eq!(
        ts("fn_name(a1, a2).field = 3"),
        Ok("fn_name(a1, a2).field = 3".to_string())
    );
    assert_eq!(
        ts("fn_name{a1, a2}.field = fn().f4"),
        Ok("fn_name{ a1, a2 }.field = fn().f4".to_string())
    );
    assert_eq!(
        ts("fn()()(fn2('abc'))(1, 2)()"),
        Ok("fn()()(fn2('abc'))(1, 2)()".to_string())
    );
    assert_eq!(ts("a = fn()().field"), Ok("a = fn()().field".to_string()));
    assert_eq!(
        ts("a = fn{fn}{fn}(2){fn}.field(3).b"),
        Ok("a = fn{ fn }{ fn }(2){ fn }.field(3).b".to_string())
    );

    // RetStat
    assert_eq!(
        ts("fn = function(a) return a end"),
        Ok("fn = function(a) return a end".to_string())
    );
    assert_eq!(
        ts("fn = function() return end"),
        Ok("fn = function() return end".to_string())
    );
    assert_eq!(
        ts("fn = function(a, b) return a, b end"),
        Ok("fn = function(a, b) return a, b end".to_string())
    );

    // no RetStat
    assert_eq!(ts("fn = function(a, b) end"), Ok("fn = function(a, b) end".to_string()));
    assert_eq!(ts("fn = function() end"), Ok("fn = function() end".to_string()));
    assert_eq!(
        ts("fn_name():method():fn{a1, a2}"),
        Ok("fn_name():method():fn{ a1, a2 }".to_string())
    );
    assert_eq!(
        ts("function Obj:type() print(str) end"),
        Ok("function Obj:type() print(str) end".to_string())
    );
}

#[test]
fn test_stat() {
    assert_eq!(ts(";"), Ok("".to_string()));
    assert_eq!(ts(";;;;;;;"), Ok("".to_string()));
    assert_eq!(ts("a = 32;;;;;;;"), Ok("a = 32; ".to_string()));
    assert_eq!(
        ts(r#"a = "32";;;;b = {3, 4};;;;;c = 45"#),
        Ok("a = \"32\"; b = { 3, 4 }; c = 45".to_string())
    );
    assert_eq!(
        ts("a = 3+2; b =12-3; c=-42;"),
        Ok("a = 3 + 2; b = 12 - 3; c = -42; ".to_string())
    );
}

#[test]
fn test_for() {
    assert_eq!(
        ts(r#"for a in pairs(tbl) do
                            x.fn(a)
                        end"#),
        Ok("for a in pairs(tbl) do x.fn(a) end".to_string())
    );
    assert_eq!(
        ts("for a = 5, 1, -1 do x.fn(a) end"),
        Ok("for a = 5, 1, -1 do x.fn(a) end".to_string())
    );
    assert_eq!(
        ts("for a = 1, 5 do x.fn(a) fn(b + 3) end"),
        Ok("for a = 1, 5 do x.fn(a); fn(b + 3) end".to_string())
    );
    assert_eq!(
        ts("while a < 4 do fn(a) fn(b) break end"),
        Ok("while a < 4 do fn(a); fn(b); break end".to_string())
    );
    assert_eq!(
        ts("local a, b repeat fn(a) fn(b) until a > b print(a, b)"),
        Ok("local a, b; repeat fn(a); fn(b) until a > b; print(a, b)".to_string())
    );
    assert_eq!(
        ts(r#"local a, b
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
  return 4, 6"#),
        Ok(
            "local a, b; for i in ipairs(tbl) do print(i, a); break return end; a, b = b, a; \
               a.b = b; b.a = a; ::lab1::; repeat fn(a); fn(b) return until a > b; print(a, b); goto lab1 return 4, 6"
                .to_string()
        )
    );
}

#[test]
fn test_var() {
    assert_eq!(ts("local a, b"), Ok("local a, b".to_string()));
    assert_eq!(ts("local a, b = 4, 4 & 1"), Ok("local a, b = 4, 4 & 1".to_string()));
    assert_eq!(ts("local a, b"), Ok("local a, b".to_string()));
    assert_eq!(ts("local a, b = 3, 4"), Ok("local a, b = 3, 4".to_string()));
    assert_eq!(ts("local a, b, "), Err(TestError::ErrorWhileParsing));
    assert_eq!(ts("local , a, b"), Err(TestError::ErrorWhileParsing));
    assert_eq!(
        ts("a,b,c = 4, 4 & 1, func(42)"),
        Ok("a, b, c = 4, 4 & 1, func(42)".to_string())
    );
}

#[test]
fn test_round_prefix() {
    assert_eq!(ts("(fn2())()"), Ok("(fn2())()".to_string()));
    assert_eq!(ts("((fn2()))()"), Ok("((fn2()))()".to_string()));
    assert_eq!(
        ts("((fn2()))(fn2())() fn2().field (fn2())()"),
        Ok("((fn2()))(fn2())(); fn2().field(fn2())()".to_string())
    );
    assert_eq!(ts("a = (((fn2()))())"), Ok("a = (((fn2()))())".to_string()));
    assert_eq!(ts("({ a = 2}).a = 3"), Ok("({ a = 2 }).a = 3".to_string()));
    assert_eq!(ts("(fn()):fl().a = 3"), Ok("(fn()):fl().a = 3".to_string()));
    assert_eq!(
        ts("(fn()):fl().a, ({}).f = 3, (3&2)"),
        Ok("(fn()):fl().a, ({}).f = 3, (3 & 2)".to_string())
    );
    assert_eq!(
        ts("local str = ({ a = 3, b = 2 })[param]"),
        Ok("local str = ({ a = 3, b = 2 })[param]".to_string())
    );
    // assert_eq!(
    //     ts("a = 3 (fn()):fl().a = 3"),
    //     Ok("a = 3 (fn()):fl().a = 3".to_string())
    // );
    // assert_eq!(
    //     ts("({ a = 2}).a = 3 (fn()):fl().a = 3"),
    //     Ok("({ a = 2}).a = 3 (fn()):fl().a = 3".to_string())
    // );
    // assert_eq!(
    //     ts("local p = 'a' ({ a = fn1, b = fn2 })[p]()"),
    //     Ok("local p = 'a' ({ a = fn1, b = fn2 })[p]()".to_string())
    // );
}

#[test]
fn test_literal() {
    assert_eq!(ts(r#"a = 123"#), Ok("a = 123".to_string()));
    assert_eq!(ts(r#"a = 123.124"#), Ok("a = 123.124".to_string()));
    assert_eq!(ts(r#"a = "123""#), Ok(r#"a = "123""#.to_string()));
    assert_eq!(
        ts(r#"a = "\"12'3'\"344\"""#),
        Ok("a = \"\\\"12'3'\\\"344\\\"\"".to_string())
    );
    assert_eq!(
        ts(r#"a = '"12\'3\'"344"\\\''"#),
        Ok(r#"a = '"12\'3\'"344"\\\''"#.to_string())
    );
    assert_eq!(ts("a = [[line]]"), Ok("a = [[line]]".to_string()));
    assert_eq!(ts("a = [=[line]=]"), Ok("a = [=[line]=]".to_string()));
    assert_eq!(ts("a = [===[]===]"), Ok("a = [===[]===]".to_string()));
    assert_eq!(ts("a = [[]]"), Ok("a = [[]]".to_string()));
    assert_eq!(ts("a = [[\"\n'\"]]"), Ok("a = [[\"\n'\"]]".to_string()));
    assert_eq!(
        ts("a = [=[line\nnewline]]\n ]==] \n]===]\n]=]"),
        Ok("a = [=[line\nnewline]]\n ]==] \n]===]\n]=]".to_string())
    );
}

#[test]
fn test_if() {
    assert_eq!(
        ts("if a + b > 4 then print(a) end"),
        Ok("if a + b > 4 then print(a) end".to_string())
    );
    assert_eq!(
        ts("if a + b > 4 then print(a) else print(b) end"),
        Ok("if a + b > 4 then print(a) else print(b) end".to_string())
    );
    assert_eq!(
        ts("if a + b > 4 then print(a) elseif a+b<-4 then print(b) end"),
        Ok("if a + b > 4 then print(a) elseif a + b < -4 then print(b) end".to_string())
    );
    assert_eq!(
        ts("if a + b > 4 then print(a) elseif a+b<-4 then print(b) else print(a+b) end"),
        Ok("if a + b > 4 then print(a) elseif a + b < -4 then print(b) else print(a + b) end".to_string())
    );
    assert_eq!(
        ts("if a + b > 4 then print(a) elseif a+b<-4 then print(b) elseif a + b == 0 then print(0) else print(a+b) end"),
        Ok("if a + b > 4 then print(a) elseif a + b < -4 then print(b) elseif a + b == 0 then print(0) else print(a + b) end".to_string())
    );
    assert_eq!(ts("if a + b > 4 then print(a)"), Err(TestError::ErrorWhileParsing));
    assert_eq!(ts("if a + b > 4 print(a) end"), Err(TestError::ErrorWhileParsing));
}

#[test]
fn test_cut_comment() {
    assert_eq!(
        ts("if a + b > 4 then -- comment \n  print(a) -- comment 2 end "),
        Err(TestError::ErrorWhileParsing)
    );
    assert_eq!(
        ts("if a + b > 4 then -- comment\n--\n-- \n  print(a) -- comment 2 \nend "),
        Ok("if a + b > 4 then print(a) end".to_string())
    );
    assert_eq!(
        ts("if a --[[test]]+ b > --[[\nt\ne\ts\tt\n]] 4 then print(a) end "),
        Ok("if a + b > 4 then print(a) end".to_string())
    );
    assert_eq!(
        ts("if a --[=[test]=]+ b > --[===[\ntest\n]]===]4 then print(a) end "),
        Ok("if a + b > 4 then print(a) end".to_string())
    );
    assert_eq!(
        ts("if a --[[test\ntest]]+ --[=[sdf]=] --test\n b > --[===[]===]--[[]]--\n--[[]]4--[[]]--\n then print(a) end "),
        Ok("if a + b > 4 then print(a) end".to_string())
    );
}

#[test]
fn test_numeral() {
    assert_eq!(ts("local a = 0"), Ok("local a = 0".to_string()));
    assert_eq!(
        ts("local a = -12414341423123"),
        Ok("local a = -12414341423123".to_string())
    );
    assert_eq!(
        ts("local a = -124432423412412432142424124.12423"),
        Ok("local a = -124432423412412432142424124.12423".to_string())
    );
    assert_eq!(ts("local a = -124.12423e0"), Ok("local a = -124.12423e0".to_string()));
    assert_eq!(
        ts("local a = -124.12423E-3 e = 4"),
        Ok("local a = -124.12423E-3; e = 4".to_string())
    );
    assert_eq!(
        ts("local a = .12423E-3 e = 4"),
        Ok("local a = .12423E-3; e = 4".to_string())
    );
    assert_eq!(ts("local a = .0 e = 4"), Ok("local a = .0; e = 4".to_string()));
    assert_eq!(ts("local a = 0. e = 4"), Ok("local a = 0.; e = 4".to_string()));
    assert_eq!(ts("local a = 0x123 e = 4"), Ok("local a = 0x123; e = 4".to_string()));
    assert_eq!(
        ts("local a = 0x123abcdef e = 4"),
        Ok("local a = 0x123abcdef; e = 4".to_string())
    );
    assert_eq!(ts("local a = 0x12.4 e = 4"), Err(TestError::ErrorWhileParsing));
    assert_eq!(ts("local a = 0x12 e = 4"), Ok("local a = 0x12; e = 4".to_string()));
    assert_eq!(ts("local a = 0x12g e = 4"), Err(TestError::ErrorWhileParsing));
    assert_eq!(
        ts("local a = 0x12e-4 e = 4"),
        Ok("local a = 0x12e - 4; e = 4".to_string())
    );
}

#[test]
fn test_keep_comments_op() {
    assert_eq!(
        ts_comments("if a  --\n  +   --[[]]   b > 4 then print(a) end "),
        Ok("if a --\n +  --[[]]b > 4 then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if a  --\n  -   --[[]]   b > 4 then print(a) end "),
        Ok("if a --\n -  --[[]]b > 4 then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if a  --\n  or   --[[]]   b > 4 then print(a) end "),
        Ok("if a --\n or  --[[]]b > 4 then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if a  --\n  and   --[[]]   b > 4 then print(a) end "),
        Ok("if a --\n and  --[[]]b > 4 then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if a  --\n  ==  --[[]] b then print(a) end "),
        Ok("if a --\n ==  --[[]]b then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if a  --\n  ~=  --[[]] b then print(a) end "),
        Ok("if a --\n ~=  --[[]]b then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if a  --\n  >=  --[[]] b then print(a) end "),
        Ok("if a --\n >=  --[[]]b then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if a  --\n  <=  --[[]] b then print(a) end "),
        Ok("if a --\n <=  --[[]]b then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if a  --\n  <  --[[]] b then print(a) end "),
        Ok("if a --\n <  --[[]]b then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if a  --\n  >  --[[]] b then print(a) end "),
        Ok("if a --\n >  --[[]]b then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if a  --\n  |  --[[]] b then print(a) end "),
        Ok("if a --\n |  --[[]]b then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if a  --\n  ~  --[[]] b then print(a) end "),
        Ok("if a --\n ~  --[[]]b then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if a  --\n  &  --[[]] b then print(a) end "),
        Ok("if a --\n &  --[[]]b then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if a  --\n  >>  --[[]] b then print(a) end "),
        Ok("if a --\n >>  --[[]]b then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if a  --\n  <<  --[[]] b then print(a) end "),
        Ok("if a --\n <<  --[[]]b then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if a  --\n  ..  --[[]] b then print(a) end "),
        Ok("if a --\n ..  --[[]]b then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if a  --\n  *  --[[]] b then print(a) end "),
        Ok("if a --\n *  --[[]]b then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if a  --\n  /  --[[]] b then print(a) end "),
        Ok("if a --\n /  --[[]]b then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if a  --\n  //  --[[]] b then print(a) end "),
        Ok("if a --\n //  --[[]]b then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if a  --\n  %  --[[]] b then print(a) end "),
        Ok("if a --\n %  --[[]]b then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if a  --\n  ^  --[[]] b then print(a) end "),
        Ok("if a --\n ^  --[[]]b then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if not  --\n  b then print(a) end "),
        Ok("if not  --\nb then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if -  --\n  b then print(a) end "),
        Ok("if - --\nb then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if #  --\n  b then print(a) end "),
        Ok("if # --\nb then print(a) end".to_string())
    );
    assert_eq!(
        ts_comments("if ~  --\n  b then print(a) end "),
        Ok("if ~ --\nb then print(a) end".to_string())
    );
}
