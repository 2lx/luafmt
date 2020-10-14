use std::fmt;

use crate::config::{Config, ConfiguredWrite};
use crate::{cfg_write, cfg_write_helper};
use crate::format::loc_hint::LocHint;
use crate::format::util::*;
use super::basics::*;

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
            _ => " ",
        }
    }
}

impl ConfiguredWrite for Node {
    fn configured_write(&self, f: &mut dyn fmt::Write, cfg: &Config, buf: &str) -> fmt::Result {
        use Node::*;

        match self {
            Chunk(locl, n, locr) => cfg_write!(f, cfg, buf, LocHint(&locl, ""), n, LocHint(&locr, "")),
            VariantList(_, variants) => cfg_write_vector(f, cfg, buf, variants),
            CommentList(_, comments) => cfg_write_vector(f, cfg, buf, comments),
            NewLineList(_, newlines) => cfg_write_vector(f, cfg, buf, newlines),

            OneLineComment(_, s) => write!(f, "--{}\n", s),
            MultiLineComment(_, level, s) => {
                let level_str = (0..*level).map(|_| "=").collect::<String>();
                write!(f, "--[{}[{}]{}]", level_str, s, level_str)
            }
            NewLine(_) => write!(f, "\n"),
        }
    }
}
