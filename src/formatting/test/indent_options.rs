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
        Config { indentation_string: Some("    ".to_string()), do_end_indent_format: Some(1), ..Config::default() };
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(
        ts("do print(a) print(b) end"),
        Ok(r#"do
    print(a) print(b)
end"#
            .to_string())
    );

    assert_eq!(
        ts("do--comm\n   print(a) print(b) --[[123]]end"),
        Ok(r#"do--comm
    print(a) print(b) --[[123]]
end"#
            .to_string())
    );

    assert_eq!(
        ts(r#"do--comm
    print(a)
    print(b) --[[123]]
    end"#),
        Ok(r#"do--comm
    print(a)
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
    print(a)
    print(b) --[[123]]
    --123
    --[[345]]
end"#
            .to_string())
    );
}

#[test]
fn test_indent_every_statement() {
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
        indentation_string: Some("INDENT".to_string()),
        indent_every_statement: Some(true),
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

    let cfg =
        Config { indentation_string: Some("INDENT".to_string()), for_indent_format: Some(1), ..Config::default() };
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
        indentation_string: Some("INDENT".to_string()),
        for_indent_format: Some(1),
        indent_every_statement: Some(true),
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

    let cfg = Config { indentation_string: Some("INDENT".to_string()), if_indent_format: Some(1), ..Config::default() };
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

    let cfg =
        Config { indentation_string: Some("INDENT".to_string()), function_indent_format: Some(1), ..Config::default() };
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
        indent_every_statement: Some(true),
        indentation_string: Some("INDENT".to_string()),
        function_indent_format: Some(1),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("local function fn() --123\nprint(a) print(b) print(c) --[[345]] end"),
        Ok(r#"local function fn() --123
INDENTprint(a)
INDENTprint(b)
INDENTprint(c) --[[345]]
end"#
            .to_string())
    );

    assert_eq!(
        ts("function fn() --[[123]]print(a)--a\n print(b) print(c) --345\n end"),
        Ok(r#"function fn() --[[123]]
INDENTprint(a)--a
INDENTprint(b)
INDENTprint(c) --345
end"#
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
        indentation_string: Some("INDENT".to_string()),
        repeat_until_indent_format: Some(1),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("repeat --abc\n until --[[123]] a>3"),
        Ok(r#"repeat --abc
until --[[123]] a>3"#.to_string()));
    assert_eq!(
        ts("repeat --abc\n print(a) print(b) print(c) --123\n until --[[123]] a>3"),
        Ok(r#"repeat --abc
INDENTprint(a) print(b) print(c) --123
until --[[123]] a>3"#.to_string())
    );

    let cfg = Config {
        indent_every_statement: Some(true),
        indentation_string: Some("INDENT".to_string()),
        repeat_until_indent_format: Some(1),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(
        ts("repeat --abc\n print(a) print(b) print(c) --123\n until --[[123]] a>3"),
        Ok(r#"repeat --abc
INDENTprint(a)
INDENTprint(b)
INDENTprint(c) --123
until --[[123]] a>3"#.to_string())
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
        indentation_string: Some("INDENT".to_string()),
        while_do_indent_format: Some(1),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("while a < 3 --123\n do --234\n end"),
        Ok("while a < 3 --123\n do --234\nend".to_string())
    );
    assert_eq!(
        ts("while a < 3 --[[123]] do --234\n print(a) print(b) --[[345]] print(c) --456\n end"),
        Ok(r#"while a < 3 --[[123]] do --234
INDENTprint(a) print(b) --[[345]] print(c) --456
end"#.to_string())
    );

    let cfg = Config {
        indent_every_statement: Some(true),
        indentation_string: Some("INDENT".to_string()),
        while_do_indent_format: Some(1),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(
        ts("while a < 3 --[[123]] do --234\n print(a) print(b) --[[345]] print(c) --456\n end"),
        Ok(r#"while a < 3 --[[123]] do --234
INDENTprint(a)
INDENTprint(b) --[[345]]
INDENTprint(c) --456
end"#.to_string())
    );
}
