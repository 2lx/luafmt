use super::common::*;
use crate::config::*;

#[test]
fn test_field_options() {
    let cfg = Config::default();
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(
        ts("local a = { a, b; c ={}, d = 5--[[]]; e }"),
        Ok("local a = { a, b; c ={}, d = 5--[[]]; e }".to_string())
    );
    assert_eq!(
        ts("local a = { t = { 1, 2, 3 }; b, c ={}, d = 5--[[ hoho ]]; e; }"),
        Ok("local a = { t = { 1, 2, 3 }; b, c ={}, d = 5--[[ hoho ]]; e; }".to_string())
    );

    let cfg = Config { field_separator: Some(";".to_string()), ..Config::default() };
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(
        ts("local a = { a, b; c ={}, d = 5--[[]]; e }"),
        Ok("local a = { a; b; c ={}; d = 5--[[]]; e }".to_string())
    );
    assert_eq!(
        ts("local a = { t = { 1, 2, 3 }; b, c ={}, d = 5--[[ hoho ]]; e; }"),
        Ok("local a = { t = { 1; 2; 3 }; b; c ={}; d = 5--[[ hoho ]]; e; }".to_string())
    );

    let cfg = Config { field_separator: Some(",".to_string()), ..Config::default() };
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(
        ts("local a = { a, b; c ={}, d = 5--[[]]; e }"),
        Ok("local a = { a, b, c ={}, d = 5--[[]], e }".to_string())
    );
    assert_eq!(
        ts("local a = { t = { 1, 2, 3 }; b, c ={}, d = 5--[[ hoho ]]; e; }"),
        Ok("local a = { t = { 1, 2, 3 }, b, c ={}, d = 5--[[ hoho ]], e, }".to_string())
    );

    let cfg = Config { write_trailing_field_separator: Some(true), ..Config::default() };
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(
        ts("local a = { a, b; c ={}, d = 5--[[]]; e }"),
        Ok("local a = { a, b; c ={}, d = 5--[[]]; e }".to_string())
    );
    assert_eq!(
        ts("local a = { t = { 1, 2, 3 }; b, c ={}, d = 5--[[ hoho ]]; e; }"),
        Ok("local a = { t = { 1, 2, 3 }; b, c ={}, d = 5--[[ hoho ]]; e; }".to_string())
    );

    let cfg = Config {
        field_separator: Some(",".to_string()),
        write_trailing_field_separator: Some(true),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(
        ts("local a = { a, b; c ={}, d = 5--[[]]; e }"),
        Ok("local a = { a, b, c ={}, d = 5--[[]], e, }".to_string())
    );
    assert_eq!(
        ts("local a = { t = { 1, 2, 3 }; b, c ={}, d = 5--[[ hoho ]]; e; }"),
        Ok("local a = { t = { 1, 2, 3, }, b, c ={}, d = 5--[[ hoho ]], e, }".to_string())
    );

    let cfg = Config {
        field_separator: Some(";".to_string()),
        write_trailing_field_separator: Some(true),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);
    assert_eq!(
        ts("local a = { a, b; c ={}, d = 5--[[]]; e }"),
        Ok("local a = { a; b; c ={}; d = 5--[[]]; e; }".to_string())
    );
    assert_eq!(
        ts("local a = { t = { 1, 2, 3 }; b, c ={}, d = 5--[[ hoho ]]; e; }"),
        Ok("local a = { t = { 1; 2; 3; }; b; c ={}; d = 5--[[ hoho ]]; e; }".to_string())
    );
}
