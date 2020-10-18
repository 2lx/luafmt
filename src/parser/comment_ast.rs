use std::fmt::Write;

use super::common::*;
use crate::config::*;
use crate::formatting::loc_hint::*;
use crate::formatting::list;
use crate::formatting::decoration::*;
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

impl<'a> list::NoSepListItem<'a> for Node {
    fn list_item_prefix_hint(&self, cfg: &'a Config) -> &'a str {
        use Node::*;
        match self {
            MultiLineComment(_, _, _) | OneLineComment(_, _) => match cfg.hint_before_comment.as_ref() {
                Some(s) => s,
                None => "",
            },
            _ => "",
        }
    }

    fn list_item_suffix_hint(&self, cfg: &'a Config) -> &'a str {
        use Node::*;
        match self {
            MultiLineComment(..) => match cfg.hint_after_multiline_comment.as_ref() {
                Some(s) => s,
                None => "",
            },
            _ => "",
        }
    }

    fn need_indent(&self, cfg: &'a Config) -> bool {
        use Node::*;
        match self {
            OneLineComment(..) => cfg.indentation_string.is_some() && cfg.indent_oneline_comments == Some(true),
            MultiLineComment(..) => cfg.indentation_string.is_some() && cfg.indent_multiline_comments == Some(true),
            _ => false,
        }
    }
}

impl ConfiguredWrite for Node {
    fn configured_write(&self, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> std::fmt::Result {
        use Node::*;

        #[allow(non_snake_case)]
        let Hint = SpaceLocHint;
        let cfg_write_vector = list::cfg_write_list_items::<Node, SpaceLocHint>;

        match self {
            Chunk(locl, n, locr) => {
                let mut nl = false;
                if let VariantList(_, lists) = &**n {
                    // check `indentation_string` in advance
                    if !lists.is_empty() && cfg.indentation_string.is_some() {
                        if let (_, CommentList(_, comments)) = &lists[0] {
                            if !comments.is_empty() {
                                match &comments[0] {
                                    (_, OneLineComment(..)) if cfg.indent_first_oneline_comment == Some(true) => {
                                            nl = true;
                                    }
                                    (_, MultiLineComment(..)) if cfg.indent_first_multiline_comment == Some(true) => {
                                            nl = true;
                                    }
                                    _ => {},
                                }
                            }
                        }
                    }
                };
                cfg_write!(f, cfg, buf, state, NewLineDecor(Hint(&locl, ""), nl), n, Hint(&locr, ""))
            }
            VariantList(_, variants) => cfg_write_vector(f, cfg, buf, state, variants),
            CommentList(_, comments) => cfg_write_vector(f, cfg, buf, state, comments),
            NewLineList(_, newlines) => cfg_write_vector(f, cfg, buf, state, newlines),

            OneLineComment(_, s) => match cfg.remove_comments {
                Some(true) => match cfg.remove_newlines {
                    Some(true) => Ok(()),
                    _ => write!(f, "\n"),
                },
                _ => match cfg.hint_before_oneline_comment_text.as_ref() {
                    Some(prefix) => write!(f, "--{}{}\n", prefix, s.trim_start()),
                    None => write!(f, "--{}\n", s),
                },
            },

            MultiLineComment(_, level, s) => match cfg.remove_comments {
                Some(true) => Ok(()),
                _ => {
                    let level_str = (0..*level).map(|_| "=").collect::<String>();
                    match (
                        cfg.hint_before_multiline_comment_text.as_ref(),
                        cfg.hint_after_multiline_comment_text.as_ref(),
                    ) {
                        (Some(prefix), Some(suffix)) => {
                            write!(f, "--[{}[{}{}{}]{}]", level_str, prefix, s.trim(), suffix, level_str)
                        }
                        (Some(prefix), None) => {
                            write!(f, "--[{}[{}{}]{}]", level_str, prefix, s.trim_start(), level_str)
                        }
                        (None, Some(suffix)) => {
                            write!(f, "--[{}[{}{}]{}]", level_str, s.trim_end(), suffix, level_str)
                        }
                        _ => write!(f, "--[{}[{}]{}]", level_str, s, level_str),
                    }
                }
            },
            NewLine(_) => match cfg.remove_newlines {
                Some(true) => Ok(()),
                _ => write!(f, "\n"),
            },
        }
    }
}
