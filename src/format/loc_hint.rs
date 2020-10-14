use std::fmt;
use crate::config::{Config, ConfiguredWrite};
use crate::parser::basics::{Loc};
use crate::parser::parse_comment;

pub struct LocHint<'a, 'b>(pub &'a Loc, pub &'b str);
pub struct SpaceLocHint<'a, 'b>(pub &'a Loc, pub &'b str);

impl ConfiguredWrite for LocHint<'_, '_> {
    fn configured_write(&self, f: &mut dyn fmt::Write, cfg: &Config, buf: &str) -> fmt::Result {
        let comments_buffer = &buf[self.0.0..self.0.1];
        match parse_comment(comments_buffer) {
            Ok(node_tree) => {
                let mut comments_output = String::new();
                match node_tree.configured_write(&mut comments_output, cfg, comments_buffer) {
                    Ok(_) => {
                        if cfg.normalize_ws == Some(true) {
                            if comments_output.is_empty() {
                                write!(f, "{}", self.1)?;
                                return Ok(());
                            } else {
                                let prefix = match comments_output.chars().next().unwrap() {
                                    '-' => " ",
                                    _ => "",
                                };
                                let suffix = match comments_output.chars().last().unwrap() {
                                    ']' => " ",
                                    _ => "",
                                };
                                write!(f, "{}{}{}", prefix, comments_output, suffix)?;
                            }
                        } else {
                            write!(f, "{}", comments_output)?;
                        }
                    }
                    Err(err) => return Err(err),
                };
            }
            _ => return Err(std::fmt::Error),
        }

        Ok(())
    }
}

impl ConfiguredWrite for SpaceLocHint<'_, '_> {
    fn configured_write(&self, f: &mut dyn fmt::Write, cfg: &Config, buf: &str) -> fmt::Result {
        if cfg.normalize_ws == Some(true) {
            write!(f, "{}", self.1)?;
            return Ok(());
        }

        write!(f, "{}", &buf[self.0.0..self.0.1])
    }
}

