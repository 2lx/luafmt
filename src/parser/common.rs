use std::fmt::Write;
use std::cmp::min;

use crate::config::*;

#[derive(Debug, Clone)]
pub struct Loc(pub usize, pub usize);

impl ConfiguredWrite for Loc {
    fn configured_write(&self, f: &mut String, _cfg: &Config, buf: &str, _state: &mut State) -> std::fmt::Result {
        write!(f, "{}", self.substr(buf))
    }
}

impl Loc {
    pub fn len(&self) -> usize {
        self.1 - min(self.0, self.1)
    }

    pub fn substr(&self, buf: &str) -> String {
        buf.chars().by_ref().skip(self.0).take(self.len()).collect::<String>()
    }
}

#[derive(Debug)]
pub struct Str<'a>(pub &'a str);

impl ConfiguredWrite for Str<'_> {
    fn configured_write(&self, f: &mut String, _cfg: &Config, _buf: &str, _state: &mut State) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
