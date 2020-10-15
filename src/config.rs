use std::fmt;
use std::fmt::Debug;

#[macro_export]
macro_rules! cfg_write_helper {
    ($wrt:expr, $cfg:expr, $buf:expr, $arg:literal) => {
        write!($wrt, $arg)
    };
    ($wrt:expr, $cfg:expr, $buf:expr, $arg:expr) => {
        $arg.configured_write($wrt, $cfg, $buf)
    };
}

#[macro_export]
macro_rules! cfg_write {
    ($wrt:expr, $cfg:expr, $buf:expr, $($arg:expr),+) => {{
        $( cfg_write_helper!($wrt, $cfg, $buf, $arg)?; )+
        Ok(())
    }};
}

#[derive(Debug)]
pub struct Config {
    pub inplace: Option<bool>,
    pub recursive: Option<bool>,

    pub field_separator: Option<String>,
    // pub indent_str: Option<String>,
    pub normalize_ws: Option<bool>,
    pub remove_comments: Option<bool>,
    pub remove_newlines: Option<bool>,
    pub trailing_field_separator: Option<bool>,
}

impl Config {
    pub const fn default() -> Self {
        Config {
            inplace: None,
            recursive: None,

            field_separator: None,
            // indent_str: None,
            normalize_ws: None,
            remove_comments: None,
            remove_newlines: None,
            trailing_field_separator: None,
        }
    }

    pub fn set(&mut self, option_name: &str, value_str: &str) {
        macro_rules! set_param_value_as {
            ($field:expr, $type:ty) => {
                match value_str.parse::<$type>() {
                    Ok(value) => $field = Some(value),
                    _ => eprintln!("Invalid config `{}` option value `{}`", option_name, value_str),
                }
            }
        }

        match option_name {
            "inplace" => set_param_value_as!(self.inplace, bool),
            "recursive" => set_param_value_as!(self.recursive, bool),

            "field_separator" => set_param_value_as!(self.field_separator, String),
            "normalize_ws" => set_param_value_as!(self.normalize_ws, bool),
            "remove_comments" => set_param_value_as!(self.remove_comments, bool),
            "remove_newlines" => set_param_value_as!(self.remove_newlines, bool),
            "trailing_field_separator" => set_param_value_as!(self.trailing_field_separator, bool),
            _ => eprintln!("Invalid option name `{}`", option_name),
        };
    }
}

pub trait ConfiguredWrite {
    fn configured_write(&self, f: &mut dyn fmt::Write, config: &Config, buf: &str) -> fmt::Result;
}
