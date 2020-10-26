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
    // hint
    pub replace_zero_spaces_with_hint: Option<bool>,
    pub hint_after_multiline_comment: Option<String>,
    pub hint_after_multiline_comment_text: Option<String>,
    pub hint_before_comment: Option<String>,
    pub hint_before_multiline_comment_text: Option<String>,
    pub hint_before_oneline_comment_text: Option<String>,
    pub hint_table_constructor: Option<String>,

    pub remove_comments: Option<bool>,
    pub remove_single_newlines: Option<bool>,
    pub remove_all_newlines: Option<bool>,
    pub remove_spaces_between_tokens: Option<bool>,
    pub write_newline_at_eof: Option<bool>,

    // indentation
    pub indentation_string: Option<String>,
    pub indent_var_suffix: Option<bool>,
    pub indent_one_line_var_suffix: Option<bool>,
    pub indent_exp_list: Option<bool>,
    pub indent_one_line_exp_list: Option<bool>,

    // format
    pub newline_format_first_oneline_comment: Option<usize>,
    pub newline_format_first_multiline_comment: Option<usize>,
    pub newline_format_oneline_comment: Option<usize>,
    pub newline_format_multiline_comment: Option<usize>,
    pub newline_format_statement: Option<usize>,
    pub newline_format_do_end: Option<usize>,
    pub newline_format_for: Option<usize>,
    pub newline_format_function: Option<usize>,
    pub newline_format_if: Option<usize>,
    pub newline_format_repeat_until: Option<usize>,
    pub newline_format_table_constructor: Option<usize>,
    pub newline_format_table_field: Option<usize>,
    pub newline_format_while: Option<usize>,
    pub newline_format_binary_op: Option<usize>,
    pub newline_format_var_suffix: Option<usize>,
    pub newline_format_exp_list: Option<usize>,
    pub newline_format_exp_list_first: Option<usize>,

    // other
    // replace_tabs_with_spaces: Option<String>,
    // tabs_as_spaces_count
    pub field_separator: Option<String>,
    pub write_trailing_field_separator: Option<bool>,
    pub convert_charstring_to_normalstring: Option<bool>,

    // oneline
    pub max_width: Option<usize>,
    pub enable_oneline_binary_op: Option<bool>,
    pub enable_oneline_table_constructor: Option<bool>,
    pub enable_oneline_table_field: Option<bool>,
    pub enable_oneline_iv_table: Option<bool>,
    pub enable_oneline_if: Option<bool>,
    pub enable_oneline_top_level_function: Option<bool>,
    pub enable_oneline_scoped_function: Option<bool>,
    pub enable_oneline_var_suffix: Option<bool>,
    pub enable_oneline_exp_list: Option<bool>,
}

impl Config {
    pub const fn default() -> Self {
        Config {
            // hint
            replace_zero_spaces_with_hint: None,
            hint_after_multiline_comment: None,
            hint_after_multiline_comment_text: None,
            hint_before_comment: None,
            hint_before_multiline_comment_text: None,
            hint_before_oneline_comment_text: None,
            hint_table_constructor: None,

            remove_comments: None,
            remove_single_newlines: None,
            remove_all_newlines: None,
            remove_spaces_between_tokens: None,
            write_newline_at_eof: None,

            // indentation
            indentation_string: None,
            indent_var_suffix: None,
            indent_one_line_var_suffix: None,
            indent_exp_list: None,
            indent_one_line_exp_list: None,

            // format
            newline_format_first_oneline_comment: None,
            newline_format_first_multiline_comment: None,
            newline_format_oneline_comment: None,
            newline_format_multiline_comment: None,
            newline_format_statement: None,
            newline_format_do_end: None,
            newline_format_for: None,
            newline_format_function: None,
            newline_format_if: None,
            newline_format_repeat_until: None,
            newline_format_table_constructor: None,
            newline_format_table_field: None,
            newline_format_while: None,
            newline_format_binary_op: None,
            newline_format_var_suffix: None,
            newline_format_exp_list: None,
            newline_format_exp_list_first: None,

            // other
            field_separator: None,
            write_trailing_field_separator: None,
            convert_charstring_to_normalstring: None,

            // oneline
            max_width: None,
            enable_oneline_binary_op: None,
            enable_oneline_table_constructor: None,
            enable_oneline_table_field: None,
            enable_oneline_iv_table: None,
            enable_oneline_if: None,
            enable_oneline_top_level_function: None,
            enable_oneline_scoped_function: None,
            enable_oneline_var_suffix: None,
            enable_oneline_exp_list: None,
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
            // hint
            "replace_zero_spaces_with_hint" => set_param_value_as!(self.replace_zero_spaces_with_hint, bool),
            "hint_after_multiline_comment" => set_param_value_as!(self.hint_after_multiline_comment, String),
            "hint_after_multiline_comment_text" => set_param_value_as!(self.hint_after_multiline_comment_text, String),
            "hint_before_comment" => set_param_value_as!(self.hint_before_comment, String),
            "hint_before_multiline_comment_text" => {
                set_param_value_as!(self.hint_before_multiline_comment_text, String)
            }
            "hint_before_oneline_comment_text" => set_param_value_as!(self.hint_before_oneline_comment_text, String),
            "hint_table_constructor" => set_param_value_as!(self.hint_table_constructor, String),

            "remove_comments" => set_param_value_as!(self.remove_comments, bool),
            "remove_single_newlines" => set_param_value_as!(self.remove_single_newlines, bool),
            "remove_all_newlines" => set_param_value_as!(self.remove_all_newlines, bool),
            "remove_spaces_between_tokens" => set_param_value_as!(self.remove_spaces_between_tokens, bool),
            "write_newline_at_eof" => set_param_value_as!(self.write_newline_at_eof, bool),

            // indentation
            "indentation_string" => set_param_value_as!(self.indentation_string, String),
            "indent_var_suffix" => set_param_value_as!(self.indent_var_suffix, bool),
            "indent_one_line_var_suffix" => set_param_value_as!(self.indent_one_line_var_suffix, bool),
            "indent_exp_list" => set_param_value_as!(self.indent_exp_list, bool),
            "indent_one_line_exp_list" => set_param_value_as!(self.indent_one_line_exp_list, bool),

            // format
            "newline_format_first_oneline_comment" => {
                set_param_value_as!(self.newline_format_first_oneline_comment, usize)
            }
            "newline_format_first_multiline_comment" => {
                set_param_value_as!(self.newline_format_first_multiline_comment, usize)
            }
            "newline_format_oneline_comment" => set_param_value_as!(self.newline_format_oneline_comment, usize),
            "newline_format_multiline_comment" => set_param_value_as!(self.newline_format_multiline_comment, usize),
            "newline_format_statement" => set_param_value_as!(self.newline_format_statement, usize),
            "newline_format_do_end" => set_param_value_as!(self.newline_format_do_end, usize),
            "newline_format_for" => set_param_value_as!(self.newline_format_for, usize),
            "newline_format_function" => set_param_value_as!(self.newline_format_function, usize),
            "newline_format_if" => set_param_value_as!(self.newline_format_if, usize),
            "newline_format_repeat_until" => set_param_value_as!(self.newline_format_repeat_until, usize),
            "newline_format_table_constructor" => set_param_value_as!(self.newline_format_table_constructor, usize),
            "newline_format_table_field" => set_param_value_as!(self.newline_format_table_field, usize),
            "newline_format_while" => set_param_value_as!(self.newline_format_while, usize),
            "newline_format_binary_op" => set_param_value_as!(self.newline_format_binary_op, usize),
            "newline_format_var_suffix" => set_param_value_as!(self.newline_format_var_suffix, usize),
            "newline_format_exp_list" => set_param_value_as!(self.newline_format_exp_list, usize),
            "newline_format_exp_list_first" => set_param_value_as!(self.newline_format_exp_list_first, usize),

            // other
            "field_separator" => set_param_value_as!(self.field_separator, String),
            "write_trailing_field_separator" => set_param_value_as!(self.write_trailing_field_separator, bool),
            "convert_charstring_to_normalstring" => set_param_value_as!(self.convert_charstring_to_normalstring, bool),

            // oneline
            "max_width" => set_param_value_as!(self.max_width, usize),
            "enable_oneline_binary_op" => set_param_value_as!(self.enable_oneline_binary_op, bool),
            "enable_oneline_table_constructor" => set_param_value_as!(self.enable_oneline_table_constructor, bool),
            "enable_oneline_table_field" => set_param_value_as!(self.enable_oneline_table_field, bool),
            "enable_oneline_iv_table" => set_param_value_as!(self.enable_oneline_iv_table, bool),
            "enable_oneline_if" => set_param_value_as!(self.enable_oneline_if, bool),
            "enable_oneline_top_level_function" => set_param_value_as!(self.enable_oneline_top_level_function, bool),
            "enable_oneline_scoped_function" => set_param_value_as!(self.enable_oneline_scoped_function, bool),
            "enable_oneline_var_suffix" => set_param_value_as!(self.enable_oneline_var_suffix, bool),
            "enable_oneline_exp_list" => set_param_value_as!(self.enable_oneline_exp_list, bool),

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

        // hint
        print_opt!(self.replace_zero_spaces_with_hint, "replace_zero_spaces_with_hint");
        print_opt!(self.hint_after_multiline_comment, "hint_after_multiline_comment");
        print_opt!(self.hint_after_multiline_comment_text, "hint_after_multiline_comment_text");
        print_opt!(self.hint_before_comment, "hint_before_comment");
        print_opt!(self.hint_before_multiline_comment_text, "hint_before_multiline_comment_text");
        print_opt!(self.hint_before_oneline_comment_text, "hint_before_oneline_comment_text");
        print_opt!(self.hint_table_constructor, "hint_table_constructor");

        print_opt!(self.remove_comments, "remove_comments");
        print_opt!(self.remove_single_newlines, "remove_single_newlines");
        print_opt!(self.remove_all_newlines, "remove_all_newlines");
        print_opt!(self.remove_spaces_between_tokens, "remove_spaces_between_tokens");
        print_opt!(self.write_newline_at_eof, "write_newline_at_eof");

        // indentation
        print_opt!(self.indentation_string, "indentation_string");
        print_opt!(self.indent_var_suffix, "indent_var_suffix");
        print_opt!(self.indent_one_line_var_suffix, "indent_one_line_var_suffix");
        print_opt!(self.indent_exp_list, "indent_exp_list");
        print_opt!(self.indent_one_line_exp_list, "indent_one_line_exp_list");

        // format
        print_opt!(self.newline_format_first_oneline_comment, "newline_format_first_oneline_comment");
        print_opt!(self.newline_format_first_multiline_comment, "newline_format_first_multiline_comment");
        print_opt!(self.newline_format_oneline_comment, "newline_format_oneline_comment");
        print_opt!(self.newline_format_multiline_comment, "newline_format_multiline_comment");
        print_opt!(self.newline_format_statement, "newline_format_statement");
        print_opt!(self.newline_format_do_end, "newline_format_do_end");
        print_opt!(self.newline_format_for, "newline_format_for");
        print_opt!(self.newline_format_function, "newline_format_function");
        print_opt!(self.newline_format_if, "newline_format_if");
        print_opt!(self.newline_format_repeat_until, "newline_format_repeat_until");
        print_opt!(self.newline_format_table_constructor, "newline_format_table_constructor");
        print_opt!(self.newline_format_table_field, "newline_format_table_field");
        print_opt!(self.newline_format_while, "newline_format_while");
        print_opt!(self.newline_format_binary_op, "newline_format_binary_op");
        print_opt!(self.newline_format_var_suffix, "newline_format_var_suffix");
        print_opt!(self.newline_format_exp_list, "newline_format_exp_list");
        print_opt!(self.newline_format_exp_list_first, "newline_format_exp_list_first");

        // other
        print_opt!(self.field_separator, "field_separator");
        print_opt!(self.write_trailing_field_separator, "write_trailing_field_separator");
        print_opt!(self.convert_charstring_to_normalstring, "convert_charstring_to_normalstring");

        // oneline
        print_opt!(self.max_width, "max_width");
        print_opt!(self.enable_oneline_binary_op, "enable_oneline_binary_op");
        print_opt!(self.enable_oneline_table_constructor, "enable_oneline_table_constructor");
        print_opt!(self.enable_oneline_table_field, "enable_oneline_table_field");
        print_opt!(self.enable_oneline_iv_table, "enable_oneline_iv_table");
        print_opt!(self.enable_oneline_if, "enable_oneline_if");
        print_opt!(self.enable_oneline_top_level_function, "enable_oneline_top_level_function");
        print_opt!(self.enable_oneline_scoped_function, "enable_oneline_scoped_function");
        print_opt!(self.enable_oneline_var_suffix, "enable_oneline_var_suffix");
        print_opt!(self.enable_oneline_exp_list, "enable_oneline_exp_list");

        write!(f, "}}")?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub indent_level: isize,
    pub stack_indent: Vec<Option<&'static str>>,
    pub function_nested_level: isize,
}

impl State {
    pub const fn default() -> Self {
        State { indent_level: 0, stack_indent: Vec::new(), function_nested_level: 0 }
    }
}
