use super::common::*;
use crate::config::*;

#[test]
fn test_charstring_and_normalstring() {
    let cfg = Config { convert_charstring_to_normalstring: Some(true), ..Config::default() };
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
fn test_indent_table_hard() {
    // let cfg = Config { convert_charstring_to_normalstring: Some(true), ..Config::default() };
    // let ts = |s: &str| ts_base(s, &cfg);
    //
    // local blocks = { {
    //         id = _blockid(1),
    //         type = "user_input",
    //         input = { data = {
    //             { id = "PHN.79262089601", duration = 100500 },
    //             { id = "PHN.79185449997" },
    //             { id = "PHN.79262089601", testfield = "Test" },
    //         } },
    //     }, {
    //         id = _blockid(2),
    //         type = "classify"
    // } }
}
