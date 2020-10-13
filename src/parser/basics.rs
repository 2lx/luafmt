use std::fmt;
use crate::config::{Config, ConfiguredWrite};

#[derive(Debug)]
pub struct Loc(pub usize, pub usize);

#[derive(Debug)]
pub struct Str(pub &'static str);

impl ConfiguredWrite for Str {
    fn configured_write(&self, f: &mut dyn fmt::Write, _cfg: &Config, _buf: &str) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
