use super::common::*;
use crate::config::*;

#[test]
fn test_binary_ops() {
    let cfg = Config {
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        format_type_binary_op: Some(1),
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
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        format_type_binary_op: Some(2),
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
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        format_type_binary_op: Some(1),
        max_width: Some(30),
        enable_oneline_binary_op: Some(true),
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
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        format_type_binary_op: Some(1),
        max_width: Some(50),
        enable_oneline_binary_op: Some(true),
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
        // remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        format_type_binary_op: Some(1),
        max_width: Some(50),
        enable_oneline_binary_op: Some(true),
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
        // remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        format_type_binary_op: Some(1),
        max_width: Some(40),
        enable_oneline_binary_op: Some(true),
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
        // remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        format_type_binary_op: Some(1),
        max_width: Some(33),
        enable_oneline_binary_op: Some(true),
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
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        format_type_table: Some(1),
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
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        format_type_table: Some(1),
        max_width: Some(50),
        enable_oneline_table: Some(true),
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
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        format_type_table: Some(1),
        max_width: Some(27),
        enable_oneline_table: Some(true),
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
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        format_type_table: Some(1),
        max_width: Some(27),
        enable_oneline_table: Some(true),
        field_separator: Some(";".to_string()),
        write_trailing_field_separator: Some(true),
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
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        format_type_table: Some(2),
        max_width: Some(27),
        field_separator: Some(";".to_string()),
        write_trailing_field_separator: Some(true),
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
I   c={1; 2; 3; {a=1; b=2;};
I   I   5;}; d = {}; e;}"#
        .to_string())
    );
}

#[test]
fn test_if() {
    let cfg = Config {
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        format_type_if: Some(1),
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
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        format_type_if: Some(1),
        max_width: Some(120),
        enable_oneline_if: Some(true),
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
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        format_type_if: Some(1),
        max_width: Some(20),
        enable_oneline_if: Some(true),
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
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        format_type_if: Some(1),
        max_width: Some(2000),
        enable_oneline_if: Some(true),
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
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        format_type_function: Some(1),
        indent_every_statement: Some(true),
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
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        format_type_function: Some(1),
        indent_every_statement: Some(true),
        max_width: Some(120),
        enable_oneline_top_level_function: Some(true),
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
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        format_type_function: Some(1),
        indent_every_statement: Some(true),
        max_width: Some(120),
        enable_oneline_scoped_function: Some(true),
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
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        format_type_function: Some(1),
        indent_every_statement: Some(true),
        max_width: Some(120),
        enable_oneline_top_level_function: Some(true),
        enable_oneline_scoped_function: Some(true),
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
fn test_method_call() {
    let cfg = Config {
        // indentation_string: Some("I   ".to_string()),
        // max_width: Some(20),
        format_type_method_call: Some(1),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"object.field.field:method().field:method():method().field.field:method():method()"#),
        Ok(r#"object.field.field
:method().field
:method()
:method().field.field
:method()
:method()"#.to_string())
    );

    let cfg = Config {
        indentation_string: Some("I   ".to_string()),
        max_width: Some(20),
        format_type_method_call: Some(1),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"object.field.field:method().field:method():method().field.field:method():method()"#),
        Ok(r#"object.field.field
:method().field
:method()
:method().field.field
:method()
:method()"#.to_string())
    );

    let cfg = Config {
        indentation_string: Some("I   ".to_string()),
        max_width: Some(20),
        format_type_method_call: Some(1),
        enable_oneline_method_call: Some(true),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"object.field.field:method().field:method():method().field.field:method():method()"#),
        Ok(r#"object.field.field
:method().field
:method():method().field.field
:method():method()"#.to_string())
    );

    let cfg = Config {
        indentation_string: Some("I   ".to_string()),
        max_width: Some(24),
        format_type_method_call: Some(1),
        enable_oneline_method_call: Some(true),
        indent_method_call: Some(true),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"object.field.field:method().field:method():method().field.field:method():method()"#),
        Ok(r#"object.field.field
I   :method().field
I   :method():method().field.field
I   :method():method()"#.to_string())
    );
}

#[test]
fn test_table_field() {
    let cfg = Config {
        indentation_string: Some("I   ".to_string()),
        max_width: Some(24),
        format_type_table_field: Some(1),
        // enable_oneline_talbe_field: Some(true),
        // indent_table_field: Some(true),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"object.field.field:method().field:method():method().field.field:method():method()"#),
        Ok(r#"object
.field
.field:method()
.field:method():method()
.field
.field:method():method()"#.to_string())
    );

    let cfg = Config {
        indentation_string: Some("I   ".to_string()),
        max_width: Some(24),
        format_type_table_field: Some(1),
        format_type_method_call: Some(1),
        // enable_oneline_talbe_field: Some(true),
        // indent_table_field: Some(true),
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
:method()"#.to_string())
    );

    let cfg = Config {
        indentation_string: Some("I   ".to_string()),
        max_width: Some(24),
        format_type_table_field: Some(1),
        // format_type_method_call: Some(1),
        // enable_oneline_talbe_field: Some(true),
        indent_table_field: Some(true),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"object.field.field:method().field:method():method().field.field:method():method()"#),
        Ok(r#"object
I   .field
I   .field:method()
I   .field:method():method()
I   .field
I   .field:method():method()"#.to_string())
    );

    let cfg = Config {
        indentation_string: Some("I   ".to_string()),
        max_width: Some(24),
        format_type_table_field: Some(1),
        format_type_method_call: Some(1),
        // enable_oneline_talbe_field: Some(true),
        indent_table_field: Some(true),
        indent_method_call: Some(true),
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
I   :method()"#.to_string())
    );

    let cfg = Config {
        indentation_string: Some("I   ".to_string()),
        max_width: Some(24),
        format_type_table_field: Some(1),
        format_type_method_call: Some(1),
        enable_oneline_table_field: Some(true),
        indent_table_field: Some(true),
        indent_method_call: Some(true),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"object.field.field:method().field:method():method().field.field:method():method()"#),
        Ok(r#"object.field.field
I   :method().field
I   :method()
I   :method().field
I   .field
I   :method()
I   :method()"#.to_string())
    );

    let cfg = Config {
        indentation_string: Some("I   ".to_string()),
        max_width: Some(24),
        format_type_table_field: Some(1),
        format_type_method_call: Some(1),
        enable_oneline_table_field: Some(true),
        enable_oneline_method_call: Some(true),
        indent_table_field: Some(true),
        indent_method_call: Some(true),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"object.field.field:method().field:method():method().field.field:method():method()"#),
        Ok(r#"object.field.field
I   :method().field
I   :method():method()
I   .field.field
I   :method():method()"#.to_string())
    );
}
