use crate::config::*;
use std::fmt::Write;
use crate::parser::common::Loc;

#[macro_export]
macro_rules! out_of_range_write {
    ($wrt: expr, $cfg: expr, $buf: expr, $state: expr, $span: expr, $($arg:expr),+) => {{
        if util::test_out_of_range(&$state.pos_range, $span) {
            return write!($wrt, "{}", $span.substr($buf));
        } else if util::test_not_completely_contained(&$state.pos_range, $span) {
            return cfg_write!($wrt, $cfg, $buf, $state, $($arg),+);
        }
    }};
}

#[macro_export]
macro_rules! out_of_range_only_write {
    ($wrt: expr, $cfg: expr, $buf: expr, $state: expr, $span: expr) => {{
        if util::test_out_of_range(&$state.pos_range, $span) {
            return write!($wrt, "{}", $span.substr($buf));
        }
    }};
}

#[macro_export]
macro_rules! out_of_range_comment_only_write {
    ($wrt: expr, $cfg: expr, $buf: expr, $state: expr, $span: expr) => {{
        if util::test_out_of_range(&$state.comment_pos_range, $span) {
            return write!($wrt, "{}", $span.substr($buf));
        }
    }};
}

#[macro_export]
macro_rules! test_oneline {
    ($wrt:expr, $cfg:expr, $buf:expr, $state: expr, $($arg:expr),+) => {{
        let mut test_state = $state.clone();
        let mut test_cfg = $cfg.clone();
        let mut buffer = String::new();

        let mut flag = false;
        $( if cfg_write_helper!(&mut buffer, &mut test_cfg, $buf, &mut test_state, $arg).is_err() {
            flag = true;
        })+

        match flag {
            false => {
                // let mut left_len = util::get_len_after_newline(&buffer, $cfg);
                // if left_len == buffer.chars().count() {
                //     left_len += util::get_len_after_newline($wrt, $cfg);
                // }
                let left_len = util::get_len_after_newline($wrt, $cfg);
                let right_len = util::get_len_till_newline(&buffer, $cfg);

                match left_len + right_len < $cfg.fmt.max_width.unwrap() {
                    true => Some(buffer),
                    false => None,
                }
            }
            true => None
        }
    }};
}

#[macro_export]
macro_rules! test_oneline_no_nl {
    ($($arg:expr),+) => {{
        match test_oneline!($( $arg ),+ ) {
            Some(buffer) if !util::has_newlines(&buffer) => Some(buffer),
            _ => None,
        }
    }};
}

pub fn test_out_of_range(range: &Option<(usize, usize)>, span: &Loc) -> bool {
    if let Some(&(l, r)) = range.as_ref() {
        return span.1 <= l || span.0 >= r;
    }

    return false;
}

pub fn test_not_completely_contained(range: &Option<(usize, usize)>, span: &Loc) -> bool {
    if let Some(&(l, r)) = range.as_ref() {
        return (span.0 < l && span.1 > l) || (span.0 < r && span.1 > r);
    }

    return false;
}

pub fn trim_end_spaces_and_tabs<'a>(string: &'a String) -> &'a str {
    string.trim_end_matches(|ch: char| return ch == ' ' || ch == '\t')
}

pub fn get_len_after_newline(s: &str, _cfg: &Config) -> usize {
    return s.chars().rev().take_while(|&c| c != '\n').count();
}

pub fn get_len_till_newline(s: &str, _cfg: &Config) -> usize {
    return s.chars().take_while(|&c| c != '\n').count();
}

pub fn has_newlines(s: &str) -> bool {
    return s.find('\n').is_some();
}

pub fn charstring_to_normalstring(s: &str) -> String {
    let mut result = String::new();
    let mut escaped = false;

    for ch in s.chars() {
        if ch == '"' && !escaped {
            result.push('\\');
            result.push('"');
        } else {
            result.push(ch);
        }

        escaped = !escaped && ch == '\\';
    }
    result
}

pub fn write_indent(f: &mut String, cfg: &Config, state: &State) -> std::fmt::Result {
    let indentation = match &cfg.fmt.indentation_string {
        Some(indent_str) => (0..state.indent_level).map(|_| &indent_str[..]).collect::<String>(),
        None => String::new(),
    };

    write!(f, "{}", indentation)
}

pub fn line_range_to_pos_range(buf: &str, lr_opt: Option<(usize, usize)>) -> Option<(usize, usize)> {
    let mut nlpos: Vec<usize> = buf.chars().enumerate().filter(|(_, ch)| *ch == '\n').map(|(ind, _)| ind).collect();
    nlpos.insert(0, 0);
    nlpos.push(buf.chars().count());

    if let Some(&(l, r)) = lr_opt.as_ref() {
        let left = match l > 0 && l <= r && l < nlpos.len() {
            true => l,
            false => 1,
        };
        let right = match r > 0 && l <= r && r < nlpos.len() {
            true => r,
            false => nlpos.len() - 1,
        };

        return Some((nlpos[left - 1], nlpos[right]));
    }

    None
}

#[test]
fn test_line_range_to_pos_range() {
    let source = r#"/usr/bin/lua

fn = function(a)
    print("Какой-то текст в юникоде")
end

--comment
print(123)"#;

    assert_eq!(line_range_to_pos_range(&source, None), None);
    assert_eq!(line_range_to_pos_range(&source, Some((0, 100))), Some((0, 94)));
    assert_eq!(line_range_to_pos_range(&source, Some((1, 100))), Some((0, 94)));
    assert_eq!(line_range_to_pos_range(&source, Some((0, 8))), Some((0, 94)));
    assert_eq!(line_range_to_pos_range(&source, Some((1, 8))), Some((0, 94)));

    assert_eq!(line_range_to_pos_range(&source, Some((1, 1))), Some((0, 12)));
    assert_eq!(line_range_to_pos_range(&source, Some((2, 2))), Some((12, 13)));
    assert_eq!(line_range_to_pos_range(&source, Some((3, 3))), Some((13, 30)));
    assert_eq!(line_range_to_pos_range(&source, Some((4, 4))), Some((30, 68)));
    assert_eq!(line_range_to_pos_range(&source, Some((5, 5))), Some((68, 72)));
    assert_eq!(line_range_to_pos_range(&source, Some((6, 6))), Some((72, 73)));
    assert_eq!(line_range_to_pos_range(&source, Some((7, 7))), Some((73, 83)));
    assert_eq!(line_range_to_pos_range(&source, Some((8, 8))), Some((83, 94)));

    assert_eq!(line_range_to_pos_range(&source, Some((1, 2))), Some((0, 13)));
    assert_eq!(line_range_to_pos_range(&source, Some((2, 4))), Some((12, 68)));
    assert_eq!(line_range_to_pos_range(&source, Some((3, 6))), Some((13, 73)));
}

#[test]
fn test_trim_end_spaces_and_tabs() {
    assert_eq!(trim_end_spaces_and_tabs(&"abc\t  \t  ".to_string()), "abc");
    assert_eq!(trim_end_spaces_and_tabs(&"abc\t  \n\t  ".to_string()), "abc\t  \n");
    assert_eq!(trim_end_spaces_and_tabs(&"abc\t  \r\t  ".to_string()), "abc\t  \r");
}

#[test]
fn test_len_after_newline() {
    let cfg = Config::default();
    assert_eq!(get_len_after_newline("abc", &cfg), 3);
    assert_eq!(get_len_after_newline("abc\t  \n  ", &cfg), 2);
    assert_eq!(get_len_after_newline("abc\t  \n  absdsrf", &cfg), 9);
    assert_eq!(get_len_after_newline("\nabc\t dasdsadas \n  asdasdas\nabsdsrf", &cfg), 7);
    assert_eq!(get_len_after_newline("abc\t  \nабв", &cfg), 3);
}

#[test]
fn test_len_till_newline() {
    let cfg = Config::default();
    assert_eq!(get_len_till_newline("abc", &cfg), 3);
    assert_eq!(get_len_till_newline("abc\t  \n  ", &cfg), 6);
    assert_eq!(get_len_till_newline("abc\n  \n  absdsrf", &cfg), 3);
    assert_eq!(get_len_till_newline("\nabc\t dasdsadas", &cfg), 0);
    assert_eq!(get_len_till_newline("ab\nc\t  \nабв", &cfg), 2);
}

#[test]
fn test_has_newlines() {
    assert_eq!(has_newlines("abc\t  \n  "), true);
    assert_eq!(has_newlines("abc\t  \r\tasdas   "), false);
}

#[test]
fn test_charstring_to_normalstring() {
    assert_eq!(charstring_to_normalstring(r#" hi ab"cas"das   "#), r#" hi ab\"cas\"das   "#);
    assert_eq!(charstring_to_normalstring(r#" hi ab\"cas\"das   "#), r#" hi ab\"cas\"das   "#);
    assert_eq!(charstring_to_normalstring(r#" hi ab\\"cas\\"das   "#), r#" hi ab\\\"cas\\\"das   "#);
    assert_eq!(charstring_to_normalstring(r#" hi ab\\\"cas\\\"das   "#), r#" hi ab\\\"cas\\\"das   "#);
}
