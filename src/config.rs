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
            field_separator: None,
            // indent_str: None,
            normalize_ws: None,
            remove_comments: None,
            remove_newlines: None,
            trailing_field_separator: None,
        }
    }

    pub fn set(&mut self, param_str: &str, value_str: &str) {
        match param_str {
            "remove_comments" => match value_str.parse::<bool>() {
                Ok(value) => self.remove_comments = Some(value),
                _ => eprintln!("Invalid `remove_comments` option value `{:?}`", value_str),
            },
            "remove_newlines" => match value_str.parse::<bool>() {
                Ok(value) => self.remove_newlines = Some(value),
                _ => eprintln!("Invalid `remove_newlines` option value `{:?}`", value_str),
            },
            "trailing_field_separator" => match value_str.parse::<bool>() {
                Ok(value) => self.trailing_field_separator = Some(value),
                _ => eprintln!("Invalid `trailing_field_separator` option value `{:?}`", value_str),
            },
            "field_separator" => match value_str.parse::<String>() {
                Ok(value) => self.field_separator = Some(value),
                _ => eprintln!("Invalid `field_separator` option value `{:?}`", value_str),
            },
            _ => eprintln!("Invalid option `{}`", param_str),
        };
    }
}

pub trait ConfiguredWrite {
    fn configured_write(&self, f: &mut dyn fmt::Write, config: &Config, buf: &str) -> fmt::Result;
}
