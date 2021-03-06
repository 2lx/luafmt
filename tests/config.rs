extern crate luafmt;

use luafmt::config::{Config, FormatOpts};
use std::path::Path;

#[test]
fn test_load_from_file() {
    let cfg_path_buf = Path::new("tests/scripts1/.luafmt.lua").to_path_buf();
    let actual = Config::default().reload_format_from_file(&cfg_path_buf);

    let expected = Config {
        fmt: FormatOpts {
            hint_after_multiline_comment: Some(" ".to_string()),
            hint_after_multiline_comment_text: Some(" ".to_string()),
            hint_before_comment: Some(" ".to_string()),
            hint_before_multiline_comment_text: Some(" ".to_string()),
            hint_before_oneline_comment_text: Some(" ".to_string()),
            remove_spaces_between_tokens: Some(true),
            replace_zero_spaces_with_hint: Some(true),
            indentation_string: Some("    ".to_string()),
            newline_format_statement: Some(1),
            newline_format_oneline_comment: Some(1),
            newline_format_first_oneline_comment: Some(1),
            newline_format_do_end: Some(1),
            newline_format_for: Some(1),
            newline_format_function: Some(1),
            newline_format_if: Some(1),
            newline_format_repeat_until: Some(1),
            newline_format_while: Some(1),
            field_separator: Some(",".to_string()),
            write_trailing_field_separator: Some(true),
            ..FormatOpts::default()
        },
        ..Config::default()
    };

    assert!(actual.is_ok());
    assert_eq!(actual.unwrap(), expected);

    let cfg_path_buf = Path::new("tests/scripts1/subdir1/subdir2/.luafmt_inner.lua").to_path_buf();
    let actual = Config::default().reload_format_from_file(&cfg_path_buf);

    let expected = Config::default();

    assert!(actual.is_ok());
    let actual = actual.unwrap();
    assert_eq!(actual, expected);
    assert_eq!(actual, Config::default());
    assert!(actual.has_empty_format());
}
