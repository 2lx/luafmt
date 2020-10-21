use crate::parser;
use std::fmt;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;

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
    fn configured_write(&self, f: &mut String, config: &Config, buf: &str, state: &mut State) -> std::fmt::Result;
}

#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    // comments
    pub hint_after_multiline_comment: Option<String>,
    pub hint_after_multiline_comment_text: Option<String>,
    pub hint_before_comment: Option<String>,
    pub hint_before_multiline_comment_text: Option<String>,
    pub hint_before_oneline_comment_text: Option<String>,
    pub remove_comments: Option<bool>,
    pub remove_single_newlines: Option<bool>,
    pub remove_all_newlines: Option<bool>,
    pub remove_spaces_between_tokens: Option<bool>,
    pub replace_zero_spaces_with_hint: Option<bool>,

    // indent
    pub indentation_string: Option<String>,
    pub indent_oneline_comments: Option<bool>,
    pub indent_multiline_comments: Option<bool>,
    pub indent_first_oneline_comment: Option<bool>,
    pub indent_first_multiline_comment: Option<bool>,
    pub indent_every_statement: Option<bool>,
    pub do_end_indent_format: Option<usize>,
    pub for_indent_format: Option<usize>,
    pub function_indent_format: Option<usize>,
    pub if_indent_format: Option<usize>,
    pub repeat_until_indent_format: Option<usize>,
    pub table_indent_format: Option<usize>,
    pub while_do_indent_format: Option<usize>,
    pub binary_op_indent_format: Option<usize>,

    // other
    // replace_tabs_with_spaces: Option<String>,
    // tabs_as_spaces_count
    pub hint_table_constructor: Option<String>,
    pub field_separator: Option<String>,
    pub write_trailing_field_separator: Option<bool>,
    pub max_width: Option<usize>,
    pub enable_oneline_binary_op: Option<bool>,
}

impl Config {
    pub const fn default() -> Self {
        Config {
            // comments
            hint_after_multiline_comment: None,
            hint_after_multiline_comment_text: None,
            hint_before_comment: None,
            hint_before_multiline_comment_text: None,
            hint_before_oneline_comment_text: None,
            remove_comments: None,
            remove_single_newlines: None,
            remove_all_newlines: None,
            remove_spaces_between_tokens: None,
            replace_zero_spaces_with_hint: None,

            // indent
            indentation_string: None,
            indent_every_statement: None,
            indent_oneline_comments: None,
            indent_multiline_comments: None,
            indent_first_oneline_comment: None,
            indent_first_multiline_comment: None,
            do_end_indent_format: None,
            for_indent_format: None,
            function_indent_format: None,
            if_indent_format: None,
            repeat_until_indent_format: None,
            table_indent_format: None,
            while_do_indent_format: None,
            binary_op_indent_format: None,

            // other
            hint_table_constructor: None,
            field_separator: None,
            write_trailing_field_separator: None,
            max_width: None,
            enable_oneline_binary_op: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        *self == Config::default()
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
            // comments
            "hint_after_multiline_comment" => set_param_value_as!(self.hint_after_multiline_comment, String),
            "hint_after_multiline_comment_text" => set_param_value_as!(self.hint_after_multiline_comment_text, String),
            "hint_before_comment" => set_param_value_as!(self.hint_before_comment, String),
            "hint_before_multiline_comment_text" => {
                set_param_value_as!(self.hint_before_multiline_comment_text, String)
            }
            "hint_before_oneline_comment_text" => set_param_value_as!(self.hint_before_oneline_comment_text, String),
            "remove_comments" => set_param_value_as!(self.remove_comments, bool),
            "remove_single_newlines" => set_param_value_as!(self.remove_single_newlines, bool),
            "remove_all_newlines" => set_param_value_as!(self.remove_all_newlines, bool),
            "remove_spaces_between_tokens" => set_param_value_as!(self.remove_spaces_between_tokens, bool),
            "replace_zero_spaces_with_hint" => set_param_value_as!(self.replace_zero_spaces_with_hint, bool),

            // indent
            "indentation_string" => set_param_value_as!(self.indentation_string, String),
            "indent_every_statement" => set_param_value_as!(self.indent_every_statement, bool),
            "indent_oneline_comments" => set_param_value_as!(self.indent_oneline_comments, bool),
            "indent_multiline_comments" => set_param_value_as!(self.indent_multiline_comments, bool),
            "indent_first_oneline_comment" => set_param_value_as!(self.indent_first_oneline_comment, bool),
            "indent_first_multiline_comment" => set_param_value_as!(self.indent_first_multiline_comment, bool),
            "do_end_indent_format" => set_param_value_as!(self.do_end_indent_format, usize),
            "for_indent_format" => set_param_value_as!(self.for_indent_format, usize),
            "function_indent_format" => set_param_value_as!(self.function_indent_format, usize),
            "if_indent_format" => set_param_value_as!(self.if_indent_format, usize),
            "repeat_until_indent_format" => set_param_value_as!(self.repeat_until_indent_format, usize),
            "table_indent_format" => set_param_value_as!(self.table_indent_format, usize),
            "while_do_indent_format" => set_param_value_as!(self.while_do_indent_format, usize),
            "binary_op_indent_format" => set_param_value_as!(self.binary_op_indent_format, usize),

            // other
            "hint_table_constructor" => set_param_value_as!(self.hint_table_constructor, String),
            "field_separator" => set_param_value_as!(self.field_separator, String),
            "write_trailing_field_separator" => set_param_value_as!(self.write_trailing_field_separator, bool),
            "max_width" => set_param_value_as!(self.max_width, usize),
            "enable_oneline_binary_op" => set_param_value_as!(self.enable_oneline_binary_op, bool),
            _ => eprintln!("Invalid option name `{}`", option_name),
        }
    }

    fn load_options_from_config(&mut self, node: &parser::lua_ast::Node) {
        use parser::lua_ast::Node::*;
        match node {
            VarsExprs(_, _, varlist, exprlist) => match (&**varlist, &**exprlist) {
                (VarList(_, vars), ExpList(_, exprs)) => match (vars.iter().next(), exprs.iter().next()) {
                    (Some((_, var, _, _)), Some((_, expr, _, _))) => match (var, expr) {
                        (Name(_, str_name), Numeral(_, str_expr))
                        | (Name(_, str_name), NormalStringLiteral(_, str_expr))
                        | (Name(_, str_name), CharStringLiteral(_, str_expr)) => self.set(str_name, str_expr),

                        (Name(_, str_name), True(..)) => self.set(str_name, "true"),
                        (Name(_, str_name), False(..)) => self.set(str_name, "false"),
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            StatementList(_, stts) => {
                for (_, node) in stts {
                    self.load_options_from_config(node);
                }
            }
            _ => {}
        }
    }

    pub fn load_from_file(file_path: &PathBuf) -> Result<Self, String> {
        let content = fs::read_to_string(file_path)
            .expect(&format!("An error occured while reading config file `{}`", file_path.display()));

        let mut cfg = Config::default();

        use parser::lua_ast::Node::*;
        match parser::parse_lua(&content) {
            Ok(node_tree) => {
                match node_tree {
                    Chunk(_, node, _) => cfg.load_options_from_config(&node),
                    SheBangChunk(_, _, _, node, _) => cfg.load_options_from_config(&node),
                    _ => {}
                };
            }
            Err(err) => {
                return Err(format!("An error occured while parsing config file `{}`: {}", file_path.display(), err))
            }
        }

        Ok(cfg)
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! print_opt {
            ($field:expr, $name:literal) => {
                if $field.is_some() {
                    write!(f, "\t{}: {:?},\n", $name, $field.as_ref().unwrap())?;
                }
            };
        }

        write!(f, "{{\n")?;

        // comments
        print_opt!(self.hint_after_multiline_comment, "hint_after_multiline_comment");
        print_opt!(self.hint_after_multiline_comment_text, "hint_after_multiline_comment_text");
        print_opt!(self.hint_before_comment, "hint_before_comment");
        print_opt!(self.hint_before_multiline_comment_text, "hint_before_multiline_comment_text");
        print_opt!(self.hint_before_oneline_comment_text, "hint_before_oneline_comment_text");
        print_opt!(self.remove_comments, "remove_comments");
        print_opt!(self.remove_single_newlines, "remove_single_newlines");
        print_opt!(self.remove_all_newlines, "remove_all_newlines");
        print_opt!(self.remove_spaces_between_tokens, "remove_spaces_between_tokens");
        print_opt!(self.replace_zero_spaces_with_hint, "replace_zero_spaces_with_hint");

        // indent
        print_opt!(self.indentation_string, "indentation_string");
        print_opt!(self.indent_every_statement, "indent_every_statement");
        print_opt!(self.indent_oneline_comments, "indent_oneline_comments");
        print_opt!(self.indent_multiline_comments, "indent_multiline_comments");
        print_opt!(self.indent_first_oneline_comment, "indent_first_oneline_comment");
        print_opt!(self.indent_first_multiline_comment, "indent_first_multiline_comment");
        print_opt!(self.do_end_indent_format, "do_end_indent_format");
        print_opt!(self.for_indent_format, "for_indent_format");
        print_opt!(self.function_indent_format, "function_indent_format");
        print_opt!(self.if_indent_format, "if_indent_format");
        print_opt!(self.repeat_until_indent_format, "repeat_until_indent_format");
        print_opt!(self.table_indent_format, "table_indent_format");
        print_opt!(self.while_do_indent_format, "while_do_indent_format");
        print_opt!(self.binary_op_indent_format, "binary_op_indent_format");

        // other
        print_opt!(self.hint_table_constructor, "hint_table_constructor");
        print_opt!(self.field_separator, "field_separator");
        print_opt!(self.write_trailing_field_separator, "write_trailing_field_separator");
        print_opt!(self.max_width, "max_width");
        print_opt!(self.enable_oneline_binary_op, "enable_oneline_binary_op");

        write!(f, "}}")?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct State {
    pub indent_level: isize,
}

impl State {
    pub const fn default() -> Self {
        State { indent_level: 0 }
    }
}
