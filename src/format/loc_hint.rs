use std::fmt;
use crate::config::{Config, ConfiguredWrite};
use crate::parser::basics::*;
use crate::parser::parse_comment;

pub struct CommentLocHint<'a, 'b>(pub &'a Loc, pub &'b str);
pub struct SpaceLocHint<'a, 'b>(pub &'a Loc, pub &'b str);

pub trait LocHintConstructor<'a, 'b> {
    fn new(loc: &'a Loc, s: &'b str) -> Self;
}

impl<'a, 'b> LocHintConstructor<'a, 'b> for CommentLocHint<'a, 'b> {
    fn new(loc: &'a Loc, s: &'b str) -> Self {
        CommentLocHint(loc, s)
    }
}

impl<'a, 'b> LocHintConstructor<'a, 'b> for SpaceLocHint<'a, 'b> {
    fn new(loc: &'a Loc, s: &'b str) -> Self {
        SpaceLocHint(loc, s)
    }
}

impl CommentLocHint<'_, '_> {
    fn write_formatted_comment_block(&self, f: &mut dyn fmt::Write, cfg: &Config, _buf: &str, comment_block: String) -> fmt::Result {
        if cfg.normalize_ws == Some(true) {
            if comment_block.is_empty() {
                write!(f, "{}", self.1)?;
            } else {
                let prefix = match comment_block.chars().next().unwrap() {
                    '-' => " ",
                    _ => "",
                };
                let suffix = match comment_block.chars().last().unwrap() {
                    ']' => " ",
                    _ => "",
                };
                write!(f, "{}{}{}", prefix, comment_block, suffix)?;
            }
        } else {
            write!(f, "{}", comment_block)?;
        }
        Ok(())
    }
}

impl ConfiguredWrite for CommentLocHint<'_, '_> {
    fn configured_write(&self, f: &mut dyn fmt::Write, cfg: &Config, buf: &str) -> fmt::Result {
        let comment_buffer = &buf[self.0.0..self.0.1];
        match parse_comment(comment_buffer) {
            Ok(node_tree) => {
                let mut formatted_comment_block = String::new();
                match node_tree.configured_write(&mut formatted_comment_block, cfg, comment_buffer) {
                    Ok(_) => self.write_formatted_comment_block(f, cfg, buf, formatted_comment_block),
                    Err(err) => Err(err),
                }
            }
            _ => Err(std::fmt::Error),
        }
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

