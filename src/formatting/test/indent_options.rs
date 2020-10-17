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

    let cfg =
        Config { indentation_string: Some("    ".to_string()), do_end_format: Some(1), ..Config::default() };
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(ts("do print(a) print(b) end"), Ok(r#"do
    print(a) print(b)
end"#.to_string()));

    assert_eq!(ts("do--comm\n   print(a) print(b) --[[123]]end"), Ok(r#"do--comm
    print(a) print(b) --[[123]]
end"#.to_string()));

    assert_eq!(ts(r#"do--comm
    print(a)
    print(b) --[[123]]
    end"#),
    Ok(r#"do--comm
    print(a)
    print(b) --[[123]]
end"#.to_string()));

    assert_eq!(ts(r#"do --[[123]] print(a)
    print(b) --[[123]]
    --123
    --[[345]]end"#),
    Ok(r#"do --[[123]]
    print(a)
    print(b) --[[123]]
    --123
    --[[345]]
end"#.to_string()));
}

#[test]
fn test_indent_every_statement() {
    let cfg = Config::default();
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(ts("do print(a) print(b) print(c) end"), Ok("do print(a) print(b) print(c) end".to_string()));
    assert_eq!(ts("for i = 1, 3 do --[[1]] print(a) --2\n print(b) --3\n print(c) --[[4]] end"),
               Ok("for i = 1, 3 do --[[1]] print(a) --2\n print(b) --3\n print(c) --[[4]] end".to_string()));
    assert_eq!(ts("while a > 4 do --[[1]] print(a) --2\n print(b) --3\n print(c) --[[4]] end"),
               Ok("while a > 4 do --[[1]] print(a) --2\n print(b) --3\n print(c) --[[4]] end".to_string()));

    let cfg =
        Config { indent_every_statement: Some(true), ..Config::default() };
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(ts("do print(a) print(b) print(c) end"), Ok(r#"do print(a)
print(b)
print(c) end"#.to_string()));
    assert_eq!(ts("for i = 1, 3 do --[[1]] print(a) --2\n print(b) --[[3]] print(c) --[[4]] end"),
               Ok("for i = 1, 3 do --[[1]] print(a) --2
print(b) --[[3]]
print(c) --[[4]] end".to_string()));
    assert_eq!(ts("while a > 4 do --[[1]] print(a) --2\n print(b) --3\n print(c) --[[4]] end"),
               Ok("while a > 4 do --[[1]] print(a) --2
print(b) --3
print(c) --[[4]] end".to_string()));
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

    let cfg =
        Config { indentation_string: Some("    ".to_string()), if_then_else_format: Some(1), ..Config::default() };
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(ts("if a > b then print(a) print(b) end"), Ok("if a > b then print(a) print(b) end".to_string()));
    assert_eq!(
        ts("if a > b --comment\n then print(a) print(b) end"),
        Ok("if a > b --comment\n then print(a) print(b) end".to_string())
    );
}
