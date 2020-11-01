use super::common::*;
use crate::config::*;

#[test]
fn test_charstring_and_normalstring() {
    let cfg = Config {
        fmt: FormatOpts { convert_charstring_to_normalstring: Some(true), ..FormatOpts::default() },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts("local a, b, c = 'abc\"', '\"bcd\"', \"c'd'e\" "),
        Ok(r#"local a, b, c = "abc\"", "\"bcd\"", "c'd'e" "#.to_string())
    );
    assert_eq!(
        ts(r#"local a, b, c = 'abc"', '"bcd"', "c'd'e" "#),
        Ok(r#"local a, b, c = "abc\"", "\"bcd\"", "c'd'e" "#.to_string())
    );
    assert_eq!(
        ts(r#"local a, b, c = 'abc\\"', '\"bcd\\\\"', "c\'d\'e" "#),
        Ok(r#"local a, b, c = "abc\\\"", "\"bcd\\\\\"", "c\'d\'e" "#.to_string())
    );
}

#[test]
fn test_unicode() {
    let cfg = Config {
        fmt: FormatOpts { convert_charstring_to_normalstring: Some(true), ..FormatOpts::default() },
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(
        ts(r#"local a = "Строка на русском языке""#),
        Ok(r#"local a = "Строка на русском языке""#.to_string())
    );

    let source = r#"local tbl = { field1 = "Какой-то текст", field2 = "Some text" }"#;
    assert_eq!(ts(source), Ok(source.to_string()), "{:?}", crate::parser::parse_lua(source));

    let source = r#"local tbl = { field1 = { "Текст на русском" }, field2 = { "Some text" } }"#;
    assert_eq!(ts(source), Ok(source.to_string()), "{:?}", crate::parser::parse_lua(source));
}
