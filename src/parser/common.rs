use std::fmt;
use crate::config::*;

#[derive(Debug)]
pub struct Loc(pub usize, pub usize);

#[derive(Debug)]
pub struct Str<'a>(pub &'a str);

impl ConfiguredWrite for Str<'_> {
    fn configured_write(&self, f: &mut dyn fmt::Write, _cfg: &Config, _buf: &str, _state: &State) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
