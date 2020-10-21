use super::common::*;
use crate::config::*;

#[test]
fn test_binary_ops() {
    let cfg = Config {
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        binary_op_indent_format: Some(1),
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
I   I   or fn2(c, d, e)
I   or fn3(d, e, c)
I   I   I   I   and c
I   I   I   and d
I   I   and e"
            .to_string())
    );

    let cfg = Config {
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        binary_op_indent_format: Some(2),
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
I   I   fn2(c, d, e) or
I   fn3(d, e, c) and
I   I   I   I   c and
I   I   I   d and
I   I   e"
            .to_string())
    );

    let cfg = Config {
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        binary_op_indent_format: Some(1),
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
I   I   or fn2(c, d, e)
I   or fn3(d, e, c) and c
I   I   I   and d
I   I   and e"
            .to_string())
    );
    assert_eq!(
        ts("local a = fn1(b, c, d) or fn2(c, d, e) or fn3(d, e, c) and (c and d and e)"),
        Ok("local a = fn1(b, c, d)
I   I   or fn2(c, d, e)
I   or fn3(d, e, c)
I   I   and (c and d and e)"
            .to_string())
    );

    let cfg = Config {
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        binary_op_indent_format: Some(1),
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
fn test_table() {
    let cfg = Config {
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        table_indent_format: Some(1),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(ts("local a = {a=3, b=23-1, c=a}"), Ok(r#"local a = {
I   a=3,
I   b=23-1,
I   c=a
}"#.to_string()));
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
}"#.to_string())
    );

    let cfg = Config {
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        table_indent_format: Some(1),
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
}"#.to_string())
    );

    let cfg = Config {
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        table_indent_format: Some(1),
        max_width: Some(27),
        enable_oneline_table: Some(true),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(ts("local a = {a=3, b=23-1, c=a}"), Ok(r#"local a = {
I   a=3,
I   b=23-1,
I   c=a
}"#.to_string()));
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
}"#.to_string())
    );

    let cfg = Config {
        remove_single_newlines: Some(true),
        indentation_string: Some("I   ".to_string()),
        table_indent_format: Some(1),
        max_width: Some(27),
        enable_oneline_table: Some(true),
        field_separator: Some(";".to_string()),
        write_trailing_field_separator: Some(true),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(ts("local a = {a=3, b=23-1, c=a}"), Ok(r#"local a = {
I   a=3;
I   b=23-1;
I   c=a;
}"#.to_string()));
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
}"#.to_string())
    );
}
