use std::str::CharIndices;
type TChars<'input> = std::iter::Peekable<CharIndices<'input>>;

fn seek_end_by_predicate(chars: &mut TChars, start: usize, f: &dyn Fn(char, bool) -> bool) -> (usize, bool) {
    match chars.peek() {
        None => return (start, false),
        _ => {}
    };

    let mut end = start;
    let mut escaped = false;

    loop {
        match chars.peek() {
            Some(&(i, ch)) => {
                if f(ch, escaped) {
                    return (i, true);
                }

                end = i;
                chars.next();
                escaped = ch == '\\';
            }
            None => return (end + 1, false),
        };
    }
}

pub fn get_shebang_ends(chars: &mut TChars, start: usize) -> (usize, usize) {
    // we already got "#!" symbols
    let (end, succ) = seek_end_by_predicate(chars, start, &|ch: char, _| ch == '\n');

    if succ {
        // skip '\n'
        chars.next();
        return (end, end + 1);
    }
    return (end, end);
}

pub fn get_integer_end(chars: &mut TChars, start: usize) -> (usize, bool) {
    let (end, succ) = seek_end_by_predicate(chars, start, &|ch: char, _| !ch.is_ascii_digit());

    if start >= end && !succ {
        return (end, false);
    } else {
        // integer numbers always end correctly
        return (end, true);
    }
}

pub fn get_hex_integer_end(chars: &mut TChars, start: usize) -> (usize, bool) {
    let (end, succ) = seek_end_by_predicate(chars, start, &|ch: char, _| {
        !ch.is_ascii_digit() && !(ch >= 'A' && ch <= 'F') && !(ch >= 'a' && ch <= 'f')
    });

    if start >= end && !succ {
        return (end, false);
    } else {
        // hex integer numbers always end correctly
        return (end, true);
    }
}

pub fn get_float_end(chars: &mut TChars, start: usize) -> (usize, bool) {
    // we already got one float symbol
    let (end, _) = seek_end_by_predicate(chars, start, &|ch: char, _| !ch.is_ascii_digit() && ch != '.');

    match chars.peek() {
        Some(&(i, 'e')) | Some(&(i, 'E')) => {
            chars.next();

            let mut cur_ind = i;
            match chars.peek() {
                Some(&(si, '-')) => {
                    chars.next();
                    cur_ind = si;
                }
                Some(&(_, _)) => {}
                None => return (i + 1, false),
            };
            return get_integer_end(chars, cur_ind + 1);
        }
        Some(&(_, _)) => return (end, true),
        None => return (end, true),
    }
}

pub fn get_variable_end(chars: &mut TChars, start: usize) -> (usize, bool) {
    let (end, succ) = seek_end_by_predicate(chars, start, &|ch: char, _| {
        !ch.is_ascii_alphabetic() && !ch.is_ascii_digit() && ch != '_'
    });

    if !succ && start >= end {
        return (end, false);
    }

    // variables always end correctly
    return (end, true);
}

pub fn get_string_ends(chars: &mut TChars, prefix: char, start: usize) -> (usize, usize, bool) {
    // we already got one symbol - prefix
    let (text_end, succ) = seek_end_by_predicate(chars, start, &|ch: char, escaped: bool| !escaped && ch == prefix);

    if succ {
        // skip `prefix` char
        chars.next();
        return (text_end, text_end + 1, true);
    } else {
        return (text_end, text_end, false);
    }
}

fn get_oneline_comment_ends(chars: &mut TChars, start: usize) -> (usize, usize, bool) {
    // we already got "--" symbols
    let (text_end, succ) = seek_end_by_predicate(chars, start, &|ch: char, _| ch == '\n');

    if succ {
        // skip '\n'
        chars.next();
        return (text_end, text_end + 1, true);
    } else {
        return (text_end, text_end, true);
    }
}

pub fn get_multiline_string_level(chars: &mut TChars, start: usize) -> usize {
    let (end, _) = seek_end_by_predicate(chars, start, &|ch: char, _| ch != '=');

    match end >= start {
        true => return end - start,
        false => return 0,
    }
}

pub fn get_multiline_string_ends(chars: &mut TChars, level: usize, start: usize) -> (usize, usize, bool) {
    // we already got "[==[" symbols
    let mut end = start;
    let mut escaped = false;

    loop {
        match chars.next() {
            Some((text_end, ch)) => {
                if !escaped && ch == ']' {
                    let cur_level = get_multiline_string_level(chars, text_end + 1);

                    if level == cur_level {
                        match chars.peek() {
                            Some(&(token_end, ']')) => {
                                chars.next();
                                return (text_end, token_end + 1, true);
                            }
                            Some(&(_, _)) => (),
                            None => return (text_end + cur_level + 1, text_end + cur_level + 1, false),
                        }
                    }
                }

                end = text_end;
                escaped = ch == '\\';
            }
            None => return (end + 1, end + 1, false),
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
                            let (text_end, token_end, succ) = get_multiline_string_ends(chars, level, text_start);

                            return (text_start, text_end, token_end, Some(level), succ);
                        }
                        Some(&(cur_i, _)) => {
                            let (text_end, token_end, succ) = get_oneline_comment_ends(chars, cur_i);
                            return (text_start, text_end, token_end, None, succ);
                        }
                        None => return (text_start, text_start + level + 1, text_start + level + 1, None, true),
                    }
                }
                Some(&(square_2_index, '[')) => {
                    chars.next();

                    text_start = square_2_index + 1;
                    let (text_end, token_end, succ) = get_multiline_string_ends(chars, 0, text_start);
                    return (text_start, text_end, token_end, Some(0), succ);
                }
                Some(&(cur_i, _)) => {
                    let (text_end, token_end, succ) = get_oneline_comment_ends(chars, cur_i);
                    return (text_start, text_end, token_end, None, succ);
                }
                None => return (text_start, text_start + 1, text_start + 1, None, true),
            }
        }
        _ => {
            let (text_end, token_end, succ) = get_oneline_comment_ends(chars, text_start);
            return (text_start, text_end, token_end, None, succ);
        }
    };
}

#[test]
fn test_get_shebang_end() {
    let mystr = String::from("#!/usr/bin/lua\n  ");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    assert_eq!(get_shebang_ends(&mut iter, 0), (14, 15));

    let mystr = String::from("#!/usr/bin/lua\n");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    assert_eq!(get_shebang_ends(&mut iter, 0), (14, 15));

    let mystr = String::from("#!/usr/bin/lua");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    assert_eq!(get_shebang_ends(&mut iter, 0), (14, 14));

    let mystr = String::from("#");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    assert_eq!(get_shebang_ends(&mut iter, 1), (1, 1));

    let mystr = String::from("#!");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    assert_eq!(get_shebang_ends(&mut iter, 1), (2, 2));

    let mystr = String::from("#!");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_shebang_ends(&mut iter, 2), (2, 2));
}

#[test]
fn test_get_integer_end() {
    let mystr = String::from("123");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_integer_end(&mut iter, 0), (3, true));

    let mystr = String::from("-123");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    assert_eq!(get_integer_end(&mut iter, 0), (4, true));

    let mystr = String::from("");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_integer_end(&mut iter, 0), (0, false));

    let mystr = String::from("-");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    assert_eq!(get_integer_end(&mut iter, 1), (1, false));
}

#[test]
fn test_get_hex_integer_end() {
    let mystr = String::from("123");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_hex_integer_end(&mut iter, 0), (3, true));

    let mystr = String::from("1234567890ABCDEF1");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_hex_integer_end(&mut iter, 0), (17, true));

    let mystr = String::from("-123AEF");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    assert_eq!(get_hex_integer_end(&mut iter, 0), (7, true));

    let mystr = String::from("");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_hex_integer_end(&mut iter, 0), (0, false));

    let mystr = String::from("-");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    assert_eq!(get_hex_integer_end(&mut iter, 1), (1, false));
}

#[test]
fn test_get_float_end() {
    let mystr = String::from("123.4");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_float_end(&mut iter, 0), (5, true));

    let mystr = String::from("123.4E-3");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_float_end(&mut iter, 0), (8, true));

    let mystr = String::from(".123");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_float_end(&mut iter, 0), (4, true));

    let mystr = String::from("-123.");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    assert_eq!(get_float_end(&mut iter, 0), (5, true));

    let mystr = String::from("");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_float_end(&mut iter, 0), (0, true));

    let mystr = String::from(",");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_float_end(&mut iter, 0), (0, true));

    let mystr = String::from("-");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    assert_eq!(get_float_end(&mut iter, 1), (1, true));

    let mystr = String::from("123.4E");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_float_end(&mut iter, 0), (6, false));

    let mystr = String::from("123.4e-");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_float_end(&mut iter, 0), (7, false));

    let mystr = String::from("123.4e-5");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_float_end(&mut iter, 0), (8, true));

    let mystr = String::from("123.4E5");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_float_end(&mut iter, 0), (7, true));
}

#[test]
fn test_get_variable_end() {
    let mystr = String::from("a");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_variable_end(&mut iter, 0), (1, true));

    let mystr = String::from("_ab3b3");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_variable_end(&mut iter, 0), (6, true));

    let mystr = String::from("");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_variable_end(&mut iter, 0), (0, false));
}

#[test]
fn test_get_string_ends() {
    let mystr = String::from("'123456' ");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    assert_eq!(get_string_ends(&mut iter, '\'', 0), (7, 8, true));

    let mystr = String::from("\"123456\"");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    assert_eq!(get_string_ends(&mut iter, '"', 0), (7, 8, true));

    let mystr = String::from("'123456");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    assert_eq!(get_string_ends(&mut iter, '\'', 0), (7, 7, false));

    let mystr = String::from("\" str str\ntrt\"");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    assert_eq!(get_string_ends(&mut iter, '"', 0), (13, 14, true));

    let mystr = String::from("\" str str\ntrt");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    assert_eq!(get_string_ends(&mut iter, '"', 0), (13, 13, false));
}

#[test]
fn test_get_oneline_comment_ends() {
    let mystr = String::from("--\n");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_oneline_comment_ends(&mut iter, 2), (2, 3, true));

    let mystr = String::from("--");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_oneline_comment_ends(&mut iter, 2), (2, 2, true));

    let mystr = String::from("-- 123\n");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_oneline_comment_ends(&mut iter, 2), (6, 7, true));

    let mystr = String::from("-- 123");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_oneline_comment_ends(&mut iter, 2), (6, 6, true));
}

#[test]
fn test_get_multiline_string_level() {
    let mystr = String::from("=[");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_multiline_string_level(&mut iter, 0), 1);

    let mystr = String::from("=");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_multiline_string_level(&mut iter, 0), 1);

    let mystr = String::from("===[");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_multiline_string_level(&mut iter, 0), 3);

    let mystr = String::from("===");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_multiline_string_level(&mut iter, 0), 3);

    let mystr = String::from("[");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_multiline_string_level(&mut iter, 0), 0);

    let mystr = String::from("");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_multiline_string_level(&mut iter, 0), 0);
}

#[test]
fn test_get_multiline_string_ends() {
    let mystr = String::from("[[123]]");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_multiline_string_ends(&mut iter, 0, 2), (5, 7, true));

    let mystr = String::from("[[]]");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_multiline_string_ends(&mut iter, 0, 2), (2, 4, true));

    let mystr = String::from("[=[striing\n\"'\"]=]");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    iter.next();
    assert_eq!(get_multiline_string_ends(&mut iter, 1, 3), (14, 17, true));

    let mystr = String::from("[===[abc]=]]====]==]===]=]");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    iter.next();
    iter.next();
    iter.next();
    assert_eq!(get_multiline_string_ends(&mut iter, 3, 5), (19, 24, true));

    let mystr = String::from("[[abc");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_multiline_string_ends(&mut iter, 0, 2), (5, 5, false));

    let mystr = String::from("[[abc]");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_multiline_string_ends(&mut iter, 0, 2), (6, 6, false));

    let mystr = String::from("[=[abc]=");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    iter.next();
    assert_eq!(get_multiline_string_ends(&mut iter, 1, 2), (8, 8, false));

    let mystr = String::from("[===[abc]=]]====]==]==]=]");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    iter.next();
    iter.next();
    iter.next();
    assert_eq!(get_multiline_string_ends(&mut iter, 3, 5), (25, 25, false));
}

#[test]
fn test_get_comment_start_end_and_type() {
    let mystr = String::from("--\n");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (2, 2, 3, None, true));

    let mystr = String::from("--");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (2, 2, 2, None, true));

    let mystr = String::from("--123\n");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (2, 5, 6, None, true));

    let mystr = String::from("--123");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (2, 5, 5, None, true));

    let mystr = String::from("--[[]]");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (4, 4, 6, Some(0), true));

    let mystr = String::from("--[[123]]");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (4, 7, 9, Some(0), true));

    let mystr = String::from("--[=[]=]");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (5, 5, 8, Some(1), true));

    let mystr = String::from("--[===[123]===]");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (7, 10, 15, Some(3), true));

    let mystr = String::from("--[===123\n");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (2, 9, 10, None, true));

    let mystr = String::from("--[===123");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (2, 9, 9, None, true));

    let mystr = String::from("--[===[123");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (7, 10, 10, Some(3), false));

    let mystr = String::from("--[===[123]");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (7, 11, 11, Some(3), false));

    let mystr = String::from("--[===[123]===\n");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (7, 15, 15, Some(3), false));

    let mystr = String::from("--[===[123]=== ]");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_ends_and_type(&mut iter, 2), (7, 16, 16, Some(3), false));
}
