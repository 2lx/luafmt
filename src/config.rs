use std::fmt;

pub struct Config {
    pub indent_width: usize,
}

pub trait ConfiguredWrite {
    fn cfg_write(&self, f: &mut dyn fmt::Write, config: &Config) -> fmt::Result;
}
