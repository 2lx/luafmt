use std::fmt::Write;
use crate::config::*;

#[derive(Debug, Clone)]
pub struct Loc(pub usize, pub usize);

#[derive(Debug)]
pub struct Str<'a>(pub &'a str);

impl ConfiguredWrite for Str<'_> {
    fn configured_write(&self, f: &mut String, _cfg: &Config, _buf: &str, _state: &mut State) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
