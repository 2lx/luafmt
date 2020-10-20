extern crate luafmt;

use std::path::Path;
use luafmt::config::Config;

#[test]
fn test_load_from_file() {
    let cfg_path_buf = Path::new("tests/scripts1/.luafmt.lua").to_path_buf();
    let actual = Config::load_from_file(&cfg_path_buf);

    let expected = Config {
        _empty: false,
        hint_after_multiline_comment: Some(" ".to_string()),
        hint_after_multiline_comment_text: Some(" ".to_string()),
        hint_before_comment: Some(" ".to_string()),
        hint_before_multiline_comment_text: Some(" ".to_string()),
        hint_before_oneline_comment_text: Some(" ".to_string()),
        remove_spaces_between_tokens: Some(true),
        replace_zero_spaces_with_hint: Some(true),
        indentation_string: Some("    ".to_string()),
        indent_every_statement: Some(true),
        indent_oneline_comments: Some(true),
        indent_first_oneline_comment: Some(true),
        do_end_indent_format: Some(1),
        for_indent_format: Some(1),
        function_indent_format: Some(1),
        if_indent_format: Some(1),
        repeat_until_indent_format: Some(1),
        while_do_indent_format: Some(1),
        field_separator: Some(",".to_string()),
        write_trailing_field_separator: Some(true),
        .. Config::default()
    };

    assert_eq!(actual, expected);

    let cfg_path_buf = Path::new("tests/scripts1/subdir1/subdir2/.luafmt_inner.lua").to_path_buf();
    let actual = Config::load_from_file(&cfg_path_buf);

    let expected = Config {
        _empty: true,
        .. Config::default()
    };

    assert_eq!(actual, expected);
    assert_eq!(actual, Config::default());
}
