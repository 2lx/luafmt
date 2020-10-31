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
        enable_oneline_table: Some(true),
        enable_oneline_kv_table_field: Some(true),
        enable_oneline_iv_table_field: Some(true),
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
        enable_oneline_table: Some(true),
        enable_oneline_kv_table_field: Some(true),
        enable_oneline_iv_table_field: Some(true),
        indentation_string: Some("    ".to_string()),
        max_width: Some(22),
        ..Config::default()
    };
    let ts = |s: &str| ts_base(s, &cfg);

    assert_eq!(ts(r#"local a = { { {a=1, b= 2},{ a=2,b=3}}, { { a = 1, b=4}}}"#),
               Ok(r#"local a = { {
    { a = 1, b = 2 },
    { a = 2, b = 3 }
}, {
    { a = 1, b = 4 }
} }"#.to_string()));

    let cfg = Config {
        hint_table_constructor: Some(" ".to_string()),
        replace_zero_spaces_with_hint: Some(true),
        remove_spaces_between_tokens: Some(true),
        remove_single_newlines: Some(true),
        newline_format_table_constructor: Some(1),
        newline_format_table_field: Some(1),
        enable_oneline_table: Some(true),
        enable_oneline_kv_table_field: Some(false),
        enable_oneline_iv_table_field: Some(true),
        indentation_string: Some("    ".to_string()),
        max_width: Some(90),
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
    { { a = 1, b = 2, c = "asdfj;lkfkjsalfkjfsdsfdsfdsfdsfsdfdsfsdfsdfdsf" } },
    {
        { a = 2, b = 2, c = "asdfj;lkfkjsalfkjfsdsfdsfdsfdsfsdfdsfsdfsdfdsf" },
        { a = 3, b = 2, c = "asdfj;lkfkjsalfkjfsdsfdsfdsfdsfsdfdsfsdfsdfdsf" }
    }
} } }"#;
    assert_eq!(ts(source), Ok(source.to_string()));

    let source = r#"data = {
    { field1 = "someid", field2 = 100500 },
    { field1 = "somevalue" },
    { field1 = "somestring", testfield = "Test" },
}"#;
    assert_eq!(ts(source), Ok(source.to_string()));

    let source = r#"fields = { {
    field1 = fieldvalue,
    field2 = "string",
    field3 = {
        field31 = { { field = fields[1] }, { field = fields[2] } },
        field32 = { { field = fields[1] }, { field = fields[2] } },
        field33 = { fields = field2, fields = { "field", "field" }, field = 1 },
    },
} }"#;
    assert_eq!(ts(source), Ok(source.to_string()));

    let source = r#"field = { {
    field = {
        functionname(2, 1, "string", field_name, 123123),
        functionname(1, 2, "string", field_name, 123123),
    },
    field2 = fieldname(1),
} }"#;
    assert_eq!(ts(source), Ok(source.to_string()));

    let source = r#"field = { {
    field = { func(2, 1, "string"), func(1, 2, "string") },
    field2 = fieldname(1),
} }"#;
    assert_eq!(ts(source), Ok(source.to_string()));
}
