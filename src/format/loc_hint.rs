use std::fmt;
use crate::config::{Config, ConfiguredWrite};
use crate::parser::basics::{Loc};

pub struct LocHint<'a, 'b>(pub &'a Loc, pub &'b str);

impl ConfiguredWrite for LocHint<'_, '_> {
    fn configured_write(&self, f: &mut dyn fmt::Write, cfg: &Config, buf: &str) -> fmt::Result {
        if cfg.remove_comments == Some(true) {
            return write!(f, "{}", self.1)
        }

        if cfg.normalize_ws != Some(true) {
            return write!(f, "{}", &buf[self.0.0..self.0.1])
        }

        let trimmed = &buf[self.0.0..self.0.1].trim_matches(' ');
        if trimmed.len() > 0 {
            let prefix = match trimmed.chars().next().unwrap() {
                '-' => " ",
                _ => "",
            };
            let suffix = match trimmed.chars().last().unwrap() {
                '\n' => "",
                _ => " ",
            };

            write!(f, "{}{}{}", prefix, trimmed, suffix)?;
        } else {
            write!(f, "{}", self.1)?;
        }
        Ok(())
    }
}

