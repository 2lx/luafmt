use super::common::*;
use crate::config::*;

#[test]
fn test_binary_ops() {
    let cfg = Config {
        fmt: FormatOpts {
            remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            newline_format_binary_op: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("if a.print(a, b) and b > 3 or fn(a, b, c) * 123 <= 1000 then end"),
        Ok(r#"if a.print(a, b)
I   I   and b
I   I   I   > 3
I   or fn(a, b, c)
I   I   I   * 123
I   I   <= 1000 then end"#
            .to_string())
    );
    assert_eq!(
        ts("local a = fn1(b, c, d) or fn2(c, d, e) or fn3(d, e, c) and c and d and e"),
        Ok("local a = fn1(b, c, d)
I   or fn2(c, d, e)
I   or fn3(d, e, c)
I   I   and c
I   I   and d
I   I   and e"
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            newline_format_binary_op: Some(2),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("if a.print(a, b) and b > 3 or fn(a, b, c) * 123 <= 1000 then end"),
        Ok(r#"if a.print(a, b) and
I   I   b >
I   I   I   3 or
I   fn(a, b, c) *
I   I   I   123 <=
I   I   1000 then end"#
            .to_string())
    );
    assert_eq!(
        ts("local a = fn1(b, c, d) or fn2(c, d, e) or fn3(d, e, c) and c and d and e"),
        Ok("local a = fn1(b, c, d) or
I   fn2(c, d, e) or
I   fn3(d, e, c) and
I   I   c and
I   I   d and
I   I   e"
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            newline_format_binary_op: Some(1),
            max_width: Some(30),
            force_single_line_binary_op: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("if a.print(a, b) and b > 3 or fn(a, b, c) * 123 <= 1000 then end"),
        Ok(r#"if a.print(a, b) and b > 3
I   or fn(a, b, c) * 123
I   I   <= 1000 then end"#
            .to_string())
    );
    assert_eq!(
        ts("local a = fn1(b, c, d) or fn2(c, d, e) or fn3(d, e, c) and c and d and e"),
        Ok("local a = fn1(b, c, d)
I   or fn2(c, d, e)
I   or fn3(d, e, c) and c
I   I   and d and e"
            .to_string())
    );
    assert_eq!(
        ts("local a = fn1(b, c, d) or fn2(c, d, e) or fn3(d, e, c) and (c and d and e)"),
        Ok("local a = fn1(b, c, d)
I   or fn2(c, d, e)
I   or fn3(d, e, c)
I   I   and (c and d and e)"
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            newline_format_binary_op: Some(1),
            max_width: Some(50),
            force_single_line_binary_op: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("if a.print(a, b) and b > 3 or fn(a, b, c) * 123 <= 1000 then end"),
        Ok(r#"if a.print(a, b) and b > 3
I   or fn(a, b, c) * 123 <= 1000 then end"#
            .to_string())
    );
    assert_eq!(
        ts("local a = fn1(b, c, d) or fn2(c, d, e) or fn3(d, e, c) and c and d and e"),
        Ok("local a = fn1(b, c, d) or fn2(c, d, e)
I   or fn3(d, e, c) and c and d and e"
            .to_string())
    );
    assert_eq!(
        ts("local a = fn1(b, c, d) or fn2(c, d, e) or fn3(d, e, c) and (c and d and e)"),
        Ok("local a = fn1(b, c, d) or fn2(c, d, e)
I   or fn3(d, e, c) and (c and d and e)"
            .to_string())
    );
}

#[test]
fn test_binary_op_same() {
    let cfg = Config {
        fmt: FormatOpts {
            // remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            newline_format_binary_op: Some(1),
            max_width: Some(50),
            force_single_line_binary_op: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("local a = 'abcdefg1' .. 'abcdefg2' .. 'abcdefg3' .. 'abcdefg4' .. 'abcdefg5' .. 'abcdefg6' .. 'abcdefg7'"),
        Ok(r#"local a = 'abcdefg1' .. 'abcdefg2' .. 'abcdefg3'
I   .. 'abcdefg4' .. 'abcdefg5' .. 'abcdefg6'
I   .. 'abcdefg7'"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            // remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            newline_format_binary_op: Some(1),
            max_width: Some(40),
            force_single_line_binary_op: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("local a = 'abcdefg1' .. 'abcdefg2' .. 'abcdefg3' .. 'abcdefg4' .. 'abcdefg5' .. 'abcdefg6' .. 'abcdefg7'"),
        Ok(r#"local a = 'abcdefg1' .. 'abcdefg2'
I   .. 'abcdefg3' .. 'abcdefg4'
I   .. 'abcdefg5' .. 'abcdefg6'
I   .. 'abcdefg7'"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            // remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            newline_format_binary_op: Some(1),
            max_width: Some(33),
            force_single_line_binary_op: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("local a = 'abcdefgh' .. 'abcdefgh' .. 'abcdefgh' .. 'abcdefgh' .. 'abcdefgh' .. 'abcdefgh' .. 'abcdefgh'"),
        Ok(r#"local a = 'abcdefgh'
I   .. 'abcdefgh' .. 'abcdefgh'
I   .. 'abcdefgh' .. 'abcdefgh'
I   .. 'abcdefgh' .. 'abcdefgh'"#
            .to_string())
    );
}

#[test]
fn test_table() {
    let cfg = Config {
        fmt: FormatOpts {
            remove_single_newlines: Some(true),
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
            remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            newline_format_table_constructor: Some(1),
            newline_format_table_field: Some(1),
            max_width: Some(40),
            force_single_line_table: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(ts("local a = {a=3, b=23-1, c=a}"), Ok(r#"local a = {a=3, b=23-1, c=a}"#.to_string()));
    assert_eq!(
        ts("local a = { b = 123, c={1, 2, 3, {a=1, b=2}, 5}, d = {}, e}"),
        Ok(r#"local a = {
I   b = 123,
I   c={1, 2, 3, {a=1, b=2}, 5},
I   d = {},
I   e
}"#
        .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            newline_format_table_constructor: Some(1),
            newline_format_table_field: Some(1),
            max_width: Some(55),
            force_single_line_table: Some(true),
            force_single_line_kv_table_field: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(ts("local a = {a=3, b=23-1, c=a}"), Ok(r#"local a = {a=3, b=23-1, c=a}"#.to_string()));
    assert_eq!(
        ts("local a = { b = 123, c={1, 2, 3, {a=1, b=2}, 5}, d = {}, e}"),
        Ok(r#"local a = { b = 123, c={1, 2, 3, {a=1, b=2}, 5}, d = {},
I   e
}"#
        .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            newline_format_table_constructor: Some(1),
            newline_format_table_field: Some(1),
            max_width: Some(27),
            force_single_line_table: Some(true),
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
I   I   {a=1, b=2},
I   I   5
I   },
I   d = {},
I   e
}"#
        .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            newline_format_table_constructor: Some(1),
            newline_format_table_field: Some(1),
            max_width: Some(15),
            force_single_line_table: Some(true),
            field_separator: Some(";".to_string()),
            write_trailing_field_separator: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("local a = {a=3, b=23-1, c=a}"),
        Ok(r#"local a = {
I   a=3;
I   b=23-1;
I   c=a;
}"#
        .to_string())
    );
    let cfg = Config {
        fmt: FormatOpts {
            remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            newline_format_table_constructor: Some(1),
            newline_format_table_field: Some(1),
            max_width: Some(27),
            force_single_line_table: Some(true),
            field_separator: Some(";".to_string()),
            write_trailing_field_separator: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("local a = {a=3, b=23-1, c=a}"),
        Ok(r#"local a = {
I   a=3;
I   b=23-1;
I   c=a;
}"#
        .to_string())
    );
    assert_eq!(
        ts("local a = { b = 123, c={1, 2, 3, {a=1, b=2}, 5}, d = {}, e}"),
        Ok(r#"local a = {
I   b = 123;
I   c={
I   I   1;
I   I   2;
I   I   3;
I   I   {a=1; b=2};
I   I   5;
I   };
I   d = {};
I   e;
}"#
        .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            newline_format_table_constructor: Some(1),
            newline_format_table_field: Some(1),
            force_single_line_table: Some(true),
            force_single_line_iv_table_field: Some(true),
            force_single_line_kv_table_field: Some(true),
            max_width: Some(28),
            field_separator: Some(";".to_string()),
            write_trailing_field_separator: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("local a = {a=3, b=23-1, c=a}"),
        Ok(r#"local a = {a=3; b=23-1; c=a;
}"#
        .to_string())
    );
    assert_eq!(
        ts("local a = { b = 123, c={1, 2, 3, {a=1, b=2}, 5}, d = {}, e}"),
        Ok(r#"local a = { b = 123;
I   c={
I   I   1; 2; 3; {a=1; b=2};
I   I   5;
I   }; d = {}; e;
}"#
        .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            // newline_format_table_constructor: Some(1),
            newline_format_table_field: Some(1),
            // force_single_line_table: Some(true),
            force_single_line_iv_table_field: Some(true),
            force_single_line_kv_table_field: Some(true),
            max_width: Some(29),
            field_separator: Some(";".to_string()),
            write_trailing_field_separator: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(ts("local a = {a=3, b=23-1, c=a}"), Ok(r#"local a = {a=3; b=23-1; c=a;}"#.to_string()));
    assert_eq!(
        ts("local a = { b = 123, c={1, 2, 3, {a=1, b=2}, 5}, d = {}, e}"),
        Ok(r#"local a = { b = 123;
I   c={1; 2; 3; {a=1; b=2;}; 5;
I   }; d = {}; e;}"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            // newline_format_table_constructor: Some(1),
            newline_format_table_field: Some(1),
            // force_single_line_table: Some(true),
            force_single_line_iv_table_field: Some(true),
            force_single_line_kv_table_field: Some(true),
            max_width: Some(35),
            field_separator: Some(";".to_string()),
            write_trailing_field_separator: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(ts("local a = {a=3, b=23-1, c=a}"), Ok(r#"local a = {a=3; b=23-1; c=a;}"#.to_string()));
    assert_eq!(
        ts("local a = { b = 123, c={1, 2, 3, {a=1, b=2}, 5}, d = {}, e}"),
        Ok(r#"local a = { b = 123;
I   c={1; 2; 3; {a=1; b=2;}; 5;};
I   d = {}; e;}"#
            .to_string())
    );
}

#[test]
fn test_if() {
    let cfg = Config {
        fmt: FormatOpts {
            remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            newline_format_if: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"if --[[1]] a > 3 --[[2]] then print(4) else print(0) --[[14]]end"#),
        Ok(r#"if --[[1]] a > 3 --[[2]] then
I   print(4)
else
I   print(0) --[[14]]
end"#
            .to_string())
    );

    assert_eq!(
        ts(
            r#"if --[[1]] a > 3 --[[2]] then --[[3]] print(4) --[[4]] elseif --[[5]]a<3--[[6]] then --[[7]]print(2)--[[8]]
elseif --[[9]]a == 3 --[[10]]then--[[11]] print(3)--[[12]] else--[[13]] print(0) --[[14]]end"#
        ),
        Ok(r#"if --[[1]] a > 3 --[[2]] then --[[3]]
I   print(4) --[[4]]
elseif --[[5]]a<3--[[6]] then --[[7]]
I   print(2)--[[8]]
elseif --[[9]]a == 3 --[[10]]then--[[11]]
I   print(3)--[[12]]
else--[[13]]
I   print(0) --[[14]]
end"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            newline_format_if: Some(1),
            max_width: Some(120),
            force_single_line_if: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"if --[[1]] a > 3 --[[2]] then print(4) else print(0) --[[14]]end"#),
        Ok(r#"if --[[1]] a > 3 --[[2]] then print(4) else print(0) --[[14]]end"#.to_string())
    );

    assert_eq!(
        ts(
            r#"if --[[1]] a > 3 --[[2]] then --[[3]] print(4) --[[4]] elseif --[[5]]a<3--[[6]] then --[[7]]print(2)--[[8]]
elseif --[[9]]a == 3 --[[10]]then--[[11]] print(3)--[[12]] else--[[13]] print(0) --[[14]]end"#
        ),
        Ok(r#"if --[[1]] a > 3 --[[2]] then --[[3]]
I   print(4) --[[4]]
elseif --[[5]]a<3--[[6]] then --[[7]]
I   print(2)--[[8]]
elseif --[[9]]a == 3 --[[10]]then--[[11]]
I   print(3)--[[12]]
else--[[13]]
I   print(0) --[[14]]
end"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            newline_format_if: Some(1),
            max_width: Some(20),
            force_single_line_if: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"if --[[1]] a > 3 --[[2]] then print(4) else print(0) --[[14]]end"#),
        Ok(r#"if --[[1]] a > 3 --[[2]] then
I   print(4)
else
I   print(0) --[[14]]
end"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            newline_format_if: Some(1),
            max_width: Some(2000),
            force_single_line_if: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"if --[[1]] a > 3 --[[2]] then print(4) else print(0) --[[14]]end"#),
        Ok(r#"if --[[1]] a > 3 --[[2]] then print(4) else print(0) --[[14]]end"#.to_string())
    );
    assert_eq!(
        ts(
            r#"if --[[1]] a > 3 --[[2]] then --[[3]] print(4) --[[4]] elseif --[[5]]a<3--[[6]] then --[[7]]print(2)--[[8]]
elseif --[[9]]a == 3 --[[10]]then--[[11]] print(3)--[[12]] else--[[13]] print(0) --[[14]]end"#
        ),
        Ok(r#"if --[[1]] a > 3 --[[2]] then --[[3]]
I   print(4) --[[4]]
elseif --[[5]]a<3--[[6]] then --[[7]]
I   print(2)--[[8]]
elseif --[[9]]a == 3 --[[10]]then--[[11]]
I   print(3)--[[12]]
else--[[13]]
I   print(0) --[[14]]
end"#
            .to_string())
    );
}

#[test]
fn test_function() {
    let cfg = Config {
        fmt: FormatOpts {
            remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            newline_format_function: Some(1),
            newline_format_statement: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"function fn(a, b) print(a) local fn2 = function(a, b) return a < b end print(b) end"#),
        Ok(r#"function fn(a, b)
I   print(a)
I   local fn2 = function(a, b)
I   I   return a < b
I   end
I   print(b)
end"#
            .to_string())
    );

    assert_eq!(
        ts(r#"function fn(a, b) return function(a, b) return a < b end end"#),
        Ok(r#"function fn(a, b)
I   return function(a, b)
I   I   return a < b
I   end
end"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            newline_format_function: Some(1),
            newline_format_statement: Some(1),
            max_width: Some(120),
            force_single_line_top_level_function: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"function fn(a, b) print(a) local fn2 = function(a, b) return a < b end print(b) end"#),
        Ok(r#"function fn(a, b)
I   print(a)
I   local fn2 = function(a, b)
I   I   return a < b
I   end
I   print(b)
end"#
            .to_string())
    );

    assert_eq!(
        ts(r#"function fn(a, b) return function(a, b) return a < b end end"#),
        Ok(r#"function fn(a, b) return function(a, b) return a < b end end"#.to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            newline_format_function: Some(1),
            newline_format_statement: Some(1),
            max_width: Some(120),
            force_single_line_scoped_function: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"function fn(a, b) print(a) local fn2 = function(a, b) return a < b end print(b) end"#),
        Ok(r#"function fn(a, b)
I   print(a)
I   local fn2 = function(a, b) return a < b end
I   print(b)
end"#
            .to_string())
    );

    assert_eq!(
        ts(r#"function fn(a, b) return function(a, b) return a < b end end"#),
        Ok(r#"function fn(a, b)
I   return function(a, b) return a < b end
end"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            remove_single_newlines: Some(true),
            indentation_string: Some("I   ".to_string()),
            newline_format_function: Some(1),
            newline_format_statement: Some(1),
            max_width: Some(120),
            force_single_line_top_level_function: Some(true),
            force_single_line_scoped_function: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"function fn(a, b) print(a) local fn2 = function(a, b) return a < b end print(b) end"#),
        Ok(r#"function fn(a, b)
I   print(a)
I   local fn2 = function(a, b) return a < b end
I   print(b)
end"#
            .to_string())
    );

    assert_eq!(
        ts(r#"function fn(a, b) return function(a, b) return a < b end end"#),
        Ok(r#"function fn(a, b) return function(a, b) return a < b end end"#.to_string())
    );

    assert_eq!(
        ts(r#"function fn(a, b) print(a) print(b) return function(a, b) return a < b end end"#),
        Ok(r#"function fn(a, b)
I   print(a)
I   print(b)
I   return function(a, b) return a < b end
end"#
            .to_string())
    );
}

#[test]
fn test_table_suffix() {
    let cfg =
        Config { fmt: FormatOpts { newline_format_var_suffix: Some(1), ..FormatOpts::default() }, ..Config::default() };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"object.field.field:method().field:method():method().field.field:method():method()"#),
        Ok(r#"object
.field
.field
:method()
.field
:method()
:method()
.field
.field
:method()
:method()"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            max_width: Some(20),
            newline_format_var_suffix: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"object.field.field:method().field:method():method().field.field:method():method()"#),
        Ok(r#"object
.field
.field
:method()
.field
:method()
:method()
.field
.field
:method()
:method()"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            max_width: Some(20),
            newline_format_var_suffix: Some(1),
            force_single_line_var_suffix: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"object.field.field:method().field:method():method().field.field:method():method()"#),
        Ok(r#"object.field.field
:method().field
:method():method()
.field.field:method()
:method()"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            max_width: Some(24),
            newline_format_var_suffix: Some(1),
            force_single_line_var_suffix: Some(true),
            indent_var_suffix: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"object.field.field:method().field:method():method().field.field:method():method()"#),
        Ok(r#"object.field.field
I   :method().field
I   :method():method()
I   .field.field:method()
I   :method()"#
            .to_string())
    );
}

#[test]
fn test_table_field() {
    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            max_width: Some(24),
            newline_format_var_suffix: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"object.field.field:method().field:method():method().field.field:method():method()"#),
        Ok(r#"object
.field
.field
:method()
.field
:method()
:method()
.field
.field
:method()
:method()"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            max_width: Some(24),
            newline_format_var_suffix: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"object.field.field:method().field:method():method().field.field:method():method()"#),
        Ok(r#"object
.field
.field
:method()
.field
:method()
:method()
.field
.field
:method()
:method()"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            max_width: Some(24),
            newline_format_var_suffix: Some(1),
            indent_var_suffix: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"object.field.field:method().field:method():method().field.field:method():method()"#),
        Ok(r#"object
I   .field
I   .field
I   :method()
I   .field
I   :method()
I   :method()
I   .field
I   .field
I   :method()
I   :method()"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            max_width: Some(24),
            newline_format_var_suffix: Some(1),
            force_single_line_var_suffix: Some(true),
            indent_var_suffix: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"object.field.field:method().field:method():method().field.field:method():method()"#),
        Ok(r#"object.field.field
I   :method().field
I   :method():method()
I   .field.field:method()
I   :method()"#
            .to_string())
    );

    assert_eq!(
        ts(r#"object:method(object2.field:method(object3.field:method(object4)))"#),
        Ok(r#"object
I   :method(object2
I   I   .field
I   I   :method(object3
I   I   I   .field
I   I   I   :method(object4)))"#
            .to_string())
    );

    assert_eq!(
        ts(
            r#"object:method1(object1.field1.field2):method2(object2):method3():method4(object4:method(object5.field))"#
        ),
        Ok(r#"object
I   :method1(object1
I   I   .field1.field2)
I   :method2(object2)
I   :method3()
I   :method4(object4
I   I   :method(object5
I   I   I   .field))"#
            .to_string())
    );

    assert_eq!(
        ts(r#"object:method1(object1):method2(object2):method3(object3.field31.field32):method4(object4:method41())"#),
        Ok(r#"object:method1(object1)
I   :method2(object2)
I   :method3(object3
I   I   .field31
I   I   .field32)
I   :method4(object4
I   I   :method41())"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            max_width: Some(24),
            newline_format_var_suffix: Some(1),
            force_single_line_var_suffix: Some(true),
            indent_var_suffix: Some(true),
            indent_exp_list: Some(true),
            newline_format_exp_list_first: Some(1),
            force_single_line_exp_list: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"object:method(object2.field:method(object3.field:method(object4)))"#),
        Ok(r#"object:method(
I   object2.field
I   I   :method(
I   I   I   object3
I   I   I   I   .field
I   I   I   I   :method(
I   I   I   I   I   object4)))"#
            .to_string())
    );

    assert_eq!(
        ts(
            r#"object:method1(object1.field1.field2):method2(object2):method3():method4(object4:method(object5.field))"#
        ),
        Ok(r#"object:method1(
I   I   object1.field1
I   I   I   .field2)
I   :method2(object2)
I   :method3():method4(
I   I   object4:method(
I   I   I   object5
I   I   I   I   .field))"#
            .to_string())
    );

    assert_eq!(
        ts(r#"object:method1(object1):method2(object2):method3(object3.field31.field32):method4(object4:method41())"#),
        Ok(r#"object:method1(object1)
I   :method2(object2)
I   :method3(
I   I   object3.field31
I   I   I   .field32)
I   :method4(
I   I   object4
I   I   I   :method41())"#
            .to_string())
    );
}

#[test]
fn test_exp_list() {
    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            max_width: Some(24),
            newline_format_exp_list: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"local a = b
fn(12, "abc", a)"#),
        Ok(r#"local a = b
fn(12,
"abc",
a)"#
        .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            max_width: Some(24),
            newline_format_exp_list: Some(1),
            newline_format_exp_list_first: Some(1),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"local a = b
fn(12, "abc", a)"#),
        Ok(r#"local a =
b
fn(
12,
"abc",
a)"#
        .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            max_width: Some(24),
            newline_format_exp_list: Some(1),
            force_single_line_exp_list: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"local a = b
fn(12, "abc", a)"#),
        Ok(r#"local a = b
fn(12, "abc", a)"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            max_width: Some(24),
            newline_format_exp_list: Some(1),
            newline_format_exp_list_first: Some(1),
            force_single_line_exp_list: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"local a = b
fn(12, "abc", a)"#),
        Ok(r#"local a = b
fn(12, "abc", a)"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            max_width: Some(24),
            newline_format_exp_list: Some(1),
            force_single_line_exp_list: Some(true),
            indent_exp_list: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"local a = b, "asdasdsad", 12312321
fn(12, "abc", abvad, {a=12321, b="asdad"})"#),
        Ok(r#"local a = b, "asdasdsad",
I   12312321
fn(12, "abc", abvad,
I   {a=12321, b="asdad"})"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            max_width: Some(24),
            newline_format_exp_list: Some(1),
            newline_format_exp_list_first: Some(1),
            force_single_line_exp_list: Some(true),
            indent_exp_list: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"local a = b, "asdasdsad", 12312321
fn(12, "abc", abvad, {a=12321, b="asdad"})"#),
        Ok(r#"local a = b, "asdasdsad",
I   12312321
fn(12, "abc", abvad,
I   {a=12321, b="asdad"})"#
            .to_string())
    );
    assert_eq!(
        ts(r#"local asdadasdasdasdaasd = b, "asdasdsad", 12312321
fn(12, "abc", abvad, {a=12321, b="asdad"})"#),
        Ok(r#"local asdadasdasdasdaasd =
I   b, "asdasdsad",
I   12312321
fn(12, "abc", abvad,
I   {a=12321, b="asdad"})"#
            .to_string())
    );

    let cfg = Config {
        fmt: FormatOpts {
            indentation_string: Some("I   ".to_string()),
            max_width: Some(26),
            newline_format_exp_list: Some(1),
            newline_format_function: Some(1),
            newline_format_statement: Some(1),
            force_single_line_exp_list: Some(true),
            remove_single_newlines: Some(true),
            indent_exp_list: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"a = fn("str", function(a) print(a) print(a + 1) end, 123, function(b) print(b) print(b + 1) end)"#),
        Ok(r#"a = fn("str", function(a)
I   print(a)
I   print(a + 1)
end, 123, function(b)
I   print(b)
I   print(b + 1)
end)"#
            .to_string())
    );
}
