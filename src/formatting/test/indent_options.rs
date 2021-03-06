use super::common::*;
use crate::config::*;

#[test]
fn test_indent_do_end() {
    let cfg = Config::default();
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(ts("do print(a) print(b) end"), Ok("do print(a) print(b) end".to_string()));
    assert_eq!(
        ts("do --comment\n print(a) print(b) --[[123]] end"),
        Ok("do --comment\n print(a) print(b) --[[123]] end".to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            newline_format_do_end: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(
        ts("do print(a) print(b) end"),
        Ok(r#"do
I   print(a) print(b)
end"#
            .to_string())
    );

    assert_eq!(
        ts("do--comm\n   print(a) print(b) --[[123]]end"),
        Ok(r#"do--comm
I   print(a) print(b) --[[123]]
end"#
            .to_string())
    );

    assert_eq!(
        ts(r#"do--comm
    print(a)
    print(b) --[[123]]
    end"#),
        Ok(r#"do--comm
I   print(a)
    print(b) --[[123]]
end"#
            .to_string())
    );

    assert_eq!(
        ts(r#"do --[[123]] print(a)
    print(b) --[[123]]
    --123
    --[[345]]end"#),
        Ok(r#"do --[[123]]
I   print(a)
    print(b) --[[123]]
    --123
    --[[345]]
end"#
            .to_string())
    );
}

#[test]
fn test_newline_format_statement() {
    let cfg = Config::default();
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(ts("do print(a) print(b) print(c) end"), Ok("do print(a) print(b) print(c) end".to_string()));
    assert_eq!(
        ts("for i = 1, 3 do --[[1]] print(a) --2\n print(b) --3\n print(c) --[[4]] end"),
        Ok("for i = 1, 3 do --[[1]] print(a) --2\n print(b) --3\n print(c) --[[4]] end".to_string())
    );
    assert_eq!(
        ts("while a > 4 do --[[1]] print(a) --2\n print(b) --3\n print(c) --[[4]] end"),
        Ok("while a > 4 do --[[1]] print(a) --2\n print(b) --3\n print(c) --[[4]] end".to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("INDENT".to_string()),
            newline_format_statement: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(
        ts("do print(a) print(b) print(c) end"),
        Ok(r#"do print(a)
INDENTprint(b)
INDENTprint(c) end"#
            .to_string())
    );
    assert_eq!(
        ts("for i = 1, 3 do --[[1]] print(a) --2\n print(b) --[[3]] print(c) --[[4]] end"),
        Ok("for i = 1, 3 do --[[1]] print(a) --2
INDENTprint(b) --[[3]]
INDENTprint(c) --[[4]] end"
            .to_string())
    );
    //     assert_eq!(
    //         ts("while a > 4 do --[[1]] print(a) --2\n print(b) --3\n print(c) --[[4]] end"),
    //         Ok("while a > 4 do --[[1]] print(a) --2
    // INDENTprint(b) --3
    // INDENTprint(c) --[[4]] end"
    //             .to_string())
    //     );
}

#[test]
fn test_indent_for() {
    let cfg = Config::default();
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(
        ts("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\n, --7\n1 --8\ndo--9\n print(a) --[[10]]end"),
        Ok("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\n, --7\n1 --8\ndo--9\n print(a) --[[10]]end".to_string())
    );
    assert_eq!(
        ts("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\n, --7\n1 --8\ndo--9\n --[[10]]end"),
        Ok("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\n, --7\n1 --8\ndo--9\n --[[10]]end".to_string())
    );
    assert_eq!(
        ts("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\ndo--9\n print(a) --[[10]]end"),
        Ok("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\ndo--9\n print(a) --[[10]]end".to_string())
    );
    assert_eq!(
        ts("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\ndo--9\n --[[10]]end"),
        Ok("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\ndo--9\n --[[10]]end".to_string())
    );
    assert_eq!(
        ts("for --[[1]]a--[[2]] in--[[3]] ipairs(t)--4\ndo--9\n print(a) --[[10]]end"),
        Ok("for --[[1]]a--[[2]] in--[[3]] ipairs(t)--4\ndo--9\n print(a) --[[10]]end".to_string())
    );
    assert_eq!(
        ts("for --[[1]]a--[[2]] in--[[3]] ipairs(t)--4\ndo--9\n --[[10]]end"),
        Ok("for --[[1]]a--[[2]] in--[[3]] ipairs(t)--4\ndo--9\n --[[10]]end".to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("INDENT".to_string()),
            newline_format_for: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\n, --7\n1 --8\ndo--9\n print(a) --[[10]]end"),
        Ok("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\n, --7\n1 --8\ndo--9\nINDENTprint(a) --[[10]]\nend"
            .to_string())
    );
    assert_eq!(
        ts("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\n, --7\n1 --8\ndo--[[9]] print(a) --[[10]]end"),
        Ok("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\n, --7\n1 --8\ndo--[[9]]\nINDENTprint(a) --[[10]]\nend"
            .to_string())
    );

    assert_eq!(
        ts("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\n, --7\n1 --8\ndo--9\n --[[10]]end"),
        Ok("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\n, --7\n1 --8\ndo--9\n --[[10]]\nend".to_string())
    );

    assert_eq!(
        ts("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\ndo--9\n print(a) --[[10]]end"),
        Ok("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\ndo--9\nINDENTprint(a) --[[10]]\nend".to_string())
    );
    assert_eq!(
        ts("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\ndo--[[9]] print(a) --[[10]]end"),
        Ok("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\ndo--[[9]]\nINDENTprint(a) --[[10]]\nend".to_string())
    );

    assert_eq!(
        ts("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\ndo--9\n --[[10]]end"),
        Ok("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\ndo--9\n --[[10]]end".to_string())
    );
    assert_eq!(
        ts("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\ndo--[[9]] --[[10]]end"),
        Ok("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\ndo--[[9]] --[[10]]end".to_string())
    );

    assert_eq!(
        ts("for --[[1]]a--[[2]] in--[[3]] ipairs(t)--4\ndo--9\n print(a) --[[10]]end"),
        Ok("for --[[1]]a--[[2]] in--[[3]] ipairs(t)--4\ndo--9\nINDENTprint(a) --[[10]]\nend".to_string())
    );
    assert_eq!(
        ts("for --[[1]]a--[[2]] in--[[3]] ipairs(t)--4\ndo--[[9\n]] print(a) --[[10]]end"),
        Ok("for --[[1]]a--[[2]] in--[[3]] ipairs(t)--4\ndo--[[9\n]]\nINDENTprint(a) --[[10]]\nend".to_string())
    );

    assert_eq!(
        ts("for --[[1]]a--[[2]] in--[[3]] ipairs(t)--4\ndo--9\n --[[10]]end"),
        Ok("for --[[1]]a--[[2]] in--[[3]] ipairs(t)--4\ndo--9\n --[[10]]\nend".to_string())
    );
    assert_eq!(
        ts("for --[[1]]a--[[2]] in--[[3]] ipairs(t)--4\ndo--[[9]] --[[10]]end"),
        Ok("for --[[1]]a--[[2]] in--[[3]] ipairs(t)--4\ndo--[[9]] --[[10]]\nend".to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("INDENT".to_string()),
            newline_format_for: Some(1),
            newline_format_statement: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\n, --7\n1 --8\ndo--9\n print(a) a=b print(c) --[[10]]end"),
        Ok("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\n, --7\n1 --8\ndo--9\nINDENTprint(a)\nINDENTa=b\nINDENTprint(c) --[[10]]\nend"
            .to_string())
    );
}

#[test]
fn test_indent_if_then_else() {
    let cfg = Config::default();
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(ts("if a > b then print(a) print(b) end"), Ok("if a > b then print(a) print(b) end".to_string()));
    assert_eq!(
        ts("if a > b --comment\n then print(a) print(b) end"),
        Ok("if a > b --comment\n then print(a) print(b) end".to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("INDENT".to_string()),
            newline_format_if: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(
        ts(
            r#"if a > 3 then --[[3]] print(4) --[[4]] elseif --[[5]]a<3--[[6]] then --[[7]]print(2)--[[8]] elseif --[[9]]a == 3 --[[10]]then--[[11]] print(3)--[[12]] else--[[13]] print(0) --[[14]]end"#
        ),
        Ok(r#"if a > 3 then --[[3]]
INDENTprint(4) --[[4]]
elseif --[[5]]a<3--[[6]] then --[[7]]
INDENTprint(2)--[[8]]
elseif --[[9]]a == 3 --[[10]]then--[[11]]
INDENTprint(3)--[[12]]
else--[[13]]
INDENTprint(0) --[[14]]
end"#
            .to_string())
    );
    assert_eq!(
        ts(
            r#"if a > 3 then --[[3]] elseif --[[5]]a<3--[[6]] then --[[7]] elseif --[[9]]a == 3 --[[10]]then--[[11]] print(3)--[[12]] else--[[13]] print(0) --[[14]]end"#
        ),
        Ok(r#"if a > 3 then --[[3]]
elseif --[[5]]a<3--[[6]] then --[[7]]
elseif --[[9]]a == 3 --[[10]]then--[[11]]
INDENTprint(3)--[[12]]
else--[[13]]
INDENTprint(0) --[[14]]
end"#
            .to_string())
    );
    assert_eq!(
        ts(
            r#"if a > 3 then --[[3]] print(4) --[[4]] elseif --[[5]]a<3--[[6]] then --[[7]]print(2)--[[8]] elseif --[[9]]a == 3 --[[10]]then--[[11]] print(3)--[[12]] else--[[13]] end"#
        ),
        Ok(r#"if a > 3 then --[[3]]
INDENTprint(4) --[[4]]
elseif --[[5]]a<3--[[6]] then --[[7]]
INDENTprint(2)--[[8]]
elseif --[[9]]a == 3 --[[10]]then--[[11]]
INDENTprint(3)--[[12]]
else--[[13]]
end"#
            .to_string())
    );
    assert_eq!(
        ts(
            r#"if a > 3 then --[[3]] elseif --[[5]]a<3--[[6]] then --[[7]]print(2)--[[8]] elseif --[[9]]a == 3 --[[10]]then--[[11]] print(3)--[[12]] else--[[13]] end"#
        ),
        Ok(r#"if a > 3 then --[[3]]
elseif --[[5]]a<3--[[6]] then --[[7]]
INDENTprint(2)--[[8]]
elseif --[[9]]a == 3 --[[10]]then--[[11]]
INDENTprint(3)--[[12]]
else--[[13]]
end"#
            .to_string())
    );
    assert_eq!(
        ts(r#"if a > 3 then --[[3]] print(4) --[[4]] else--[[13]] print(0) --[[14]]end"#),
        Ok(r#"if a > 3 then --[[3]]
INDENTprint(4) --[[4]]
else--[[13]]
INDENTprint(0) --[[14]]
end"#
            .to_string())
    );
    assert_eq!(
        ts(r#"if a > 3 then --[[3]] else--[[13]] print(0) --[[14]]end"#),
        Ok(r#"if a > 3 then --[[3]] else--[[13]]
INDENTprint(0) --[[14]]
end"#
            .to_string())
    );
    assert_eq!(
        ts(r#"if a > 3 then --[[3]] print(4) --[[4]] else--[[13]] end"#),
        Ok(r#"if a > 3 then --[[3]]
INDENTprint(4) --[[4]]
else--[[13]]
end"#
            .to_string())
    );
    assert_eq!(
        ts(r#"if a > 3 then --[[3]] else--[[13]] end"#),
        Ok(r#"if a > 3 then --[[3]] else--[[13]]
end"#
            .to_string())
    );
    assert_eq!(
        ts(
            r#"if a > 3 then --[[3]] print(4) --[[4]] elseif --[[5]]a<3--[[6]] then --[[7]]print(2)--[[8]] elseif --[[9]]a == 3 --[[10]]then--[[11]] print(3)--[[12]] end"#
        ),
        Ok(r#"if a > 3 then --[[3]]
INDENTprint(4) --[[4]]
elseif --[[5]]a<3--[[6]] then --[[7]]
INDENTprint(2)--[[8]]
elseif --[[9]]a == 3 --[[10]]then--[[11]]
INDENTprint(3)--[[12]]
end"#
            .to_string())
    );
    assert_eq!(
        ts(
            r#"if a > 3 then --[[3]] elseif --[[5]]a<3--[[6]] then --[[7]]print(2)--[[8]] elseif --[[9]]a == 3 --[[10]]then--[[11]] print(3)--[[12]] end"#
        ),
        Ok(r#"if a > 3 then --[[3]]
elseif --[[5]]a<3--[[6]] then --[[7]]
INDENTprint(2)--[[8]]
elseif --[[9]]a == 3 --[[10]]then--[[11]]
INDENTprint(3)--[[12]]
end"#
            .to_string())
    );
    assert_eq!(
        ts(r#"if a > 3 then --[[3]] print(4) --[[4]] end"#),
        Ok(r#"if a > 3 then --[[3]]
INDENTprint(4) --[[4]]
end"#
            .to_string())
    );
    assert_eq!(
        ts(r#"if a > 3 then --[[3]] end"#),
        Ok(r#"if a > 3 then --[[3]]
end"#
            .to_string())
    );
}

#[test]
fn test_indent_function() {
    let cfg = Config::default();
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(
        ts("local function fn() --123\nprint(a) print(b) print(c) --[[345]] end"),
        Ok("local function fn() --123\nprint(a) print(b) print(c) --[[345]] end".to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("INDENT".to_string()),
            newline_format_function: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("local function fn() --123\nprint(a) print(b) print(c) --[[345]] end"),
        Ok(r#"local function fn() --123
INDENTprint(a) print(b) print(c) --[[345]]
end"#
            .to_string())
    );

    assert_eq!(
        ts("function fn() --[[123]]print(a)--a\n print(b) print(c) --345\n end"),
        Ok(r#"function fn() --[[123]]
INDENTprint(a)--a
 print(b) print(c) --345
end"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            newline_format_statement: Some(1),
            indentation_string: Some("INDENT".to_string()),
            newline_format_function: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("local function fn() --123\nprint(a) print(b) print(c) --[[345]] return 1 end"),
        Ok(r#"local function fn() --123
INDENTprint(a)
INDENTprint(b)
INDENTprint(c) --[[345]]
INDENTreturn 1
end"#
            .to_string())
    );

    assert_eq!(
        ts("function fn() --[[123]]print(a)--a\n print(b) print(c) --345\n return; end"),
        Ok(r#"function fn() --[[123]]
INDENTprint(a)--a
INDENTprint(b)
INDENTprint(c) --345
INDENTreturn;
end"#
            .to_string())
    );

    assert_eq!(
        ts(r#"-- function Value.new(value, compare_values, print_value)
Value = class(function(self, value, compare_values, print_value)
self._value_t = util.xtype(value)
self._value = value
self._compare_values = compare_values
self._print_value = print_value
end)
"#),
        Ok(r#"-- function Value.new(value, compare_values, print_value)
Value = class(function(self, value, compare_values, print_value)
INDENTself._value_t = util.xtype(value)
INDENTself._value = value
INDENTself._compare_values = compare_values
INDENTself._print_value = print_value
end)
"#
        .to_string())
    );
}

#[test]
fn test_indent_repeat_until() {
    let cfg = Config::default();
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(ts("repeat --abc\n until --[[123]] a>3"), Ok("repeat --abc\n until --[[123]] a>3".to_string()));
    assert_eq!(
        ts("repeat --abc\n print(a) print(b) print(c) --123\n until --[[123]] a>3"),
        Ok("repeat --abc\n print(a) print(b) print(c) --123\n until --[[123]] a>3".to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("INDENT".to_string()),
            newline_format_repeat_until: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("repeat --abc\n until --[[123]] a>3"),
        Ok(r#"repeat --abc
until --[[123]] a>3"#
            .to_string())
    );
    assert_eq!(
        ts("repeat --abc\n print(a) print(b) print(c) --123\n until --[[123]] a>3"),
        Ok(r#"repeat --abc
INDENTprint(a) print(b) print(c) --123
until --[[123]] a>3"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            newline_format_statement: Some(1),
            indentation_string: Some("INDENT".to_string()),
            newline_format_repeat_until: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(
        ts("repeat --abc\n print(a) print(b) print(c) --123\n until --[[123]] a>3"),
        Ok(r#"repeat --abc
INDENTprint(a)
INDENTprint(b)
INDENTprint(c) --123
until --[[123]] a>3"#
            .to_string())
    );
}

#[test]
fn test_indent_while_do() {
    let cfg = Config::default();
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(ts("while a < 3 --123\n do end"), Ok("while a < 3 --123\n do end".to_string()));
    assert_eq!(
        ts("while a < 3 --[[123]] do --234\n print(a) print(b) --[[345]] print(c) --456\n end"),
        Ok("while a < 3 --[[123]] do --234\n print(a) print(b) --[[345]] print(c) --456\n end".to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("INDENT".to_string()),
            newline_format_while: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(ts("while a < 3 --123\n do --234\n end"), Ok("while a < 3 --123\n do --234\nend".to_string()));
    assert_eq!(
        ts("while a < 3 --[[123]] do --234\n print(a) print(b) --[[345]] print(c) --456\n end"),
        Ok(r#"while a < 3 --[[123]] do --234
INDENTprint(a) print(b) --[[345]] print(c) --456
end"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            newline_format_statement: Some(1),
            indentation_string: Some("INDENT".to_string()),
            newline_format_while: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(
        ts("while a < 3 --[[123]] do --234\n print(a) print(b) --[[345]] print(c) --456\n end"),
        Ok(r#"while a < 3 --[[123]] do --234
INDENTprint(a)
INDENTprint(b) --[[345]]
INDENTprint(c) --456
end"#
            .to_string())
    );
}

#[test]
fn test_indent_table() {
    let cfg = Config::default();
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(ts("local a = {a=3, b=23-1, c=a}"), Ok("local a = {a=3, b=23-1, c=a}".to_string()));
    assert_eq!(
        ts("local a = { b = 123, c={1, 2, 3, {a=1, b=2}, 5}, d = {}, e}"),
        Ok("local a = { b = 123, c={1, 2, 3, {a=1, b=2}, 5}, d = {}, e}".to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            newline_format_table_constructor: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("local a = {a=3, b=23-1, c=a}"),
        Ok(r#"local a = {a=3, b=23-1, c=a
}"#
        .to_string())
    );
    assert_eq!(
        ts("local a = { b = 123, c={1, 2, 3, {a=1, b=2}, 5}, d = {}, e}"),
        Ok(r#"local a = { b = 123, c={1, 2, 3, {a=1, b=2
I   I   }, 5
I   }, d = {}, e
}"#
        .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            newline_format_table_field: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(ts("local a = {}"), Ok(r#"local a = {}"#.to_string()));
    assert_eq!(
        ts("local a = {a}"),
        Ok(r#"local a = {
I   a}"#
            .to_string())
    );
    assert_eq!(
        ts("local a = {a=3}"),
        Ok(r#"local a = {
I   a=3}"#
            .to_string())
    );
    assert_eq!(
        ts("local a = {a=3, b=23-1, c=a}"),
        Ok(r#"local a = {
I   a=3,
I   b=23-1,
I   c=a}"#
            .to_string())
    );
    assert_eq!(
        ts("local a = { b = 123, c={1, 2, 3, {a=1, b=2}, 5}, d = {}, e }"),
        Ok(r#"local a = {
I   b = 123,
I   c={
I   I   1,
I   I   2,
I   I   3,
I   I   {
I   I   I   a=1,
I   I   I   b=2},
I   I   5},
I   d = {},
I   e }"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            newline_format_table_constructor: Some(1),
            newline_format_table_field: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("local a = {a=3, b=23-1, c=a}"),
        Ok(r#"local a = {
I   a=3,
I   b=23-1,
I   c=a
}"#
        .to_string())
    );
    assert_eq!(
        ts("local a = { b = 123, c={1, 2, 3, {a=1, b=2}, 5}, d = {}, e}"),
        Ok(r#"local a = {
I   b = 123,
I   c={
I   I   1,
I   I   2,
I   I   3,
I   I   {
I   I   I   a=1,
I   I   I   b=2
I   I   },
I   I   5
I   },
I   d = {},
I   e
}"#
        .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            newline_format_table_constructor: Some(1),
            newline_format_statement: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"return string.format("{ type = \"%s\" }", self._value_t)"#),
        Ok(r#"return string.format("{ type = \"%s\" }", self._value_t)"#.to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            hint_table_constructor: Some(" ".to_string()),
            replace_zero_spaces_with_hint: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(ts("local a = {a=3, b=23-1, c=a}"), Ok("local a = { a = 3, b = 23 - 1, c = a }".to_string()));
    assert_eq!(
        ts("local a = { b = 123, c={1, 2, 3, {a=1, b=2}, 5}, d = {}, e}"),
        Ok("local a = { b = 123, c = { 1, 2, 3, { a = 1, b = 2 }, 5 }, d = { }, e }".to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            newline_format_table_constructor: Some(1),
            newline_format_table_field: Some(1),
            hint_table_constructor: Some(" ".to_string()),
            replace_zero_spaces_with_hint: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("local a = {a=3, b=23-1, c=a}"),
        Ok(r#"local a = {
I   a = 3,
I   b = 23 - 1,
I   c = a
}"#
        .to_string())
    );
    assert_eq!(
        ts("local a = { b = 123, c={1, 2, 3, {a=1, b=2}, 5}, d = {}, e}"),
        Ok(r#"local a = {
I   b = 123,
I   c = {
I   I   1,
I   I   2,
I   I   3,
I   I   {
I   I   I   a = 1,
I   I   I   b = 2
I   I   },
I   I   5
I   },
I   d = { },
I   e
}"#
        .to_string())
    );
}

#[test]
fn test_indent_all() {
    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I     ".to_string()),
            newline_format_statement: Some(1),
            newline_format_do_end: Some(1),
            newline_format_for: Some(1),
            newline_format_function: Some(1),
            newline_format_if: Some(1),
            newline_format_repeat_until: Some(1),
            newline_format_while: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("print(a) --123\n --1234\nprint(b) do print(c) --1\n  --2\n  --3\n while a<c do print(d) print(e) repeat print(a) until c<d --123123\nend --werewr\nprint(f) --3243\nend print(h)"),
        Ok(r#"print(a) --123
 --1234
print(b)
do
I     print(c) --1
  --2
  --3
I     while a<c do
I     I     print(d)
I     I     print(e)
I     I     repeat
I     I     I     print(a)
I     I     until c<d --123123
I     end --werewr
I     print(f) --3243
end
print(h)"#.to_string())
    );

    // oneline one
    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I     ".to_string()),
            newline_format_statement: Some(1),
            newline_format_oneline_comment: Some(1),
            newline_format_do_end: Some(1),
            newline_format_for: Some(1),
            newline_format_function: Some(1),
            newline_format_if: Some(1),
            newline_format_repeat_until: Some(1),
            newline_format_while: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("print(a) --123\n --1234\nprint(b) do\n--com\nprint(c) --1\n  --2\n  --3\n while a<c do\n--135\n print(d) print(e) repeat print(a) until c<d --123123\n --345\nend --werewr\nprint(f) --3243\nend print(h)"),
        Ok(r#"print(a) --123
--1234
print(b)
do
I     --com
I     print(c) --1
I     --2
I     --3
I     while a<c do
I     I     --135
I     I     print(d)
I     I     print(e)
I     I     repeat
I     I     I     print(a)
I     I     until c<d --123123
I     --345
I     end --werewr
I     print(f) --3243
end
print(h)"#.to_string())
    );

    // oneline both
    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I     ".to_string()),
            newline_format_statement: Some(1),
            newline_format_first_oneline_comment: Some(1),
            newline_format_oneline_comment: Some(1),
            newline_format_do_end: Some(1),
            newline_format_for: Some(1),
            newline_format_function: Some(1),
            newline_format_if: Some(1),
            newline_format_repeat_until: Some(1),
            newline_format_while: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("print(a) --123\n --1234\nprint(b) do print(c) --1\n  --2\n  --3\n while a<c do print(d) print(e) repeat print(a) until c<d --123123\n --345\nend --werewr\nprint(f) --3243\nend print(h)"),
        Ok(r#"print(a)
--123
--1234
print(b)
do
I     print(c)
I     --1
I     --2
I     --3
I     while a<c do
I     I     print(d)
I     I     print(e)
I     I     repeat
I     I     I     print(a)
I     I     until c<d
I     --123123
I     --345
I     end
I     --werewr
I     print(f)
--3243
end
print(h)"#.to_string())
    );

    assert_eq!(
        ts("#!/usr/bin/lua\n --123\nprint(a) --[[123]] --1234\nprint(b) do print(c) --[[0]]--1\n  --2\n  --3\n while a<c do print(d) print(e) repeat print(a) until c<d --123123\n --345\nend --werewr\nprint(f) --[[141242]] --3243\nend print(h)"),
        Ok(r#"#!/usr/bin/lua
--123
print(a) --[[123]]
--1234
print(b)
do
I     print(c) --[[0]]
I     --1
I     --2
I     --3
I     while a<c do
I     I     print(d)
I     I     print(e)
I     I     repeat
I     I     I     print(a)
I     I     until c<d
I     --123123
I     --345
I     end
I     --werewr
I     print(f) --[[141242]]
--3243
end
print(h)"#.to_string())
    );

    // multiline one
    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I     ".to_string()),
            newline_format_statement: Some(1),
            newline_format_multiline_comment: Some(1),
            newline_format_do_end: Some(1),
            newline_format_for: Some(1),
            newline_format_function: Some(1),
            newline_format_if: Some(1),
            newline_format_repeat_until: Some(1),
            newline_format_while: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("#!/usr/bin/lua\n --[[123]] --[[234]] --123\nprint(a) --[[123]] --1234\n   --[=[234]=]print(b) do print(c) --[[0]]--1\n  --[==[2]==]--[[3]]--4\n  --[[5]] while a<c do print(d) print(e) repeat print(a) until c<d --[[123123]] --[[345]]\nend --werewr\nprint(f) --[[141242]] --[[]] --3243\nend print(h)"),
        Ok(r#"#!/usr/bin/lua
 --[[123]]
--[[234]] --123
print(a) --[[123]] --1234
--[=[234]=]
print(b)
do
I     print(c) --[[0]]--1
I     --[==[2]==]
I     --[[3]]--4
I     --[[5]]
I     while a<c do
I     I     print(d)
I     I     print(e)
I     I     repeat
I     I     I     print(a)
I     I     until c<d --[[123123]]
I     --[[345]]
I     end --werewr
I     print(f) --[[141242]]
--[[]] --3243
end
print(h)"#.to_string())
    );

    // multiline both
    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I     ".to_string()),
            newline_format_statement: Some(1),
            newline_format_first_multiline_comment: Some(1),
            newline_format_multiline_comment: Some(1),
            newline_format_do_end: Some(1),
            newline_format_for: Some(1),
            newline_format_function: Some(1),
            newline_format_if: Some(1),
            newline_format_repeat_until: Some(1),
            newline_format_while: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("#!/usr/bin/lua\n --[[123]] --[[234]] --123\nprint(a) --[[123]] --1234\n   --[=[234]=]print(b) do print(c) --[[0]]--1\n  --[==[2]==]--[[3]]--4\n  --[[5]] while a<c do print(d) print(e) repeat print(a) until c<d --[[123123]] --[[345]]\nend --werewr\nprint(f) --[[141242]] --[[]] --3243\nend print(h)"),
        Ok(r#"#!/usr/bin/lua
--[[123]]
--[[234]] --123
print(a)
--[[123]] --1234
--[=[234]=]
print(b)
do
I     print(c)
I     --[[0]]--1
I     --[==[2]==]
I     --[[3]]--4
I     --[[5]]
I     while a<c do
I     I     print(d)
I     I     print(e)
I     I     repeat
I     I     I     print(a)
I     I     until c<d
I     --[[123123]]
I     --[[345]]
I     end --werewr
I     print(f)
--[[141242]]
--[[]] --3243
end
print(h)"#.to_string())
    );

    // multiline and oneline all
    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I     ".to_string()),
            newline_format_statement: Some(1),
            newline_format_first_oneline_comment: Some(1),
            newline_format_oneline_comment: Some(1),
            newline_format_first_multiline_comment: Some(1),
            newline_format_multiline_comment: Some(1),
            newline_format_do_end: Some(1),
            newline_format_for: Some(1),
            newline_format_function: Some(1),
            newline_format_if: Some(1),
            newline_format_repeat_until: Some(1),
            newline_format_while: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("#!/usr/bin/lua\n --[[123]] --[[234]] --123\nprint(a) --[[123]] --1234\n   --[=[234]=]print(b) do print(c) --[[0]]--1\n  --[==[2]==]--[[3]]--4\n  --[[5]] while a<c do print(d) print(e) repeat print(a) until c<d --[[123123]] --[[345]]\nend --werewr\nprint(f) --[[141242]] --[[]] --3243\nend print(h)"),
        Ok(r#"#!/usr/bin/lua
--[[123]]
--[[234]]
--123
print(a)
--[[123]]
--1234
--[=[234]=]
print(b)
do
I     print(c)
I     --[[0]]
I     --1
I     --[==[2]==]
I     --[[3]]
I     --4
I     --[[5]]
I     while a<c do
I     I     print(d)
I     I     print(e)
I     I     repeat
I     I     I     print(a)
I     I     until c<d
I     --[[123123]]
I     --[[345]]
I     end
I     --werewr
I     print(f)
--[[141242]]
--[[]]
--3243
end
print(h)"#.to_string())
    );
}

#[test]
fn test_indent_exp_list() {
    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            newline_format_statement: Some(1),
            newline_format_function: Some(1),

            indent_exp_list: Some(true),
            indent_var_suffix: Some(true),
            indent_one_line_exp_list: Some(true),
            newline_format_exp_list: Some(1),
            force_single_line_exp_list: Some(true),

            newline_format_var_suffix: Some(1),
            force_single_line_var_suffix: Some(true),
            max_width: Some(50),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"value = class(function()
    self.a = 1
    self.b = 2
end)
local a = b"#),
        Ok(r#"value = class(function()
I   I   I   self.a = 1
I   I   I   self.b = 2
I   I   end)
local a = b"#
            .to_string())
    );
    assert_eq!(
        ts(
            r#"for a, b, c in fn.field.field:method(), b.field.field:method():method().field, field.field:method() do end"#
        ),
        Ok(r#"for a, b, c in fn.field.field:method(),
I   b.field.field:method():method().field,
I   field.field:method() do end"#
            .to_string())
    );
    assert_eq!(
        ts(r#"return fn.field.field:method(), b.field.field:method():method().field, field.field:method()"#),
        Ok(r#"return fn.field.field:method(),
I   b.field.field:method():method().field,
I   field.field:method()"#
            .to_string())
    );
    assert_eq!(
        ts(r#"return fn.field.field:method(), b.field.field:method():method().field, field.field:method();"#),
        Ok(r#"return fn.field.field:method(),
I   b.field.field:method():method().field,
I   field.field:method();"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            newline_format_statement: Some(1),
            newline_format_function: Some(1),

            indent_exp_list: Some(true),
            indent_var_suffix: Some(true),
            // indent_one_line_exp_list: Some(true),
            newline_format_exp_list: Some(1),
            force_single_line_exp_list: Some(true),

            newline_format_var_suffix: Some(1),
            force_single_line_var_suffix: Some(true),
            max_width: Some(100),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"value = class(function()
    self.a = 1
    self.b = 2
end)
local a = b"#),
        Ok(r#"value = class(function()
I   self.a = 1
I   self.b = 2
end)
local a = b"#
            .to_string())
    );

    assert_eq!(
        ts(r#"value = class(vara, varb, function()
    self.a = 1
    self.b = 2
end)
local a = b"#),
        Ok(r#"value = class(vara, varb, function()
I   self.a = 1
I   self.b = 2
end)
local a = b"#
            .to_string())
    );

    assert_eq!(
        ts(r#"value = class(vara, varb, function()
    self.a = 1
    self.b = 2
end, function()
    self.a = 3
    self.b = 4
end, something_else)
local a = b"#),
        Ok(r#"value = class(vara, varb, function()
I   self.a = 1
I   self.b = 2
end, function()
I   self.a = 3
I   self.b = 4
end, something_else)
local a = b"#
            .to_string())
    );

    assert_eq!(
        ts(
            r#"value = class(vara, varb, varqweqweqweqweqwec, varqweqweqweqweqwec, varqweqweqweqweqwec, varqweqweqweqweqwec)
local a = b"#
        ),
        Ok(r#"value = class(vara, varb, varqweqweqweqweqwec, varqweqweqweqweqwec, varqweqweqweqweqwec,
I   varqweqweqweqweqwec)
local a = b"#
            .to_string())
    );

    assert_eq!(
        ts(r#"value = class(vara, varb, function()
    self.a = 1
    self.b = 2
end, varqweqweqweqweqwec, varqweqweqweqweqwec, varqweqweqweqweqwec, varqweqweqweqweqwec, varqweqweqweqweqwec, varqweqweqweqweqwec)
local a = b"#),
        Ok(r#"value = class(vara, varb, function()
I   I   self.a = 1
I   I   self.b = 2
I   end, varqweqweqweqweqwec, varqweqweqweqweqwec, varqweqweqweqweqwec, varqweqweqweqweqwec,
I   varqweqweqweqweqwec, varqweqweqweqweqwec)
local a = b"#
            .to_string())
    );
}
