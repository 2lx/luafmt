use std::fmt::Write;

use super::common::*;
use crate::config::*;
use crate::formatting::loc_hint::*;
use crate::formatting::list;
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

impl list::ListOfItems<Node> for Node {
    fn items(&self) -> Option<&Vec::<(Loc, Node)>> {
        use Node::*;
        match self {
            VariantList(_, items) | CommentList(_, items) | NewLineList(_, items) => Some(items),
            _ => None,
        }
    }

    fn need_newlines(&self, _cfg: &Config) -> bool {
        use Node::*;
        match self {
            CommentList(..) | VariantList(..) => true,
            _ => false,
        }
    }
}

impl<'a> list::AnyListItem<'a, Node> for Node {
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

    fn need_newline(&self, _parent: &Node, _f: &mut String, cfg: &Config, _buf: &str, _state: &mut State) -> bool {
        use Node::*;
        match self {
            // here we know, that CommentList is not the first item of VariantList
            CommentList(_, comments) if !comments.is_empty() => match &comments[0] {
                (_, OneLineComment(..)) => cfg.newline_format_oneline_comment == Some(1),
                (_, MultiLineComment(..)) => cfg.newline_format_multiline_comment == Some(1),
                _ => false,
            }
            OneLineComment(..) => cfg.newline_format_oneline_comment == Some(1),
            MultiLineComment(..) => cfg.newline_format_multiline_comment == Some(1),
            _ => false,
        }
    }

    fn need_first_newline(&self, _parent: &Node, _f: &mut String, cfg: &Config, _buf: &str, _state: &mut State) -> bool {
        use Node::*;
        match self {
            // here we know, that CommentList is the first item of VariantList
            CommentList(_, comments) if !comments.is_empty() => match &comments[0] {
                (_, OneLineComment(..)) => cfg.newline_format_first_oneline_comment == Some(1),
                (_, MultiLineComment(..)) => cfg.newline_format_first_multiline_comment == Some(1),
                _ => false,
            }
            _ => false,
        }
    }
}

impl ConfiguredWrite for Node {
    fn configured_write(&self, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> std::fmt::Result {
        use Node::*;

        #[allow(non_snake_case)]
        let Hint = SpaceLocHint;
        let cfg_write_list = list::cfg_write_list::<Node, SpaceLocHint>;

        match self {
            Chunk(locl, n, locr) => cfg_write!(f, cfg, buf, state, Hint(&locl, ""), n, Hint(&locr, "")),
            VariantList(_, variants) => {
                if cfg.remove_single_newlines == Some(true) && variants.len() == 1 {
                    if let (_, NewLineList(_, newlines)) = &variants[0] {
                        if newlines.len() == 1 {
                            return Ok(())
                        }
                    }
                }

                cfg_write_list(f, cfg, buf, state, self)?;
                Ok(())
            }
            comments@CommentList(..) => {
                cfg_write_list(f, cfg, buf, state, comments)?;
                Ok(())
            }
            newlines@NewLineList(..) => {
                cfg_write_list(f, cfg, buf, state, newlines)?;
                Ok(())
            }

            OneLineComment(_, s) => match cfg.remove_comments {
                Some(true) => match cfg.remove_all_newlines {
                    Some(true) => Ok(()),
                    _ => write!(f, "\n"),
                },
                _ => match cfg.hint_before_oneline_comment_text.as_ref() {
                    Some(prefix) => {
                        let strimmed = s.trim_start();

                        // do not print the `prefix` if trimmed `s` is empty
                        if strimmed.is_empty() {
                            write!(f, "--\n")?;
                        } else {
                            write!(f, "--{}{}\n", prefix, strimmed)?;
                        }
                        Ok(())
                    }
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
            NewLine(_) => match cfg.remove_all_newlines {
                Some(true) => Ok(()),
                _ => write!(f, "\n"),
            },
        }
    }
}
