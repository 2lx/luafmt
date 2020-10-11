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
    ($wrt:expr, $cfg:expr, $buf:expr, $($arg:tt),+) => {{
        $( cfg_write_helper!($wrt, $cfg, $buf, $arg)?; )+
        Ok(())
    }};
}

pub struct Config {
    pub indent_width: usize,
    pub keep_comments: bool,
}

pub trait ConfiguredWrite {
    fn configured_write(&self, f: &mut dyn fmt::Write, config: &Config, buf: &str) -> fmt::Result;
}
