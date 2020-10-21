use crate::config::*;
use super::common::*;

#[test]
fn test_replace_zero_spaces_with_hint() {
    let cfg = Config { replace_zero_spaces_with_hint: Some(true), ..Config::default() };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(ts("for a=1,   4do print  (1,4)end"), Ok("for a = 1,   4 do print  (1, 4) end".to_string()));
    assert_eq!(
        ts("for a=1,--[[  asd ]]  \n  4do print --1\n (1,4)end"),
        Ok("for a = 1,--[[  asd ]]  \n  4 do print --1\n (1, 4) end".to_string())
    );

    let cfg = Config { remove_spaces_between_tokens: Some(true), ..Config::default() };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("for a=1,--[[  asd ]]  \n  4do print --1\n (1,4)end"),
        Ok("fora=1,--[[  asd ]]\n4doprint--1\n(1,4)end".to_string())
    );

    assert_eq!(
        ts("#!/usr/bin/lua\n local  b = {2, 3} for a=1,--[[  asd ]]  \n  4do print --1\n (1,4)end --[=[1232 ]=]"),
        Ok("#!/usr/bin/lua\nlocalb={2,3}fora=1,--[[  asd ]]\n4doprint--1\n(1,4)end--[=[1232 ]=]".to_string())
    );

    let cfg = Config {
        replace_zero_spaces_with_hint: Some(true),
        remove_spaces_between_tokens: Some(true),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("for a=1,--[[  asd ]]  \n  4do print --1\n (1,4)end"),
        Ok("for a = 1,--[[  asd ]]\n4 do print--1\n(1, 4) end".to_string())
    );

    assert_eq!(
        ts("#!/usr/bin/lua\n local  b = {2, 3} for a=1,--[[  asd ]]  \n  4do print --1\n (1,4)end --[=[1232 ]=]"),
        Ok("#!/usr/bin/lua\nlocal b = {2, 3} for a = 1,--[[  asd ]]\n4 do print--1\n(1, 4) end--[=[1232 ]=]"
            .to_string())
    );
}

#[test]
fn test_comment_hints() {
    let cfg = Config { hint_after_multiline_comment: Some("W".to_string()), ..Config::default() };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("for a=1,--[[  asd ]]  \n  4do print --1\n --[[ text ]](1,4)end"),
        Ok("for a=1,--[[  asd ]]  \n  4do print --1\n --[[ text ]]W(1,4)end".to_string())
    );

    assert_eq!(
        ts("#!/usr/bin/lua\n local  b = {2, 3} for a=1,--[[  asd ]]  \n  4do print --1\n (1,4)end --[=[1232 ]=]"),
        Ok("#!/usr/bin/lua\n local  b = {2, 3} for a=1,--[[  asd ]]  \n  4do print --1\n (1,4)end --[=[1232 ]=]W"
            .to_string())
    );

    let cfg = Config { hint_before_comment: Some("W".to_string()), ..Config::default() };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("for a=1,--[[  asd ]]  \n  4do print --1\n --[[ text ]](1,4)end"),
        Ok("for a=1,W--[[  asd ]]  \n  4do print --1\n --[[ text ]](1,4)end".to_string())
    );

    assert_eq!(
        ts("#!/usr/bin/lua\n local --1\n b = {2, 3} for a=1,--[[  asd ]]  \n  4do print--1\n (1,4)end--[=[1232 ]=]"),
        Ok("#!/usr/bin/lua\n local --1\n b = {2, 3} for a=1,W--[[  asd ]]  \n  4do printW--1\n (1,4)endW--[=[1232 ]=]"
            .to_string())
    );

    let cfg = Config {
        hint_before_oneline_comment_text: Some("W1".to_string()),
        hint_before_multiline_comment_text: Some("W2".to_string()),
        hint_after_multiline_comment_text: Some("W3".to_string()),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("for a=1,--[[  asd ]] --[==[abc]==]  \n  4do print --1\n --[[text ]](1,4)end"),
        Ok("for a=1,--[[W2asdW3]] --[==[W2abcW3]==]  \n  4do print --W11\n --[[W2textW3]](1,4)end".to_string())
    );

    assert_eq!(
        ts("#!/usr/bin/lua\n local --1\n b = {2, 3} for a=1,--[[  asd ]]  \n  4do print--1\n (1,4)end--[=[1232 ]=]"),
        Ok("#!/usr/bin/lua\n local --W11\n b = {2, 3} for a=1,--[[W2asdW3]]  \n  4do print--W11\n (1,4)end--[=[W21232W3]=]".to_string())
    );

    let cfg = Config { hint_after_multiline_comment_text: Some("W3".to_string()), ..Config::default() };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("for a=1,--[[  asd ]] --[==[abc]==]  \n  4do print --1\n --[[text ]](1,4)end"),
        Ok("for a=1,--[[  asdW3]] --[==[abcW3]==]  \n  4do print --1\n --[[textW3]](1,4)end".to_string())
    );

    assert_eq!(
        ts("#!/usr/bin/lua\n local --1\n b = {2, 3} for a=1,--[[  asd ]]  \n  4do print--1\n (1,4)end--[=[1232 ]=]"),
        Ok("#!/usr/bin/lua\n local --1\n b = {2, 3} for a=1,--[[  asdW3]]  \n  4do print--1\n (1,4)end--[=[1232W3]=]"
            .to_string())
    );

    let cfg = Config { hint_before_multiline_comment_text: Some("W2".to_string()), ..Config::default() };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("for a=1,--[[  asd ]] --[==[abc]==]  \n  4do print --1\n --[[text ]](1,4)end"),
        Ok("for a=1,--[[W2asd ]] --[==[W2abc]==]  \n  4do print --1\n --[[W2text ]](1,4)end".to_string())
    );

    assert_eq!(
        ts("#!/usr/bin/lua\n local --1\n b = {2, 3} for a=1,--[[  asd ]]  \n  4do print--1\n (1,4)end--[=[1232 ]=]"),
        Ok("#!/usr/bin/lua\n local --1\n b = {2, 3} for a=1,--[[W2asd ]]  \n  4do print--1\n (1,4)end--[=[W21232 ]=]"
            .to_string())
    );
}

#[test]
fn test_remove_comments_newlines() {
    let cfg = Config { remove_comments: Some(true), ..Config::default() };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("for a=1,--[[  asd ]]  \n  4do print --1\n --[[ text ]](1,4)end"),
        Ok("for a=1,  \n  4do print \n (1,4)end".to_string())
    );

    assert_eq!(
        ts("#!/usr/bin/lua\n local --1\n b = {2, 3} for a=1,--[[  asd ]]  \n  4do print--1\n (1,4)end--[=[1232 ]=]"),
        Ok("#!/usr/bin/lua\n local \n b = {2, 3} for a=1,  \n  4do print\n (1,4)end".to_string())
    );

    let cfg = Config { remove_newlines: Some(true), ..Config::default() };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("for a=1,--[[  asd ]]  \n  4do print --1\n --[[ text ]](1,4)end"),
        Ok("for a=1,--[[  asd ]]    4do print --1\n --[[ text ]](1,4)end".to_string())
    );

    assert_eq!(
        ts("#!/usr/bin/lua\n local --1\n b = {2, 3} for a=1,--[[  asd ]]  \n  4do print--1\n (1,4)end--[=[1232 ]=]"),
        Ok("#!/usr/bin/lua\n local --1\n b = {2, 3} for a=1,--[[  asd ]]    4do print--1\n (1,4)end--[=[1232 ]=]"
            .to_string())
    );

    let cfg = Config { remove_comments: Some(true), remove_newlines: Some(true), ..Config::default() };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("for a=1,--[[  asd ]]  \n  4do print --1\n --[[ text ]](1,4)end"),
        Ok("for a=1,    4do print  (1,4)end".to_string())
    );

    assert_eq!(
        ts("#!/usr/bin/lua\n local --1\n b = {2, 3} for a=1,--[[  asd ]]  \n  4do print--1\n (1,4)end--[=[1232 ]=]"),
        Ok("#!/usr/bin/lua\n local  b = {2, 3} for a=1,    4do print (1,4)end".to_string())
    );
}
