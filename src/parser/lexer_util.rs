type TChars<'a> = std::iter::Peekable<std::iter::Enumerate<std::str::Chars<'a>>>;

fn seek_end_by_predicate(chars: &mut TChars, start: usize, f: &dyn Fn(char, bool) -> bool) -> (usize, bool, String) {
    let mut result = String::new();

    if chars.peek().is_none() {
        return (start, false, result);
    };

    let mut end = start;
    let mut escaped = false;

    loop {
        match chars.peek() {
            Some(&(i, ch)) => {
                if f(ch, escaped) {
                    return (i, true, result);
                }

                end = i;
                chars.next();
                escaped = !escaped && ch == '\\';
                result.push(ch);
            }
            None => return (end + 1, false, result),
        };
    }
}

pub fn get_shebang_ends(chars: &mut TChars, start: usize) -> (usize, usize, String) {
    // we already got "#!" symbols
    let (end, succ, result) = seek_end_by_predicate(chars, start, &|ch: char, _| ch == '\n');

    if succ {
        // skip '\n'
        chars.next();
        return (end, end + 1, result);
    }
    return (end, end, result);
}

pub fn get_integer_end(chars: &mut TChars, start: usize) -> (usize, bool, String) {
    let (end, succ, result) = seek_end_by_predicate(chars, start, &|ch: char, _| !ch.is_ascii_digit());

    if start >= end && !succ {
        return (end, false, result);
    } else {
        // integer numbers always end correctly
        return (end, true, result);
    }
}

pub fn get_hex_integer_end(chars: &mut TChars, start: usize) -> (usize, bool, String) {
    let (end, succ, result) = seek_end_by_predicate(chars, start, &|ch: char, _| {
        !ch.is_ascii_digit() && !(ch >= 'A' && ch <= 'F') && !(ch >= 'a' && ch <= 'f')
    });

    if start >= end && !succ {
        return (end, false, result);
    } else {
        // hex integer numbers always end correctly
        return (end, true, result);
    }
}

pub fn get_float_end(chars: &mut TChars, start: usize) -> (usize, bool, String) {
    // we already got one float symbol
    let (end, _, mut result) = seek_end_by_predicate(chars, start, &|ch: char, _| !ch.is_ascii_digit() && ch != '.');

    match chars.peek() {
        Some(&(i, ch @ 'e')) | Some(&(i, ch @ 'E')) => {
            chars.next();
            result.push(ch);

            let mut cur_ind = i;
            match chars.peek() {
                Some(&(si, '-')) => {
                    chars.next();
                    result.push('-');
                    cur_ind = si;
                }
                Some(&(_, _)) => {}
                None => return (i + 1, false, String::new()),
            };
            let (end, flag, int_end) = get_integer_end(chars, cur_ind + 1);
            if flag {
                result.push_str(&int_end);
                return (end, true, result);
            }

            return (end, false, result);
        }
        Some(&(_, _)) => return (end, true, result),
        None => return (end, true, result),
    }
}

pub fn get_variable_end(chars: &mut TChars, start: usize) -> (usize, bool, String) {
    let (end, succ, result) = seek_end_by_predicate(chars, start, &|ch: char, _| {
        !ch.is_ascii_alphabetic() && !ch.is_ascii_digit() && ch != '_'
    });

    if !succ && start >= end {
        return (end, false, result);
    }

    // variables always end correctly
    return (end, true, result);
}

pub fn get_string_ends(chars: &mut TChars, prefix: char, start: usize) -> (usize, usize, bool, String) {
    // we already got one symbol - prefix
    let (text_end, succ, result) =
        seek_end_by_predicate(chars, start, &|ch: char, escaped: bool| !escaped && ch == prefix);

    if succ {
        // skip `prefix` char
        chars.next();
        return (text_end, text_end + 1, true, result);
    } else {
        return (text_end, text_end, false, result);
    }
}

fn get_oneline_comment_ends(chars: &mut TChars, start: usize) -> (usize, usize, bool, String) {
    // we already got "--" symbols
    let (text_end, succ, result) = seek_end_by_predicate(chars, start, &|ch: char, _| ch == '\n');

    if succ {
        // skip '\n'
        chars.next();
        return (text_end, text_end + 1, true, result);
    } else {
        return (text_end, text_end, true, result);
    }
}

pub fn get_multiline_string_level(chars: &mut TChars, start: usize) -> usize {
    let (end, _, _) = seek_end_by_predicate(chars, start, &|ch: char, _| ch != '=');

    match end >= start {
        true => return end - start,
        false => return 0,
    }
}

pub fn get_multiline_string_ends(chars: &mut TChars, level: usize, start: usize) -> (usize, usize, bool, String) {
    // we already got "[==[" symbols
    let mut end = start;
    let mut escaped = false;
    let mut result = String::new();

    loop {
        match chars.next() {
            Some((text_end, ch)) => {
                if !escaped && ch == ']' {
                    let cur_level = get_multiline_string_level(chars, text_end + 1);

                    if level == cur_level {
                        match chars.peek() {
                            Some(&(token_end, ']')) => {
                                chars.next();
                                return (text_end, token_end + 1, true, result);
                            }
                            Some(&(_, _)) => {}
                            None => return (text_end + cur_level + 1, text_end + cur_level + 1, false, result),
                        }
                    }
                    result.push(']');
                    let level_str = (0..cur_level).map(|_| "=").collect::<String>();
                    result.push_str(&level_str);
                } else {
                    result.push(ch);
                }

                end = text_end;
                escaped = ch == '\\';
            }
            None => return (end + 1, end + 1, false, result),
        }
    }
}

pub fn get_comment_start_ends_and_type(chars: &mut TChars, start: usize) -> (usize, usize, usize, Option<usize>, bool) {
    // we already got "--" symbols
    let mut text_start = start;

    match chars.peek() {
        Some(&(_, '[')) => {
            chars.next();

            match chars.peek() {
                Some(&(level_start, '=')) => {
                    let level = get_multiline_string_level(chars, level_start);

                    match chars.peek() {
                        Some(&(square_2_index, '[')) => {
                            chars.next();

                            text_start = square_2_index + 1;
                            let (text_end, token_end, succ, _) = get_multiline_string_ends(chars, level, text_start);

                            return (text_start, text_end, token_end, Some(level), succ);
                        }
                        Some(&(cur_i, _)) => {
                            let (text_end, token_end, succ, _) = get_oneline_comment_ends(chars, cur_i);
                            return (text_start, text_end, token_end, None, succ);
                        }
                        None => return (text_start, text_start + level + 1, text_start + level + 1, None, true),
                    }
                }
                Some(&(square_2_index, '[')) => {
                    chars.next();

                    text_start = square_2_index + 1;
                    let (text_end, token_end, succ, _) = get_multiline_string_ends(chars, 0, text_start);
                    return (text_start, text_end, token_end, Some(0), succ);
                }
                Some(&(cur_i, _)) => {
                    let (text_end, token_end, succ, _) = get_oneline_comment_ends(chars, cur_i);
                    return (text_start, text_end, token_end, None, succ);
                }
                None => return (text_start, text_start + 1, text_start + 1, None, true),
            }
        }
        _ => {
            let (text_end, token_end, succ, _) = get_oneline_comment_ends(chars, text_start);
            return (text_start, text_end, token_end, None, succ);
        }
    };
}

#[test]
fn test_get_shebang_end() {
    let mystr = String::from("#!/usr/bin/lua\n  ");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    assert_eq!(get_shebang_ends(&mut iter, 0), (14, 15, "!/usr/bin/lua".to_string()));

    let mystr = String::from("#!/usr/bin/lua\n");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    assert_eq!(get_shebang_ends(&mut iter, 0), (14, 15, "!/usr/bin/lua".to_string()));

    let mystr = String::from("#!/usr/bin/lua");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    assert_eq!(get_shebang_ends(&mut iter, 0), (14, 14, "!/usr/bin/lua".to_string()));

    let mystr = String::from("#");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    assert_eq!(get_shebang_ends(&mut iter, 1), (1, 1, "".to_string()));

    let mystr = String::from("#!");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    assert_eq!(get_shebang_ends(&mut iter, 1), (2, 2, "!".to_string()));

    let mystr = String::from("#!");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_shebang_ends(&mut iter, 2), (2, 2, "".to_string()));
}

#[test]
fn test_get_integer_end() {
    let mystr = String::from("123");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_integer_end(&mut iter, 0), (3, true, "123".to_string()));

    let mystr = String::from("-123");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    assert_eq!(get_integer_end(&mut iter, 0), (4, true, "123".to_string()));

    let mystr = String::from("");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_integer_end(&mut iter, 0), (0, false, "".to_string()));

    let mystr = String::from("-");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    assert_eq!(get_integer_end(&mut iter, 1), (1, false, "".to_string()));
}

#[test]
fn test_get_hex_integer_end() {
    let mystr = String::from("123");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_hex_integer_end(&mut iter, 0), (3, true, "123".to_string()));

    let mystr = String::from("1234567890ABCDEF1");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_hex_integer_end(&mut iter, 0), (17, true, "1234567890ABCDEF1".to_string()));

    let mystr = String::from("-123AEF");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    assert_eq!(get_hex_integer_end(&mut iter, 0), (7, true, "123AEF".to_string()));

    let mystr = String::from("");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_hex_integer_end(&mut iter, 0), (0, false, String::new()));

    let mystr = String::from("-");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    assert_eq!(get_hex_integer_end(&mut iter, 1), (1, false, String::new()));
}

#[test]
fn test_get_float_end() {
    let mystr = String::from("123.4");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_float_end(&mut iter, 0), (5, true, "123.4".to_string()));

    let mystr = String::from("123.4E-3");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_float_end(&mut iter, 0), (8, true, "123.4E-3".to_string()));

    let mystr = String::from(".123");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_float_end(&mut iter, 0), (4, true, ".123".to_string()));

    let mystr = String::from("-123.");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    assert_eq!(get_float_end(&mut iter, 0), (5, true, "123.".to_string()));

    let mystr = String::from("");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_float_end(&mut iter, 0), (0, true, "".to_string()));

    let mystr = String::from(",");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_float_end(&mut iter, 0), (0, true, "".to_string()));

    let mystr = String::from("-");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    assert_eq!(get_float_end(&mut iter, 1), (1, true, "".to_string()));

    let mystr = String::from("123.4E");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_float_end(&mut iter, 0), (6, false, String::new()));

    let mystr = String::from("123.4e-");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_float_end(&mut iter, 0), (7, false, "123.4e-".to_string()));

    let mystr = String::from("123.4e-5");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_float_end(&mut iter, 0), (8, true, "123.4e-5".to_string()));

    let mystr = String::from("123.4E5");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_float_end(&mut iter, 0), (7, true, "123.4E5".to_string()));
}

#[test]
fn test_get_variable_end() {
    let mystr = String::from("a");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_variable_end(&mut iter, 0), (1, true, "a".to_string()));

    let mystr = String::from("_ab3b3");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_variable_end(&mut iter, 0), (6, true, "_ab3b3".to_string()));

    let mystr = String::from("");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_variable_end(&mut iter, 0), (0, false, String::new()));
}

#[test]
fn test_get_string_ends() {
    let mystr = String::from("'123456' ");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    assert_eq!(get_string_ends(&mut iter, '\'', 0), (7, 8, true, "123456".to_string()));

    let mystr = String::from("\"123456\"");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    assert_eq!(get_string_ends(&mut iter, '"', 0), (7, 8, true, "123456".to_string()));

    // escaped
    let mystr = String::from("\"123456\\\"");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    assert_eq!(get_string_ends(&mut iter, '"', 0), (9, 9, false, "123456\\\"".to_string()));

    let mystr = String::from("\"123456\\\\\"");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    assert_eq!(get_string_ends(&mut iter, '"', 0), (9, 10, true, "123456\\\\".to_string()));

    let mystr = String::from("'123456");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    assert_eq!(get_string_ends(&mut iter, '\'', 0), (7, 7, false, "123456".to_string()));

    let mystr = String::from("\" str str\ntrt\"");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    assert_eq!(get_string_ends(&mut iter, '"', 0), (13, 14, true, " str str\ntrt".to_string()));

    let mystr = String::from("\" str str\ntrt");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    assert_eq!(get_string_ends(&mut iter, '"', 0), (13, 13, false, " str str\ntrt".to_string()));
}

#[test]
fn test_get_oneline_comment_ends() {
    let mystr = String::from("--\n");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_oneline_comment_ends(&mut iter, 2), (2, 3, true, "".to_string()));

    let mystr = String::from("--");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_oneline_comment_ends(&mut iter, 2), (2, 2, true, "".to_string()));

    let mystr = String::from("-- 123\n");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_oneline_comment_ends(&mut iter, 2), (6, 7, true, " 123".to_string()));

    let mystr = String::from("-- 123");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_oneline_comment_ends(&mut iter, 2), (6, 6, true, " 123".to_string()));
}

#[test]
fn test_get_multiline_string_level() {
    let mystr = String::from("=[");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_multiline_string_level(&mut iter, 0), 1);

    let mystr = String::from("=");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_multiline_string_level(&mut iter, 0), 1);

    let mystr = String::from("===[");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_multiline_string_level(&mut iter, 0), 3);

    let mystr = String::from("===");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_multiline_string_level(&mut iter, 0), 3);

    let mystr = String::from("[");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_multiline_string_level(&mut iter, 0), 0);

    let mystr = String::from("");
    let mut iter = mystr.chars().enumerate().peekable();
    assert_eq!(get_multiline_string_level(&mut iter, 0), 0);
}

#[test]
fn test_get_multiline_string_ends() {
    let mystr = String::from("[[123]]");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_multiline_string_ends(&mut iter, 0, 2), (5, 7, true, "123".to_string()));

    let mystr = String::from("[[]]");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_multiline_string_ends(&mut iter, 0, 2), (2, 4, true, "".to_string()));

    let mystr = String::from("[=[striing\n\\\"'\"]=]");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    iter.next();
    assert_eq!(get_multiline_string_ends(&mut iter, 1, 3), (15, 18, true, "striing\n\\\"'\"".to_string()));

    let mystr = String::from("[=[striing\n\"'\"]=]");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    iter.next();
    assert_eq!(get_multiline_string_ends(&mut iter, 1, 3), (14, 17, true, "striing\n\"'\"".to_string()));

    let mystr = String::from("[===[abc]=]]====]==]===]=]");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    iter.next();
    iter.next();
    iter.next();
    assert_eq!(get_multiline_string_ends(&mut iter, 3, 5), (19, 24, true, "abc]=]]====]==".to_string()));

    let mystr = String::from("[[abc");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_multiline_string_ends(&mut iter, 0, 2), (5, 5, false, "abc".to_string()));

    let mystr = String::from("[[abc]");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_multiline_string_ends(&mut iter, 0, 2), (6, 6, false, "abc".to_string()));

    let mystr = String::from("[=[abc]=");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    iter.next();
    assert_eq!(get_multiline_string_ends(&mut iter, 1, 2), (8, 8, false, "abc".to_string()));

    let mystr = String::from("[===[abc]=]]====]==]==]=]");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    iter.next();
    iter.next();
    iter.next();
    assert_eq!(get_multiline_string_ends(&mut iter, 3, 5), (25, 25, false, "abc]=]]====]==]==]=]".to_string()));
}

#[test]
fn test_get_comment_start_end_and_type() {
    let mystr = String::from("--\n");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (2, 2, 3, None, true));

    let mystr = String::from("--");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (2, 2, 2, None, true));

    let mystr = String::from("--123\n");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (2, 5, 6, None, true));

    let mystr = String::from("--123");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (2, 5, 5, None, true));

    let mystr = String::from("--[[]]");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (4, 4, 6, Some(0), true));

    let mystr = String::from("--[[123]]");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (4, 7, 9, Some(0), true));

    let mystr = String::from("--[=[]=]");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (5, 5, 8, Some(1), true));

    let mystr = String::from("--[===[123]===]");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (7, 10, 15, Some(3), true));

    let mystr = String::from("--[===123\n");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (2, 9, 10, None, true));

    let mystr = String::from("--[===123");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (2, 9, 9, None, true));

    let mystr = String::from("--[===[123");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (7, 10, 10, Some(3), false));

    let mystr = String::from("--[===[123]");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (7, 11, 11, Some(3), false));

    let mystr = String::from("--[===[123]===\n");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (7, 15, 15, Some(3), false));

    let mystr = String::from("--[===[123]=== ]");
    let mut iter = mystr.chars().enumerate().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (7, 16, 16, Some(3), false));
}
