use std::fmt;
use std::fmt::Debug;

#[macro_export]
macro_rules! cfg_write_helper {
    ($wrt:expr, $cfg:expr, $buf:expr, $state:expr, $arg:literal) => {
        write!($wrt, $arg)
    };
    ($wrt:expr, $cfg:expr, $buf:expr, $state:expr, $arg:expr) => {
        $arg.configured_write($wrt, $cfg, $buf, $state)
    };
}

#[macro_export]
macro_rules! cfg_write {
    ($wrt:expr, $cfg:expr, $buf:expr, $state: expr, $($arg:expr),+) => {{
        $( cfg_write_helper!($wrt, $cfg, $buf, $state, $arg)?; )+
        Ok(())
    }};
}

pub trait ConfiguredWrite {
    fn configured_write(&self, f: &mut dyn fmt::Write, config: &Config, buf: &str, state: &mut State) -> fmt::Result;
}

#[derive(Debug)]
pub struct Config {
    pub inplace: Option<bool>,
    pub recursive: Option<bool>,

    // comments
    pub hint_after_multiline_comment: Option<String>,
    pub hint_after_multiline_comment_text: Option<String>,
    pub hint_before_comment: Option<String>,
    pub hint_before_multiline_comment_text: Option<String>,
    pub hint_before_oneline_comment_text: Option<String>,
    pub remove_comments: Option<bool>,
    pub remove_newlines: Option<bool>,
    pub remove_spaces_between_tokens: Option<bool>,
    pub replace_zero_spaces_with_hint: Option<bool>,

    // indent
    pub indentation_string: Option<String>,
    // pub indent_comments: Option<bool>,
    pub indent_every_statement: Option<bool>,
    pub do_end_indent_format: Option<usize>,
    pub for_indent_format: Option<usize>,
    pub function_indent_format: Option<usize>,
    pub if_indent_format: Option<usize>,
    pub repeat_until_indent_format: Option<usize>,
    pub while_indent_format: Option<usize>,

    // other
    // replace_tabs_with: Option<String>,
    pub field_separator: Option<String>,
    pub write_trailing_field_separator: Option<bool>,
}

impl Config {
    pub const fn default() -> Self {
        Config {
            inplace: None,
            recursive: None,

            // comments
            hint_after_multiline_comment: None,
            hint_after_multiline_comment_text: None,
            hint_before_comment: None,
            hint_before_multiline_comment_text: None,
            hint_before_oneline_comment_text: None,
            remove_comments: None,
            remove_newlines: None,
            remove_spaces_between_tokens: None,
            replace_zero_spaces_with_hint: None,

            // indent
            indentation_string: None,
            indent_every_statement: None,
            // indent_comments: None,
            do_end_indent_format: None,
            for_indent_format: None,
            function_indent_format: None,
            if_indent_format: None,
            repeat_until_indent_format: None,
            while_indent_format: None,

            // other
            field_separator: None,
            write_trailing_field_separator: None,
        }
    }

    pub fn set(&mut self, option_name: &str, value_str: &str) {
        macro_rules! set_param_value_as {
            ($field:expr, $type:ty) => {
                match value_str.parse::<$type>() {
                    Ok(value) => $field = Some(value),
                    _ => eprintln!("Invalid `{}` option value `{}`", option_name, value_str),
                }
            };
        }

        match option_name {
            "inplace" => set_param_value_as!(self.inplace, bool),
            "recursive" => set_param_value_as!(self.recursive, bool),

            // comments
            "hint_after_multiline_comment" => set_param_value_as!(self.hint_after_multiline_comment, String),
            "hint_after_multiline_comment_text" => set_param_value_as!(self.hint_after_multiline_comment_text, String),
            "hint_before_comment" => set_param_value_as!(self.hint_before_comment, String),
            "hint_before_multiline_comment_text" => {
                set_param_value_as!(self.hint_before_multiline_comment_text, String)
            }
            "hint_before_oneline_comment_text" => set_param_value_as!(self.hint_before_oneline_comment_text, String),
            "remove_comments" => set_param_value_as!(self.remove_comments, bool),
            "remove_newlines" => set_param_value_as!(self.remove_newlines, bool),
            "remove_spaces_between_tokens" => set_param_value_as!(self.remove_spaces_between_tokens, bool),
            "replace_zero_spaces_with_hint" => set_param_value_as!(self.replace_zero_spaces_with_hint, bool),

            // indent
            "indentation_string" => set_param_value_as!(self.indentation_string, String),
            "indent_every_statement" => set_param_value_as!(self.indent_every_statement, bool),
            "do_end_indent_format" => set_param_value_as!(self.do_end_indent_format, usize),
            "for_indent_format" => set_param_value_as!(self.for_indent_format, usize),
            "function_indent_format" => set_param_value_as!(self.function_indent_format, usize),
            "if_indent_format" => set_param_value_as!(self.if_indent_format, usize),
            "repeat_until_indent_format" => set_param_value_as!(self.repeat_until_indent_format, usize),
            "while_indent_format" => set_param_value_as!(self.while_indent_format, usize),

            // other
            "field_separator" => set_param_value_as!(self.field_separator, String),
            "write_trailing_field_separator" => set_param_value_as!(self.write_trailing_field_separator, bool),
            _ => eprintln!("Invalid option name `{}`", option_name),
        };
    }
}

#[derive(Debug)]
pub struct State {
    // pub static_indent: String,
    // pub static_indent_counter: isize,
    pub indent_level: isize,
}

impl State {
    pub const fn default() -> Self {
        State {
            // static_indent: String::new(),
            // static_indent_counter: 0,
            indent_level: 0,
        }
    }
}
