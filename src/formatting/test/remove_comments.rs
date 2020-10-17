use crate::config::*;
use super::common::*;

#[test]
fn test_remove_comments_ops() {
    let cfg = Config { remove_comments: Some(true), ..Config::default() };
    let ts = |s: &str| ts_base(s, &cfg);

    // binary ops
    for op in vec![
        "+", "-", "or", "and", "==", "~=", ">=", "<=", "<", ">", "|", "~", "&", ">>", "<<", "..", "*", "/", "//", "%",
        "^",
    ] {
        let left = format!("c   --1\n   --1.3\n =  --[=[2]=]   a  --3\n  {}   --[[4]]   b", op);
        let right = format!("c   \n   \n =     a  \n  {}      b", op);
        assert_eq!(ts(&left), Ok(right));

        let left = format!("c = a--\n{} --[[342]]b", op);
        let right = format!("c = a\n{} b", op);
        assert_eq!(ts(&left), Ok(right));
    }

    // unary ops
    for op in vec!["not", "-", "#", "~"] {
        let left = format!("c--[=[1]=]=--2\n{} --3\nb", op);
        let right = format!("c=\n{} \nb", op);
        assert_eq!(ts(&left), Ok(right));

        let left = format!("c   --1\n  =  --[[2]]   {}  --3\n  b", op);
        let right = format!("c   \n  =     {}  \n  b", op);
        assert_eq!(ts(&left), Ok(right));
    }
}

#[test]
fn test_remove_comments_other() {
    let cfg = Config { remove_comments: Some(true), ..Config::default() };
    let ts = |s: &'static str| ts_base(s, &cfg);

    // TableConstructor
    assert_eq!(ts("t={--\n}"), Ok("t={\n}".to_string()));
    assert_eq!(ts("t = { a --\n  =  --[[]]  3}"), Ok("t = { a \n  =    3}".to_string()));
    assert_eq!(
        ts("t = { [ --c1\n a --[[c2]]] --c3\n= --c4\n 3}"),
        Ok("t = { [ \n a ] \n= \n 3}".to_string())
    );
    assert_eq!(
        ts("t = { [ --c1\n 'a' --[[c2]]] --c3\n= --c4\n 3}"),
        Ok("t = { [ \n 'a' ] \n= \n 3}".to_string())
    );
    assert_eq!(
        ts("t = { [ --c1\n \"a\" --[[c2]]] --c3\n= --c4\n 3} --123213"),
        Ok("t = { [ \n \"a\" ] \n= \n 3} \n".to_string())
    );
}

#[test]
fn test_remove_comments_special() {
    let cfg = Config { remove_comments: Some(true), ..Config::default() };
    let ts = |s: &'static str| ts_base(s, &cfg);

    assert_eq!(ts("   "), Ok("   ".to_string()));
    assert_eq!(ts("--[[1]]"), Ok("".to_string()));
    assert_eq!(ts("--[[1]] ; --2\n "), Ok(" ; \n ".to_string()));
    assert_eq!(ts("--[[1]] print(a) --2\n "), Ok(" print(a) \n ".to_string()));
    assert_eq!(ts("#!/usr/bin/lua\n--[[1]] print(a) --2\n "), Ok("#!/usr/bin/lua\n print(a) \n ".to_string()));
    assert_eq!(ts("--123\n#!/usr/bin/lua\n--[[1]] print(a) --2\n "), Ok("\n#!/usr/bin/lua\n print(a) \n ".to_string()));
}
