use crate::config::*;
use std::fmt;

pub struct IndentHint(pub isize);

impl ConfiguredWrite for IndentHint {
    fn configured_write(&self, _: &mut dyn fmt::Write, _: &Config, _: &str, state: &mut State) -> fmt::Result {
        state.indent_level += self.0;

        Ok(())
    }
}
