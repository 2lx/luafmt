use std::fmt;

#[macro_export]
macro_rules! cfg_write_helper {
    ($wrt:expr, $cfg:expr, $arg:ident) => {
        $arg.configured_write($wrt, $cfg)
    };
    ($wrt:expr, $cfg:expr, $arg:literal) => {
        write!($wrt, $arg)
    };
    ($wrt:expr, $cfg:expr, $arg:expr) => {
        $arg.configured_write($wrt, $cfg)
    };
}

#[macro_export]
macro_rules! cfg_write {
    ($wrt:expr, $cfg:expr, $($arg:expr),+) => {{
        $( cfg_write_helper!($wrt, $cfg, $arg)?; )+
        Ok(())
    }};
}

pub struct Config {
    pub indent_width: usize,
}

pub trait ConfiguredWrite {
    fn configured_write(&self, f: &mut dyn fmt::Write, config: &Config) -> fmt::Result;
}
