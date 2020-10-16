use std::fmt;

use super::basics::*;
use crate::config::*;
use crate::format::loc_hint::SpaceLocHint;
use crate::format::util;
use crate::{cfg_write, cfg_write_helper};

#[derive(Debug)]
pub enum Node {
    OneLineComment(Loc, String),
    MultiLineComment(Loc, usize, String),
    NewLine(Loc),

    VariantList(Loc, Vec<(Loc, Node)>),
    CommentList(Loc, Vec<(Loc, Node)>),
    NewLineList(Loc, Vec<(Loc, Node)>),
    Chunk(Loc, Box<Node>, Loc),
}

impl<'a> util::PrefixHintInList<'a> for Node {
    fn prefix_hint_in_list(&self, cfg: &'a Config) -> &'a str {
        use Node::*;
        match self {
            MultiLineComment(_, _, _) | OneLineComment(_, _) => {
                match cfg.hint_before_comment.as_ref() {
                    Some(s) => s,
                    None => "",
                }
            }
            _ => "",
        }
    }
}

impl<'a> util::SuffixHintInList<'a> for Node {
    fn suffix_hint_in_list(&self, cfg: &'a Config) -> &'a str {
        use Node::*;
        match self {
            MultiLineComment(_, _, _) => {
                match cfg.hint_after_multiline_comment.as_ref() {
                    Some(s) => s,
                    None => "",
                }
            }
            _ => "",
        }
    }
}

impl ConfiguredWrite for Node {
    fn configured_write(&self, f: &mut dyn fmt::Write, cfg: &Config, buf: &str, state: &State) -> fmt::Result {
        use Node::*;

        #[allow(non_snake_case)]
        let Hint = SpaceLocHint;
        let cfg_write_vector = util::cfg_write_vector::<Node, SpaceLocHint>;

        match self {
            Chunk(locl, n, locr) => cfg_write!(f, cfg, buf, state, Hint(&locl, ""), n, Hint(&locr, "")),
            VariantList(_, variants) => cfg_write_vector(f, cfg, buf, state, variants),
            CommentList(_, comments) => cfg_write_vector(f, cfg, buf, state, comments),
            NewLineList(_, newlines) => cfg_write_vector(f, cfg, buf, state, newlines),

            OneLineComment(_, s) => match cfg.remove_comments {
                Some(true) => match cfg.remove_newlines {
                    Some(true) => Ok(()),
                    _ => write!(f, "\n"),
                },
                _ => write!(f, "--{}\n", s),
            },

            MultiLineComment(_, level, s) => match cfg.remove_comments {
                Some(true) => Ok(()),
                _ => {
                    let level_str = (0..*level).map(|_| "=").collect::<String>();
                    write!(f, "--[{}[{}]{}]", level_str, s, level_str)
                }
            },
            NewLine(_) => match cfg.remove_newlines {
                Some(true) => Ok(()),
                _ => write!(f, "\n"),
            },
        }
    }
}
