use std::fmt::Write;

use crate::config::*;

#[derive(Debug, Clone)]
pub struct Loc(pub usize, pub usize);

impl ConfiguredWrite for Loc {
    fn configured_write(&self, f: &mut String, _cfg: &Config, buf: &str, state: &mut State) -> std::fmt::Result {
        write!(f, "{}", self.substr(buf, state, 0))
    }
}

impl Loc {
    pub fn substr<'a>(&self, buf: &'a str, state: &State, offset: usize) -> &'a str {
        let byte_offset_opt = state.chars_to_bytes.get(&offset);
        let from_opt = state.chars_to_bytes.get(&(self.0 + offset));
        let to_opt = state.chars_to_bytes.get(&(self.1 + offset));

        match (from_opt, to_opt, byte_offset_opt) {
            (Some(&from), Some(&to), Some(&byte_offset)) if from >= byte_offset && to >= byte_offset => {
                &buf[from - byte_offset..to - byte_offset]
            }
            _ => {
                // println!("ERROR HERE: ({}, {}), ({}, {})", self.0, self.1, self.0 + offset, self.1 + offset);
                ""
            }
        }
    }
}

#[derive(Debug)]
pub struct Str<'a>(pub &'a str);

impl ConfiguredWrite for Str<'_> {
    fn configured_write(&self, f: &mut String, _cfg: &Config, _buf: &str, _state: &mut State) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
