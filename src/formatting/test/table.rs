use crate::config::*;
use super::common::*;

#[test]
fn test_table_iv_oneline() {
    let cfg = Config {
        hint_table_constructor: Some(" ".to_string()),
        replace_zero_spaces_with_hint: Some(true),
        remove_spaces_between_tokens: Some(true),
        remove_single_newlines: Some(true),
        newline_format_table_constructor: Some(1),
        newline_format_table_field: Some(1),
        enable_oneline_table_constructor: Some(true),
        enable_oneline_iv_table: Some(true),
        max_width: Some(100),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(ts(r#"local a = { { {a=1, b= 2},{ a=2,b=3}}, { { a = 1, b=4}}}"#),
               Ok("local a = { { { a = 1, b = 2 }, { a = 2, b = 3 } }, { { a = 1, b = 4 } } }".to_string()));

    let cfg = Config {
        hint_table_constructor: Some(" ".to_string()),
        replace_zero_spaces_with_hint: Some(true),
        remove_spaces_between_tokens: Some(true),
        remove_single_newlines: Some(true),
        newline_format_table_constructor: Some(1),
        newline_format_table_field: Some(1),
        enable_oneline_table_constructor: Some(true),
        enable_oneline_iv_table: Some(true),
        indentation_string: Some("    ".to_string()),
        max_width: Some(30),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(ts(r#"local a = { { {a=1, b= 2},{ a=2,b=3}}, { { a = 1, b=4}}}"#),
               Ok(r#"local a = { {
    { a = 1, b = 2 },
    { a = 2, b = 3 }
}, { { a = 1, b = 4 } } }"#.to_string()));

    let cfg = Config {
        hint_table_constructor: Some(" ".to_string()),
        replace_zero_spaces_with_hint: Some(true),
        remove_spaces_between_tokens: Some(true),
        remove_single_newlines: Some(true),
        newline_format_table_constructor: Some(1),
        newline_format_table_field: Some(1),
        enable_oneline_table_constructor: Some(true),
        enable_oneline_iv_table: Some(true),
        indentation_string: Some("    ".to_string()),
        max_width: Some(110),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(ts(r#"local a = { { {a=1, b= 2},{ a=2,b=3}}, { { a = 1, b=4}}}"#),
               Ok(r#"local a = { { { a = 1, b = 2 }, { a = 2, b = 3 } }, { { a = 1, b = 4 } } }"#.to_string()));


    let source = r#"local a = { { {
    { {
        { a = 1, b = 2, c = "asdfj;lkfkjsalfkjfsdsffdsfdsfdsfsddsfsdfsdfdsf" },
        { a = 2, b = 2, c = "asdfj;lkfkjsalfkjfsdsffdsfdsfdsfsddsfsdfsdfdsf" }
    }, {
        { a = 1, b = 2, c = "asdfj;lkfkjsalfkjfsdsffdsfdsfdsfsddsfsdfsdfdsf" },
        { a = 2, b = 2, c = "asdfj;lkfkjsalfkjfsdsffdsfdsfdsfsddsfsdfsdfdsf" },
        { a = 3, b = 2, c = "asdfj;lkfkjsalfkjfsdsffdsfdsfdsfsddsfsdfsdfdsf" }
    }, {
        { a = 1, b = 2, c = "asdfj;lkfkjsalfkjfsdsfdsfdsfdsfsdfdsfsdfsdfdsf" },
        { a = 2, b = 2, c = "asdfj;lkfkjsalfkjfsdsffdsfdsfdsfsddsfsdfsdfdsf" },
        { a = 3, b = 2, c = "asdfj;lkfkjsalfkjfsdsfdsfdsfdsfsddsfsdfsdfdsff" },
        { a = 4, b = 2, c = "asdfj;lkfkjsalfkjfsdsfdsfdsfdsfsdfdsfsdfsdfdsf" }
    } }
}, {
    { { a = 1, b = 2, c = "asdfj;lkfkjsalfkjfsdsfdsfdsfdsfsdfdsfsdfsdfdsf" } }, {
        { a = 2, b = 2, c = "asdfj;lkfkjsalfkjfsdsfdsfdsfdsfsdfdsfsdfsdfdsf" },
        { a = 3, b = 2, c = "asdfj;lkfkjsalfkjfsdsfdsfdsfdsfsdfdsfsdfsdfdsf" }
    }
} } }"#;
    assert_eq!(ts(source), Ok(source.to_string()));
}
