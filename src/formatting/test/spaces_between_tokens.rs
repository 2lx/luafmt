use super::common::*;
use crate::config::*;

#[test]
fn test_spaces_between_tokens_ops() {
    let cfg = Config {
        fmt: FormatOpts {
            replace_zero_spaces_with_hint: Some(true),
            remove_spaces_between_tokens: Some(true),
            hint_before_comment: Some(" ".to_string()),
            hint_after_multiline_comment: Some(" ".to_string()),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    // binary ops
    for op in vec![
        "+", "-", "or", "and", "==", "~=", ">=", "<=", "<", ">", "|", "~", "&", ">>", "<<", "..", "*", "/", "//", "%",
        "^",
    ] {
        let left = format!("c   --1\n   --1.3\n =  --[=[2]=]   a  --3\n  {}   --[[4]]   b", op);
        let right = format!("c --1\n --1.3\n= --[=[2]=] a --3\n{} --[[4]] b", op);
        assert_eq!(ts(&left), Ok(right));

        let left = format!("c = a--\n{} --[[342]]b", op);
        let right = format!("c = a --\n{} --[[342]] b", op);
        assert_eq!(ts(&left), Ok(right));
    }

    // unary ops
    for op in vec!["not", "-", "#", "~"] {
        let left = format!("c--[=[1]=]=--2\n{} --3\nb", op);
        let right = format!("c --[=[1]=] = --2\n{} --3\nb", op);
        assert_eq!(ts(&left), Ok(right));

        let left = format!("c   --1\n  =  --[[2]]   {}  --3\n  b", op);
        let right = format!("c --1\n= --[[2]] {} --3\nb", op);
        assert_eq!(ts(&left), Ok(right));
    }
}

#[test]
fn test_spaces_between_tokens_other() {
    let cfg = Config {
        fmt: FormatOpts {
            replace_zero_spaces_with_hint: Some(true),
            remove_spaces_between_tokens: Some(true),
            hint_before_comment: Some(" ".to_string()),
            hint_after_multiline_comment: Some(" ".to_string()),
            hint_table_constructor: Some(" ".to_string()),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &'static str| ts_base(s, &cfg);

    // TableConstructor
    assert_eq!(ts("t={--\n}"), Ok("t = { --\n}".to_string()));
    assert_eq!(ts("t = { a --\n  =  --[[]]  3}"), Ok("t = { a --\n= --[[]] 3 }".to_string()));
    assert_eq!(
        ts("t = { [ --c1\n a --[[c2]]] --c3\n= --c4\n 3}"),
        Ok("t = { [ --c1\na --[[c2]] ] --c3\n= --c4\n3 }".to_string())
    );
    assert_eq!(
        ts("t = { [ --c1\n 'a' --[[c2]]] --c3\n= --c4\n 3}"),
        Ok("t = { [ --c1\n'a' --[[c2]] ] --c3\n= --c4\n3 }".to_string())
    );
    assert_eq!(
        ts("t = { [ --c1\n \"a\" --[[c2]]] --c3\n= --c4\n 3}"),
        Ok("t = { [ --c1\n\"a\" --[[c2]] ] --c3\n= --c4\n3 }".to_string())
    );
    assert_eq!(
        ts("t = { [ --c1\n [[a]] --[[c2]]] --c3\n= --c4\n 3}"),
        Ok("t = { [ --c1\n[[a]] --[[c2]] ] --c3\n= --c4\n3 }".to_string())
    );
    assert_eq!(
        ts("t = { --0\n a = 1 --1\n, --2\n b = 2 --3\n, --4\n c = 3 --5\n, --6\n d = 4 --7\n, --8\n e = 5 --9\n, --10\n }"),
        Ok("t = { --0\na = 1 --1\n, --2\nb = 2 --3\n, --4\nc = 3 --5\n, --6\nd = 4 --7\n, --8\ne = 5 --9\n, --10\n}".to_string())
    );

    // FunctionDef
    assert_eq!(
        ts("fn = function --1\n( --[[2]]a --3\n  , --4\n b--[[5]] ,--6\n c --[[7]])--8\nend"),
        Ok("fn = function --1\n( --[[2]] a --3\n, --4\nb --[[5]] , --6\nc --[[7]] ) --8\nend".to_string())
    );
    assert_eq!(
        ts("fn = function --1\n( --[==[2]==]a --3\n  , --4\n b--[[5]] ,--6\n c --[[7]])--[=[8]=]print(a) --[[9]]end"),
        Ok("fn = function --1\n( --[==[2]==] a --3\n, --4\nb --[[5]] , --6\nc --[[7]] ) --[=[8]=] print(a) --[[9]] end".to_string())
    );

    // FunctionCall
    assert_eq!(ts("local a = fn--[[1]](--[[2]])"), Ok("local a = fn --[[1]] ( --[[2]] )".to_string()));
    assert_eq!(
        ts("local a = fn--[[1]](--2\na --[[3]],--[[4]]b--5\n, --6\nc--[[7]])"),
        Ok("local a = fn --[[1]] ( --2\na --[[3]] , --[[4]] b --5\n, --6\nc --[[7]] )".to_string())
    );
    assert_eq!(ts("local a = (--1\nfn--[[2]])(--[[3]])"), Ok("local a = ( --1\nfn --[[2]] )( --[[3]] )".to_string()));
    assert_eq!(
        ts("local a = (--1\nfn--[[2]])(  --3\n a --[[4]])"),
        Ok("local a = ( --1\nfn --[[2]] )( --3\na --[[4]] )".to_string())
    );
    assert_eq!(
        ts("local a = (--1\nfn--[[2]]  (--5\n  ))(  --3\n a --[[4]])"),
        Ok("local a = ( --1\nfn --[[2]] ( --5\n))( --3\na --[[4]] )".to_string())
    );
    assert_eq!(
        ts("local a = (--1\nfn--[[2]]  (--5\n  4 --[[6]]))(  --3\n a --[[4]])"),
        Ok("local a = ( --1\nfn --[[2]] ( --5\n4 --[[6]] ))( --3\na --[[4]] )".to_string())
    );
    assert_eq!(
        ts("local a = (--1\nfn--[[2]].--[[7]]fld1--8\n.--9\nfld2--[[10]]:--11\nfnname (--5\n  4 --[[6]]))(  --3\n a --[[4]])"),
        Ok("local a = ( --1\nfn --[[2]] . --[[7]] fld1 --8\n. --9\nfld2 --[[10]] : --11\nfnname( --5\n4 --[[6]] ))( --3\na --[[4]] )".to_string())
    );

    // PrefixExp
    assert_eq!(
        ts("local a = (--1\n{--[[2]]}--[[3]])--4\n[--5\n'a'--[=[6]=]]"),
        Ok("local a = ( --1\n{ --[[2]] } --[[3]] ) --4\n[ --5\n'a' --[=[6]=] ]".to_string())
    );
    assert_eq!(
        ts("(--1\n{--[[2]]}--[[3]])--4\n[--5\n'a'--[=[6]=]]--7\n(--8\n)"),
        Ok("( --1\n{ --[[2]] } --[[3]] ) --4\n[ --5\n'a' --[=[6]=] ] --7\n( --8\n)".to_string())
    );

    // Label
    assert_eq!(
        ts("::--1\nlabel1--[[2]]:: goto--[[3]]label1"),
        Ok(":: --1\nlabel1 --[[2]] :: goto --[[3]] label1".to_string())
    );
    assert_eq!(ts("::label1:: goto label1"), Ok("::label1:: goto label1".to_string()));

    // StatRetStat
    assert_eq!(ts("a = b; return"), Ok("a = b; return".to_string()));
    assert_eq!(ts("a = b; --[[1]] return"), Ok("a = b; --[[1]] return".to_string()));
    assert_eq!(ts("a = b; --[[1]] return--2\n;"), Ok("a = b; --[[1]] return --2\n;".to_string()));
    assert_eq!(
        ts("a = b; --[[1]] return--2\n2--[[3]],--[[4]]3"),
        Ok("a = b; --[[1]] return --2\n2 --[[3]] , --[[4]] 3".to_string())
    );
    assert_eq!(
        ts("a = b; --[[1]] return--2\n2--[[3]],--[[4]]3--5\n;"),
        Ok("a = b; --[[1]] return --2\n2 --[[3]] , --[[4]] 3 --5\n;".to_string())
    );

    // ParStats
    assert_eq!(
        ts("a = ({})[a]--1\n() --2\n break --3\n ({})--4\n[1]"),
        Ok("a = ({ })[a] --1\n() --2\nbreak --3\n({ }) --4\n[1]".to_string())
    );
    assert_eq!(
        ts("({})[a]--1\n() --2\n break --3\n ({})--4\n[1]"),
        Ok("({ })[a] --1\n() --2\nbreak --3\n({ }) --4\n[1]".to_string())
    );

    // If Then ElseIf Else End
    assert_eq!(
        ts(r#"if --[[1]] a > 3 --[[2]] then --[[3]] print(4) --[[4]] elseif --[[5]]a<3--[[6]] then --[[7]]print(2)--[[8]]
elseif --[[9]]a == 3 --[[10]]then--[[11]] print(3)--[[12]] else--[[13]] print(0) --[[14]]end"#),
        Ok(r#"if --[[1]] a > 3 --[[2]] then --[[3]] print(4) --[[4]] elseif --[[5]] a < 3 --[[6]] then --[[7]] print(2) --[[8]]
elseif --[[9]] a == 3 --[[10]] then --[[11]] print(3) --[[12]] else --[[13]] print(0) --[[14]] end"#.to_string())
    );
    assert_eq!(
        ts(r#"if --[[1]] a > 3 --[[2]] then --[[3]] elseif --[[5]]a<3--[[6]] then --[[7]]
elseif --[[9]]a == 3 --[[10]]then--[[11]] print(3)--[[12]] else--[[13]] print(0) --[[14]]end"#),
        Ok(r#"if --[[1]] a > 3 --[[2]] then --[[3]] elseif --[[5]] a < 3 --[[6]] then --[[7]]
elseif --[[9]] a == 3 --[[10]] then --[[11]] print(3) --[[12]] else --[[13]] print(0) --[[14]] end"#
            .to_string())
    );
    assert_eq!(
        ts(r#"if --[[1]] a > 3 --[[2]] then --[[3]] print(4) --[[4]] elseif --[[5]]a<3--[[6]] then --[[7]]print(2)--[[8]]
elseif --[[9]]a == 3 --[[10]]then--[[11]] print(3)--[[12]] else--[[13]] end"#),
        Ok(r#"if --[[1]] a > 3 --[[2]] then --[[3]] print(4) --[[4]] elseif --[[5]] a < 3 --[[6]] then --[[7]] print(2) --[[8]]
elseif --[[9]] a == 3 --[[10]] then --[[11]] print(3) --[[12]] else --[[13]] end"#.to_string())
    );
    assert_eq!(
        ts(r#"if --[[1]] a > 3 --[[2]] then --[[3]] elseif --[[5]]a<3--[[6]] then --[[7]]print(2)--[[8]]
elseif --[[9]]a == 3 --[[10]]then--[[11]] print(3)--[[12]] else--[[13]] end"#),
        Ok(r#"if --[[1]] a > 3 --[[2]] then --[[3]] elseif --[[5]] a < 3 --[[6]] then --[[7]] print(2) --[[8]]
elseif --[[9]] a == 3 --[[10]] then --[[11]] print(3) --[[12]] else --[[13]] end"#
            .to_string())
    );

    assert_eq!(
        ts(r#"if --[[1]] a > 3 --[[2]] then --[[3]] print(4) --[[4]] else--[[13]] print(0) --[[14]]end"#),
        Ok(r#"if --[[1]] a > 3 --[[2]] then --[[3]] print(4) --[[4]] else --[[13]] print(0) --[[14]] end"#.to_string())
    );
    assert_eq!(
        ts(r#"if --[[1]] a > 3 --[[2]] then --[[3]] else--[[13]] print(0) --[[14]]end"#),
        Ok(r#"if --[[1]] a > 3 --[[2]] then --[[3]] else --[[13]] print(0) --[[14]] end"#.to_string())
    );
    assert_eq!(
        ts(r#"if --[[1]] a > 3 --[[2]] then --[[3]] print(4) --[[4]] else--[[13]] end"#),
        Ok(r#"if --[[1]] a > 3 --[[2]] then --[[3]] print(4) --[[4]] else --[[13]] end"#.to_string())
    );
    assert_eq!(
        ts(r#"if --[[1]] a > 3 --[[2]] then --[[3]] else--[[13]] end"#),
        Ok(r#"if --[[1]] a > 3 --[[2]] then --[[3]] else --[[13]] end"#.to_string())
    );

    assert_eq!(
        ts(r#"if --[[1]] a > 3 --[[2]] then --[[3]] print(4) --[[4]] elseif --[[5]]a<3--[[6]] then --[[7]]print(2)--[[8]]
elseif --[[9]]a == 3 --[[10]]then--[[11]] print(3)--[[12]] end"#),
        Ok(r#"if --[[1]] a > 3 --[[2]] then --[[3]] print(4) --[[4]] elseif --[[5]] a < 3 --[[6]] then --[[7]] print(2) --[[8]]
elseif --[[9]] a == 3 --[[10]] then --[[11]] print(3) --[[12]] end"#.to_string())
    );
    assert_eq!(
        ts(r#"if --[[1]] a > 3 --[[2]] then --[[3]] elseif --[[5]]a<3--[[6]] then --[[7]]print(2)--[[8]]
elseif --[[9]]a == 3 --[[10]]then--[[11]] print(3)--[[12]] end"#),
        Ok(r#"if --[[1]] a > 3 --[[2]] then --[[3]] elseif --[[5]] a < 3 --[[6]] then --[[7]] print(2) --[[8]]
elseif --[[9]] a == 3 --[[10]] then --[[11]] print(3) --[[12]] end"#
            .to_string())
    );

    assert_eq!(
        ts(r#"if --[[1]] a > 3 --[[2]] then --[[3]] print(4) --[[4]] end"#),
        Ok(r#"if --[[1]] a > 3 --[[2]] then --[[3]] print(4) --[[4]] end"#.to_string())
    );
    assert_eq!(
        ts(r#"if --[[1]] a > 3 --[[2]] then --[[3]] end"#),
        Ok(r#"if --[[1]] a > 3 --[[2]] then --[[3]] end"#.to_string())
    );

    // local names = exprs
    assert_eq!(ts("local --[[1]]a"), Ok("local --[[1]] a".to_string()));
    assert_eq!(ts("local --[[1]]a--2\n,--[[7]] b"), Ok("local --[[1]] a --2\n, --[[7]] b".to_string()));
    assert_eq!(ts("local --[[1]]a--2\n=--[[7]] 1"), Ok("local --[[1]] a --2\n= --[[7]] 1".to_string()));
    assert_eq!(
        ts("local --[[1]]a--2\n,--3\n b --[[4]],--[[5]] c--6\n =--[[7]] 1--8\n,--9\n 2--10\n, --11\n3"),
        Ok("local --[[1]] a --2\n, --3\nb --[[4]] , --[[5]] c --6\n= --[[7]] 1 --8\n, --9\n2 --10\n, --11\n3"
            .to_string())
    );

    // FuncDecl
    assert_eq!(
        ts("function  --[[1]] a--2\n.--3\nb--[[4]](--5\na --6\n,--[[7]] b--[[8]])--[[9]] end"),
        Ok("function --[[1]] a --2\n. --3\nb --[[4]] ( --5\na --6\n, --[[7]] b --[[8]] ) --[[9]] end".to_string())
    );
    assert_eq!(
        ts("function  --[[1]] a--2\n.--3\nb--[[4]](--5\na --6\n,--[[7]] b--[[8]])--[[9]] print(a) --[[10]] end"),
        Ok("function --[[1]] a --2\n. --3\nb --[[4]] ( --5\na --6\n, --[[7]] b --[[8]] ) --[[9]] print(a) --[[10]] end".to_string())
    );
    assert_eq!(
        ts("local --[[0]] function  --[[1]] b--[[4]](--5\na --6\n,--[[7]] b--[[8]])--[[9]] end"),
        Ok("local --[[0]] function --[[1]] b --[[4]] ( --5\na --6\n, --[[7]] b --[[8]] ) --[[9]] end".to_string())
    );
    assert_eq!(
        ts("local --[[0]] function  --[[1]] b--[[4]](--5\na --6\n,--[[7]] b--[[8]])--[[9]] print(a)--[[10]]end"),
        Ok("local --[[0]] function --[[1]] b --[[4]] ( --5\na --6\n, --[[7]] b --[[8]] ) --[[9]] print(a) --[[10]] end".to_string())
    );
    assert_eq!(ts("function fn(  --5\n   --[[6]] )end"), Ok("function fn( --5\n --[[6]] ) end".to_string()));

    // For
    assert_eq!(
        ts("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\n, --7\n1 --8\ndo--9\n print(a) --[[10]]end"),
        Ok("for --[[1]] a --[[2]] = --[[3]] 1 --4\n, --5\n9 --6\n, --7\n1 --8\ndo --9\nprint(a) --[[10]] end"
            .to_string())
    );
    assert_eq!(
        ts("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\n, --7\n1 --8\ndo--9\n --[[10]]end"),
        Ok("for --[[1]] a --[[2]] = --[[3]] 1 --4\n, --5\n9 --6\n, --7\n1 --8\ndo --9\n --[[10]] end".to_string())
    );
    assert_eq!(
        ts("for --[[1]]a--[[2]] =--[[3]]--[[35]]1--4\n, --5\n9--6\ndo--9\n print(a) --[[10]]end"),
        Ok("for --[[1]] a --[[2]] = --[[3]] --[[35]] 1 --4\n, --5\n9 --6\ndo --9\nprint(a) --[[10]] end".to_string())
    );
    assert_eq!(
        ts("for --[[1]]a--[[2]] =--[[3]] 1--4\n, --5\n9--6\ndo--9\n --[[10]]end"),
        Ok("for --[[1]] a --[[2]] = --[[3]] 1 --4\n, --5\n9 --6\ndo --9\n --[[10]] end".to_string())
    );
    assert_eq!(
        ts("for --[[1]]a--[[2]] in--[[3]] ipairs(t)--4\ndo--9\n print(a) --[[10]]end"),
        Ok("for --[[1]] a --[[2]] in --[[3]] ipairs(t) --4\ndo --9\nprint(a) --[[10]] end".to_string())
    );
    assert_eq!(
        ts("for --[[1]]a--[[2]] in--[[3]] ipairs(t)--4\ndo--9\n --[[10]]end"),
        Ok("for --[[1]] a --[[2]] in --[[3]] ipairs(t) --4\ndo --9\n --[[10]] end".to_string())
    );

    // WhileDo
    assert_eq!(
        ts("while --1\n a < 3 --[[2]]do --[[3]]a = a + 1 --4\nend"),
        Ok("while --1\na < 3 --[[2]] do --[[3]] a = a + 1 --4\nend".to_string())
    );
    assert_eq!(
        ts("while --1\n a < 3 --[[2]]do--5\n --[[3]]end"),
        Ok("while --1\na < 3 --[[2]] do --5\n --[[3]] end".to_string())
    );

    // DoEnd
    assert_eq!(ts("do--1\n print(a)--[[2]]end"), Ok("do --1\nprint(a) --[[2]] end".to_string()));
    assert_eq!(ts("do--1\n --[[2]] end"), Ok("do --1\n --[[2]] end".to_string()));

    // RepeatUntil
    assert_eq!(
        ts("repeat--[[1]] print(a)--2\nuntil --[[3]]  a < 4"),
        Ok("repeat --[[1]] print(a) --2\nuntil --[[3]] a < 4".to_string())
    );
    assert_eq!(
        ts("repeat--[[1]] --2\nuntil --[[3]]  a < 4"),
        Ok("repeat --[[1]] --2\nuntil --[[3]] a < 4".to_string())
    );

    // VarsExprs
    assert_eq!(
        ts("a--[[1]].--2\nt--[[3]],--[[4]] a--5\n.--6\nr--[[7]] =--8\n 2--[[9]], --10\n'as'"),
        Ok("a --[[1]] . --2\nt --[[3]] , --[[4]] a --5\n. --6\nr --[[7]] = --8\n2 --[[9]] , --10\n'as'".to_string())
    );
    assert_eq!(
        ts("(--[[0]]a--[[1]])--11\n.--2\nt--[[3]],--[[4]] a--5\n.--6\nr--[[7]] =--8\n 2--[[9]], --10\n'as'"),
        Ok("( --[[0]] a --[[1]] ) --11\n. --2\nt --[[3]] , --[[4]] a --5\n. --6\nr --[[7]] = --8\n2 --[[9]] , --10\n'as'".to_string())
    );
}

#[test]
fn test_spaces_between_tokens_special() {
    let cfg = Config {
        fmt: FormatOpts {
            replace_zero_spaces_with_hint: Some(true),
            remove_spaces_between_tokens: Some(true),
            hint_before_comment: Some(" ".to_string()),
            hint_after_multiline_comment: Some(" ".to_string()),
            ..FormatOpts::default()
        },
        ..Config::default()
    };
    let ts = |s: &'static str| ts_base(s, &cfg);

    assert_eq!(ts("   "), Ok("".to_string()));
    assert_eq!(ts(" --[[1]] "), Ok(" --[[1]] ".to_string()));
    assert_eq!(ts("--[[1]] ; --2\n "), Ok(" --[[1]] ; --2\n".to_string()));
    assert_eq!(ts("--[[1]] print(a) --2\n "), Ok(" --[[1]] print(a) --2\n".to_string()));
    assert_eq!(
        ts("#!/usr/bin/lua\n--[[1]] print(a) --2\n "),
        Ok("#!/usr/bin/lua\n --[[1]] print(a) --2\n".to_string())
    );
    assert_eq!(
        ts("--123\n#!/usr/bin/lua\n--[[1]] print(a) --2\n "),
        Ok(" --123\n#!/usr/bin/lua\n --[[1]] print(a) --2\n".to_string())
    );
}
