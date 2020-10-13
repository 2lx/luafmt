use std::str::CharIndices;
type TChars<'input> = std::iter::Peekable<CharIndices<'input>>;

fn seek_end_by_predicate(chars: &mut TChars, start: usize, f: &dyn Fn(char, bool) -> bool) -> usize {
    let mut end = start;
    let mut escaped = false;

    loop {
        match chars.peek() {
            Some(&(i, ch)) => {
                end = i;
                if f(ch, escaped) {
                    break;
                }

                chars.next();
                escaped = ch == '\\';
            },
            None => {
                end += 1;
                break;
            }
        };
    }

    end
}

pub fn get_integer_end(chars: &mut TChars, start: usize) -> usize {
    seek_end_by_predicate(chars, start, &|ch: char, _| !ch.is_ascii_digit())
}

pub fn get_hex_integer_end(chars: &mut TChars, start: usize) -> usize {
    seek_end_by_predicate(chars, start, &|ch: char, _| {
        !ch.is_ascii_digit() && !(ch >= 'A' && ch <= 'F') && !(ch >= 'a' && ch <= 'f')
    })
}

pub fn get_float_end(chars: &mut TChars, start: usize) -> usize {
    let mut end = seek_end_by_predicate(chars, start, &|ch: char, _| !ch.is_ascii_digit() && ch != '.');

    match chars.peek() {
        Some(&(i, 'e')) | Some(&(i, 'E')) => {
            chars.next();
            match chars.peek() {
                Some(&(_, '-')) => {
                    chars.next();
                }
                _ => {}
            };
            end = get_integer_end(chars, i);
        }
        _ => {}
    }

    end
}

pub fn get_variable_end(chars: &mut TChars, start: usize) -> usize {
    seek_end_by_predicate(chars, start, &|ch: char, _| !ch.is_ascii_alphabetic() && !ch.is_ascii_digit() && ch != '_')
}

pub fn get_string_end(chars: &mut TChars, prefix: char, start: usize) -> usize {
    seek_end_by_predicate(chars, start, &|ch: char, escaped: bool| !escaped && ch == prefix)
}

fn get_oneline_comment_end(chars: &mut TChars, start: usize) -> usize {
    seek_end_by_predicate(chars, start, &|ch: char, _| ch == '\n')
}

pub fn get_multiline_string_level(chars: &mut TChars, start: usize) -> usize {
    let end = seek_end_by_predicate(chars, start, &|ch: char, _| ch != '=');

    match end <= start {
        true => 0,
        false => end - start,
    }
}

pub fn get_multiline_string_end(chars: &mut TChars, level: usize, start: usize) -> usize {
    let mut end = start;
    let mut escaped = false;

    while let Some((i, ch)) = chars.next() {
        end = i;
        if !escaped && ch == ']' {
            let cur_level = get_multiline_string_level(chars, i + 1);

            if level == cur_level {
                match chars.peek() {
                    Some(&(_, ']')) => {
                        break;
                    }
                    _ => (),
                }
            }
        }
        escaped = ch == '\\';
    }

    end
}

pub fn get_comment_start_end_and_type(chars: &mut TChars, start: usize) -> Option<(usize, usize, Option<usize>)> {
    let mut text_start = start;
    let text_end;
    let mut ml_level: Option<usize> = None;

    match chars.peek() {
        Some(&(_, '[')) => {
            chars.next();

            match chars.peek() {
                Some(&(level_start, '=')) => {
                    let level = get_multiline_string_level(chars, level_start);

                    match chars.peek() {
                        Some(&(square_2_start, '[')) => {
                            chars.next();

                            ml_level = Some(level);
                            text_start = square_2_start + 1;
                            text_end = get_multiline_string_end(chars, level, text_start);
                        }
                        Some(&(cur_i, _)) => {
                            text_end = get_oneline_comment_end(chars, cur_i);
                        }
                        None => return None,
                    }
                }
                Some(&(square_2_start, '[')) => {
                    chars.next();

                    ml_level = Some(0);
                    text_start = square_2_start + 1;
                    text_end = get_multiline_string_end(chars, 0, text_start);
                }
                Some(&(cur_i, _)) => {
                    text_end = get_oneline_comment_end(chars, cur_i);
                }
                None => return None,
            }
        }
        _ => {
            text_end = get_oneline_comment_end(chars, text_start);
        }
    };

    Some((text_start, text_end, ml_level))
}

#[test]
fn test_multiline_string_level() {
    let mystr = String::from("=[");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_multiline_string_level(&mut iter, 0), 1);

    let mystr = String::from("===[");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_multiline_string_level(&mut iter, 0), 3);

    let mystr = String::from("[");
    let mut iter = mystr.char_indices().peekable();
    assert_eq!(get_multiline_string_level(&mut iter, 0), 0);

    let mystr = String::from(" [");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    assert_eq!(get_multiline_string_level(&mut iter, 1), 0);
}

#[test]
fn test_multiline_string_end() {
    let mystr = String::from("--[[123]]");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    iter.next();
    iter.next();
    assert_eq!(get_multiline_string_end(&mut iter, 0, 4), 7);

    let mystr = String::from("--[[]]");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    iter.next();
    iter.next();
    assert_eq!(get_multiline_string_end(&mut iter, 0, 4), 4);
}

#[test]
fn test_comment_start_end_and_type() {
    let mystr = String::from("--\n");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_end_and_type(&mut iter, 2), Some((2, 2, None)));

    let mystr = String::from("--123\n");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_end_and_type(&mut iter, 2), Some((2, 5, None)));

    let mystr = String::from("--[[]]");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_end_and_type(&mut iter, 2), Some((4, 4, Some(0))));

    let mystr = String::from("--[[123]]");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_end_and_type(&mut iter, 2), Some((4, 7, Some(0))));

    let mystr = String::from("--[=[]=]");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_end_and_type(&mut iter, 2), Some((5, 5, Some(1))));

    let mystr = String::from("--[===[123]===]");
    let mut iter = mystr.char_indices().peekable();
    iter.next();
    iter.next();
    assert_eq!(get_comment_start_end_and_type(&mut iter, 2), Some((7, 10, Some(3))));
}
