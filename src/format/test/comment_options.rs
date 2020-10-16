use crate::config::*;
use crate::parser::parse_lua;

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
enum TestError {
    ErrorWhileParsing,
    ErrorWhileWriting,
}

#[allow(dead_code)]
fn ts_base(source: &str, cfg: &Config) -> Result<String, TestError> {
    match parse_lua(source) {
        Err(_) => Err(TestError::ErrorWhileParsing),
        Ok(result) => {
            let mut output = String::new();
            let state = State::default();

            match result.configured_write(&mut output, cfg, source, &state) {
                Ok(_) => Ok(output),
                _ => Err(TestError::ErrorWhileWriting),
            }
        }
    }
}

#[test]
fn test_replace_zero_spaces_with_hint() {
    let cfg = Config { replace_zero_spaces_with_hint: Some(true), ..Config::default() };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(ts("for a=1,   4do print  (1,4)end"), Ok("for a = 1,   4 do print  (1, 4) end".to_string()));
    assert_eq!(
        ts("for a=1,--[[  asd ]]  \n  4do print --1\n (1,4)end"),
        Ok("for a = 1,--[[  asd ]]  \n  4 do print --1\n (1, 4) end".to_string())
    );
}

#[test]
fn test_remove_spaces_between_tokens() {
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
}

#[test]
fn test_combo_spaces() {
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
        Ok("#!/usr/bin/lua\nlocal b = { 2, 3 } for a = 1,--[[  asd ]]\n4 do print--1\n(1, 4) end--[=[1232 ]=]".to_string())
    );
}

#[test]
fn test_hint_after_multiline_comment() {
    let cfg = Config {
        hint_after_multiline_comment: Some("W".to_string()),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("for a=1,--[[  asd ]]  \n  4do print --1\n --[[ text ]](1,4)end"),
        Ok("for a=1,--[[  asd ]]  \n  4do print --1\n --[[ text ]]W(1,4)end".to_string())
    );

    assert_eq!(
        ts("#!/usr/bin/lua\n local  b = {2, 3} for a=1,--[[  asd ]]  \n  4do print --1\n (1,4)end --[=[1232 ]=]"),
        Ok("#!/usr/bin/lua\n local  b = {2, 3} for a=1,--[[  asd ]]  \n  4do print --1\n (1,4)end --[=[1232 ]=]W".to_string())
    );
}

#[test]
fn test_hint_before_comment() {
    let cfg = Config {
        hint_before_comment: Some("W".to_string()),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("for a=1,--[[  asd ]]  \n  4do print --1\n --[[ text ]](1,4)end"),
        Ok("for a=1,W--[[  asd ]]  \n  4do print --1\n --[[ text ]](1,4)end".to_string())
    );

    assert_eq!(
        ts("#!/usr/bin/lua\n local --1\n b = {2, 3} for a=1,--[[  asd ]]  \n  4do print--1\n (1,4)end--[=[1232 ]=]"),
        Ok("#!/usr/bin/lua\n local --1\n b = {2, 3} for a=1,W--[[  asd ]]  \n  4do printW--1\n (1,4)endW--[=[1232 ]=]".to_string())
    );
}

#[test]
fn test_hint_before_comment_text() {
    let cfg = Config {
        hint_before_oneline_comment_text: Some("W1".to_string()),
        hint_before_multiline_comment_text: Some("W2".to_string()),
        hint_after_multiline_comment_text: Some("W3".to_string()),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("for a=1,--[[  asd ]]  \n  4do print --1\n --[[ text ]](1,4)end"),
        Ok("for a=1,--[[W2asdW3]]  \n  4do print --W11\n --[[W2textW3]](1,4)end".to_string())
    );

    assert_eq!(
        ts("#!/usr/bin/lua\n local --1\n b = {2, 3} for a=1,--[[  asd ]]  \n  4do print--1\n (1,4)end--[=[1232 ]=]"),
        Ok("#!/usr/bin/lua\n local --W11\n b = {2, 3} for a=1,--[[W2asdW3]]  \n  4do print--W11\n (1,4)end--[=[W21232W3]=]".to_string())
    );
}
