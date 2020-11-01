use crate::config::*;
use std::fmt::Write;

#[macro_export]
macro_rules! out_of_range_write {
    ($wrt: expr, $buf: expr, $state: expr, $span: expr) => {{
        if let Some(&(l, r)) = $state.pos_range.as_ref() {
            if $span.1 <= l || $span.0 > r {
                return write!($wrt, "{}", &$buf[$span.0..$span.1]);
            }
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
    let indentation = match &cfg.fmt.indentation_string {
        Some(indent_str) => (0..state.indent_level).map(|_| &indent_str[..]).collect::<String>(),
        None => String::new(),
    };

    write!(f, "{}", indentation)
}

pub fn line_range_to_pos_range(buf: &str, lr_opt: Option<(usize, usize)>) -> (usize, usize) {
    let mut nlpos: Vec::<usize> = buf.chars().enumerate().filter(|(_, ch)| *ch == '\n').map(|(ind, _)| ind).collect();
    nlpos.insert(0, 0);
    nlpos.push(buf.chars().count());

    if let Some(lr) = &lr_opt {
        if lr.0 > 0 && lr.1 > 0 && lr.0 <= lr.1 && lr.0 < nlpos.len() && lr.1 < nlpos.len() {
            return (nlpos[lr.0 - 1], nlpos[lr.1]);
        }
    }

    (nlpos[0], nlpos[nlpos.len() - 1])
}

#[test]
fn test_line_range_to_pos_range() {
    let source = r#"/usr/bin/lua

fn = function(a)
    print(a)
end

--comment
print(123)"#;

    assert_eq!(line_range_to_pos_range(&source, None), (0, 69));
    assert_eq!(line_range_to_pos_range(&source, Some((0, 100))), (0, 69));
    assert_eq!(line_range_to_pos_range(&source, Some((1, 100))), (0, 69));
    assert_eq!(line_range_to_pos_range(&source, Some((0, 8))), (0, 69));
    assert_eq!(line_range_to_pos_range(&source, Some((1, 8))), (0, 69));

    assert_eq!(line_range_to_pos_range(&source, Some((1, 1))), (0, 12));
    assert_eq!(line_range_to_pos_range(&source, Some((2, 2))), (12, 13));
    assert_eq!(line_range_to_pos_range(&source, Some((3, 3))), (13, 30));
    assert_eq!(line_range_to_pos_range(&source, Some((4, 4))), (30, 43));
    assert_eq!(line_range_to_pos_range(&source, Some((5, 5))), (43, 47));
    assert_eq!(line_range_to_pos_range(&source, Some((6, 6))), (47, 48));
    assert_eq!(line_range_to_pos_range(&source, Some((7, 7))), (48, 58));
    assert_eq!(line_range_to_pos_range(&source, Some((8, 8))), (58, 69));

    assert_eq!(line_range_to_pos_range(&source, Some((1, 2))), (0, 13));
    assert_eq!(line_range_to_pos_range(&source, Some((2, 4))), (12, 43));
    assert_eq!(line_range_to_pos_range(&source, Some((3, 6))), (13, 48));
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
