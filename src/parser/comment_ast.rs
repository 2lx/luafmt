use std::fmt;

use super::basics::*;
use crate::config::{Config, ConfiguredWrite};
use crate::format::loc_hint::SpaceLocHint;
use crate::format::util::*;
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

impl PrefixHintInNoSepList for Node {
    fn prefix_hint_in_no_sep_list(&self, _: &Config) -> &str {
        use Node::*;

        match self {
            MultiLineComment(_, _, _) | OneLineComment(_, _) => "",
            _ => "",
        }
    }
}

impl ConfiguredWrite for Node {
    fn configured_write(&self, f: &mut dyn fmt::Write, cfg: &Config, buf: &str) -> fmt::Result {
        use Node::*;

        match self {
            Chunk(locl, n, locr) => cfg_write!(f, cfg, buf, SpaceLocHint(&locl, ""), n, SpaceLocHint(&locr, "")),
            VariantList(_, variants) => cfg_write_vector_comments(f, cfg, buf, variants),
            CommentList(_, comments) => {
                if cfg.remove_comments != Some(true) {
                    cfg_write_vector_comments(f, cfg, buf, comments)?;
                }
                Ok(())
            }
            NewLineList(_, newlines) => {
                if cfg.remove_newlines != Some(true) {
                    cfg_write_vector_comments(f, cfg, buf, newlines)?;
                }
                Ok(())
            }

            OneLineComment(_, s) => write!(f, "--{}\n", s),
            MultiLineComment(_, level, s) => {
                let level_str = (0..*level).map(|_| "=").collect::<String>();
                write!(f, "--[{}[{}]{}]", level_str, s, level_str)
            }
            NewLine(_) => write!(f, "\n"),
        }
    }
}
