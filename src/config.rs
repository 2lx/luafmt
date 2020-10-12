use std::fmt;

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

pub struct Config {
    pub indent_str: Option<&'static str>,
    pub remove_comments: Option<bool>,
    pub normalize_ws: Option<bool>,
}

impl Config {
    pub const fn default() -> Self {
        Config { remove_comments: None, indent_str: None, normalize_ws: None }
    }

    pub const fn default_no_comments() -> Self {
        Config { remove_comments: Some(true), ..Config::default() }
    }

    pub const fn default_normalize_ws() -> Self {
        Config { normalize_ws: Some(true), ..Config::default() }
    }
}

pub trait ConfiguredWrite {
    fn configured_write(&self, f: &mut dyn fmt::Write, config: &Config, buf: &str) -> fmt::Result;
}
