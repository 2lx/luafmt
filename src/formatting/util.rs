use crate::config::*;
use std::cmp::Ordering;
use std::fmt::Write;

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

                match left_len + right_len < $cfg.max_width.unwrap() {
                    true => Some(buffer),
                    false => None,
                }
            }
            true => None
        }
    }};
}


pub fn longest_hint<'a>(hint1: &'a str, hint2: &'a str) -> &'a str {
    return match hint1.len().cmp(&hint2.len()) {
        Ordering::Less => hint2,
        Ordering::Greater => hint1,
        Ordering::Equal => hint1,
    };
}

pub fn trim_end_spaces_and_tabs<'a>(string: &'a String) -> &'a str {
    string.trim_end_matches(|ch: char| return ch == ' ' || ch == '\t')
}

pub fn get_len_after_newline(s: &str, _cfg: &Config) -> usize {
    return s.chars().rev().take_while(|&c| c != '\n').count()
}

pub fn get_len_till_newline(s: &str, _cfg: &Config) -> usize {
    return s.chars().take_while(|&c| c != '\n').count()
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
    let indentation = match &cfg.indentation_string {
        Some(indent_str) => (0..state.indent_level).map(|_| &indent_str[..]).collect::<String>(),
        None => String::new(),
    };

    write!(f, "{}", indentation)
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
