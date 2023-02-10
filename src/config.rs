use regex::Regex;
use std::fmt;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::parser;

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
    pub line_range: Option<(usize, usize)>,
    pub fmt: FormatOpts,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FormatOpts {
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
    pub write_newline_at_multiline_table: Option<bool>,
    pub write_newline_at_explist_multiline_table: Option<bool>,

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
    pub force_single_line_binary_op: Option<bool>,
    pub force_single_line_table: Option<bool>,
    pub force_single_line_iv_table_field: Option<bool>,
    pub force_single_line_kv_table_field: Option<bool>,
    pub force_single_line_if: Option<bool>,
    pub force_single_line_top_level_function: Option<bool>,
    pub force_single_line_scoped_function: Option<bool>,
    pub force_single_line_var_suffix: Option<bool>,
    pub force_single_line_exp_list: Option<bool>,
}

impl FormatOpts {
    pub const fn default() -> Self {
        FormatOpts {
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
            write_newline_at_multiline_table: None,
            write_newline_at_explist_multiline_table: None,

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
            force_single_line_binary_op: None,
            force_single_line_table: None,
            force_single_line_iv_table_field: None,
            force_single_line_kv_table_field: None,
            force_single_line_if: None,
            force_single_line_top_level_function: None,
            force_single_line_scoped_function: None,
            force_single_line_var_suffix: None,
            force_single_line_exp_list: None,
        }
    }
}

impl Config {
    pub const fn default() -> Self {
        Config { line_range: None, fmt: FormatOpts::default() }
    }

    pub fn has_empty_format(&self) -> bool {
        self.fmt == FormatOpts::default()
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

        let re_lines_opt = Regex::new(r"^([0-9]+):([0-9]+)$").unwrap();

        match option_name {
            // hint
            "replace_zero_spaces_with_hint" => set_param_value_as!(self.fmt.replace_zero_spaces_with_hint, bool),
            "hint_after_multiline_comment" => set_param_value_as!(self.fmt.hint_after_multiline_comment, String),
            "hint_after_multiline_comment_text" => {
                set_param_value_as!(self.fmt.hint_after_multiline_comment_text, String)
            }
            "hint_before_comment" => set_param_value_as!(self.fmt.hint_before_comment, String),
            "hint_before_multiline_comment_text" => {
                set_param_value_as!(self.fmt.hint_before_multiline_comment_text, String)
            }
            "hint_before_oneline_comment_text" => {
                set_param_value_as!(self.fmt.hint_before_oneline_comment_text, String)
            }
            "hint_table_constructor" => set_param_value_as!(self.fmt.hint_table_constructor, String),

            "remove_comments" => set_param_value_as!(self.fmt.remove_comments, bool),
            "remove_single_newlines" => set_param_value_as!(self.fmt.remove_single_newlines, bool),
            "remove_all_newlines" => set_param_value_as!(self.fmt.remove_all_newlines, bool),
            "remove_spaces_between_tokens" => set_param_value_as!(self.fmt.remove_spaces_between_tokens, bool),
            "write_newline_at_eof" => set_param_value_as!(self.fmt.write_newline_at_eof, bool),
            "write_newline_at_multiline_table" => set_param_value_as!(self.fmt.write_newline_at_multiline_table, bool),
            "write_newline_at_explist_multiline_table" => set_param_value_as!(self.fmt.write_newline_at_explist_multiline_table, bool),

            // indentation
            "indentation_string" => set_param_value_as!(self.fmt.indentation_string, String),
            "indent_var_suffix" => set_param_value_as!(self.fmt.indent_var_suffix, bool),
            "indent_one_line_var_suffix" => set_param_value_as!(self.fmt.indent_one_line_var_suffix, bool),
            "indent_exp_list" => set_param_value_as!(self.fmt.indent_exp_list, bool),
            "indent_one_line_exp_list" => set_param_value_as!(self.fmt.indent_one_line_exp_list, bool),

            // format
            "newline_format_first_oneline_comment" => {
                set_param_value_as!(self.fmt.newline_format_first_oneline_comment, usize)
            }
            "newline_format_first_multiline_comment" => {
                set_param_value_as!(self.fmt.newline_format_first_multiline_comment, usize)
            }
            "newline_format_oneline_comment" => set_param_value_as!(self.fmt.newline_format_oneline_comment, usize),
            "newline_format_multiline_comment" => set_param_value_as!(self.fmt.newline_format_multiline_comment, usize),
            "newline_format_statement" => set_param_value_as!(self.fmt.newline_format_statement, usize),
            "newline_format_do_end" => set_param_value_as!(self.fmt.newline_format_do_end, usize),
            "newline_format_for" => set_param_value_as!(self.fmt.newline_format_for, usize),
            "newline_format_function" => set_param_value_as!(self.fmt.newline_format_function, usize),
            "newline_format_if" => set_param_value_as!(self.fmt.newline_format_if, usize),
            "newline_format_repeat_until" => set_param_value_as!(self.fmt.newline_format_repeat_until, usize),
            "newline_format_table_constructor" => set_param_value_as!(self.fmt.newline_format_table_constructor, usize),
            "newline_format_table_field" => set_param_value_as!(self.fmt.newline_format_table_field, usize),
            "newline_format_while" => set_param_value_as!(self.fmt.newline_format_while, usize),
            "newline_format_binary_op" => set_param_value_as!(self.fmt.newline_format_binary_op, usize),
            "newline_format_var_suffix" => set_param_value_as!(self.fmt.newline_format_var_suffix, usize),
            "newline_format_exp_list" => set_param_value_as!(self.fmt.newline_format_exp_list, usize),
            "newline_format_exp_list_first" => set_param_value_as!(self.fmt.newline_format_exp_list_first, usize),

            // other
            "field_separator" => set_param_value_as!(self.fmt.field_separator, String),
            "write_trailing_field_separator" => set_param_value_as!(self.fmt.write_trailing_field_separator, bool),
            "convert_charstring_to_normalstring" => {
                set_param_value_as!(self.fmt.convert_charstring_to_normalstring, bool)
            }

            // oneline
            "max_width" => set_param_value_as!(self.fmt.max_width, usize),
            "force_single_line_binary_op" => set_param_value_as!(self.fmt.force_single_line_binary_op, bool),
            "force_single_line_table" => set_param_value_as!(self.fmt.force_single_line_table, bool),
            "force_single_line_iv_table_field" => set_param_value_as!(self.fmt.force_single_line_iv_table_field, bool),
            "force_single_line_kv_table_field" => set_param_value_as!(self.fmt.force_single_line_kv_table_field, bool),
            "force_single_line_if" => set_param_value_as!(self.fmt.force_single_line_if, bool),
            "force_single_line_top_level_function" => {
                set_param_value_as!(self.fmt.force_single_line_top_level_function, bool)
            }
            "force_single_line_scoped_function" => set_param_value_as!(self.fmt.force_single_line_scoped_function, bool),
            "force_single_line_var_suffix" => set_param_value_as!(self.fmt.force_single_line_var_suffix, bool),
            "force_single_line_exp_list" => set_param_value_as!(self.fmt.force_single_line_exp_list, bool),

            "line_range" => match re_lines_opt.captures_iter(value_str).next() {
                Some(cap) => match (cap[1].parse(), cap[2].parse()) {
                    (Ok(l1), Ok(l2)) => self.line_range = Some((l1, l2)),
                    _ => eprintln!("Invalid `{}` option value `{}`", option_name, value_str),
                },
                _ => eprintln!("Invalid `{}` option value `{}`", option_name, value_str),
            },

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

    pub fn reload_format_from_file(&self, file_path: &PathBuf) -> Result<Self, String> {
        let content = fs::read_to_string(file_path)
            .expect(&format!("An error occured while reading config file `{}`", file_path.display()));

        let mut cfg = self.clone();

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
        print_opt!(self.fmt.replace_zero_spaces_with_hint, "replace_zero_spaces_with_hint");
        print_opt!(self.fmt.hint_after_multiline_comment, "hint_after_multiline_comment");
        print_opt!(self.fmt.hint_after_multiline_comment_text, "hint_after_multiline_comment_text");
        print_opt!(self.fmt.hint_before_comment, "hint_before_comment");
        print_opt!(self.fmt.hint_before_multiline_comment_text, "hint_before_multiline_comment_text");
        print_opt!(self.fmt.hint_before_oneline_comment_text, "hint_before_oneline_comment_text");
        print_opt!(self.fmt.hint_table_constructor, "hint_table_constructor");

        print_opt!(self.fmt.remove_comments, "remove_comments");
        print_opt!(self.fmt.remove_single_newlines, "remove_single_newlines");
        print_opt!(self.fmt.remove_all_newlines, "remove_all_newlines");
        print_opt!(self.fmt.remove_spaces_between_tokens, "remove_spaces_between_tokens");
        print_opt!(self.fmt.write_newline_at_eof, "write_newline_at_eof");
        print_opt!(self.fmt.write_newline_at_multiline_table, "write_newline_at_multiline_table");
        print_opt!(self.fmt.write_newline_at_explist_multiline_table, "write_newline_at_explist_multiline_table");

        // indentation
        print_opt!(self.fmt.indentation_string, "indentation_string");
        print_opt!(self.fmt.indent_var_suffix, "indent_var_suffix");
        print_opt!(self.fmt.indent_one_line_var_suffix, "indent_one_line_var_suffix");
        print_opt!(self.fmt.indent_exp_list, "indent_exp_list");
        print_opt!(self.fmt.indent_one_line_exp_list, "indent_one_line_exp_list");

        // format
        print_opt!(self.fmt.newline_format_first_oneline_comment, "newline_format_first_oneline_comment");
        print_opt!(self.fmt.newline_format_first_multiline_comment, "newline_format_first_multiline_comment");
        print_opt!(self.fmt.newline_format_oneline_comment, "newline_format_oneline_comment");
        print_opt!(self.fmt.newline_format_multiline_comment, "newline_format_multiline_comment");
        print_opt!(self.fmt.newline_format_statement, "newline_format_statement");
        print_opt!(self.fmt.newline_format_do_end, "newline_format_do_end");
        print_opt!(self.fmt.newline_format_for, "newline_format_for");
        print_opt!(self.fmt.newline_format_function, "newline_format_function");
        print_opt!(self.fmt.newline_format_if, "newline_format_if");
        print_opt!(self.fmt.newline_format_repeat_until, "newline_format_repeat_until");
        print_opt!(self.fmt.newline_format_table_constructor, "newline_format_table_constructor");
        print_opt!(self.fmt.newline_format_table_field, "newline_format_table_field");
        print_opt!(self.fmt.newline_format_while, "newline_format_while");
        print_opt!(self.fmt.newline_format_binary_op, "newline_format_binary_op");
        print_opt!(self.fmt.newline_format_var_suffix, "newline_format_var_suffix");
        print_opt!(self.fmt.newline_format_exp_list, "newline_format_exp_list");
        print_opt!(self.fmt.newline_format_exp_list_first, "newline_format_exp_list_first");

        // other
        print_opt!(self.fmt.field_separator, "field_separator");
        print_opt!(self.fmt.write_trailing_field_separator, "write_trailing_field_separator");
        print_opt!(self.fmt.convert_charstring_to_normalstring, "convert_charstring_to_normalstring");

        // oneline
        print_opt!(self.fmt.max_width, "max_width");
        print_opt!(self.fmt.force_single_line_binary_op, "force_single_line_binary_op");
        print_opt!(self.fmt.force_single_line_table, "force_single_line_table");
        print_opt!(self.fmt.force_single_line_iv_table_field, "force_single_line_iv_table_field");
        print_opt!(self.fmt.force_single_line_kv_table_field, "force_single_line_kv_table_field");
        print_opt!(self.fmt.force_single_line_if, "force_single_line_if");
        print_opt!(self.fmt.force_single_line_top_level_function, "force_single_line_top_level_function");
        print_opt!(self.fmt.force_single_line_scoped_function, "force_single_line_scoped_function");
        print_opt!(self.fmt.force_single_line_var_suffix, "force_single_line_var_suffix");
        print_opt!(self.fmt.force_single_line_exp_list, "force_single_line_exp_list");

        print_opt!(self.line_range, "line_range");

        write!(f, "}}")?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub indent_level: isize,
    pub stack_indent: Vec<Option<&'static str>>,
    pub function_nested_level: isize,
    pub block_nested_level: isize,
    pub pos_range: Option<(usize, usize)>,
    pub comment_pos_range: Option<(usize, usize)>,
    pub comment_offset: usize,
    pub chars_to_bytes: HashMap::<usize, usize>,
}

impl State {
    pub fn default() -> Self {
        State {
            indent_level: 0,
            stack_indent: Vec::new(),
            function_nested_level: 0,
            block_nested_level: 0,
            pos_range: None,
            comment_pos_range: None,
            comment_offset: 0,
            chars_to_bytes: HashMap::new(),
        }
    }
}
