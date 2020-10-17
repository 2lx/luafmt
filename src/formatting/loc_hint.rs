use std::fmt;
use crate::config::*;
use crate::parser::common::*;
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
    fn write_formatted_comment_block(&self, f: &mut dyn fmt::Write, cfg: &Config, _buf: &str, comment_block: &str) -> fmt::Result {

        // if `comment_block` is empty
        if comment_block.is_empty() {
            if cfg.replace_zero_spaces_with_hint == Some(true) {
                write!(f, "{}", self.1)?;
            }
            return Ok(());
        }

        // hint before comment block, starting with '-'
        if cfg.hint_before_comment.is_some() && comment_block.chars().next() == Some('-') {
            write!(f, "{}", cfg.hint_before_comment.as_ref().unwrap())?;
        }

        write!(f, "{}", comment_block)?;

        // hint after comment block, ending with ']'
        if cfg.hint_after_multiline_comment.is_some() && comment_block.chars().last() == Some(']') {
            write!(f, "{}", cfg.hint_after_multiline_comment.as_ref().unwrap())?;
        }

        Ok(())
    }
}

impl ConfiguredWrite for CommentLocHint<'_, '_> {
    fn configured_write(&self, f: &mut dyn fmt::Write, cfg: &Config, buf: &str, state: &State) -> fmt::Result {
        let comment_buffer = &buf[self.0.0..self.0.1];
        match parse_comment(comment_buffer) {
            Ok(node_tree) => {
                let mut formatted_comment_block = String::new();
                match node_tree.configured_write(&mut formatted_comment_block, cfg, comment_buffer, state) {
                    Ok(_) => self.write_formatted_comment_block(f, cfg, buf, &formatted_comment_block),
                    Err(err) => Err(err),
                }
            }
            _ => Err(std::fmt::Error),
        }
    }
}

impl ConfiguredWrite for SpaceLocHint<'_, '_> {
    fn configured_write(&self, f: &mut dyn fmt::Write, cfg: &Config, buf: &str, _state: &State) -> fmt::Result {
        if cfg.remove_spaces_between_tokens == Some(true) {
            write!(f, "{}", self.1)?;
            return Ok(());
        }

        write!(f, "{}", &buf[self.0.0..self.0.1])
    }
}

