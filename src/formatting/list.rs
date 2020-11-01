use std::fmt::Write;

use super::decoration::*;
use super::loc_hint::*;
use super::util;
use crate::config::*;
use crate::parser::common::*;
use crate::{cfg_write, cfg_write_helper};

pub trait AnyListItem<'a, Node> {
    fn list_item_prefix_hint(&self, cfg: &'a Config) -> &'a str;
    fn need_newline(
        &self, prev: &Node, parent: &Node, f: &mut String, cfg: &Config, buf: &str, state: &mut State,
    ) -> bool;
    fn need_first_newline(&self, parent: &Node, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> bool;
}

pub trait SepListOfItems<Node> {
    fn items(&self) -> Option<&Vec<(Loc, Node, Loc, String)>>;
    fn element_prefix_hint(&self) -> &str;
    fn separator(&self, cfg: &Config) -> Option<String>;
    fn trailing_separator(&self, cfg: &Config) -> Option<bool>;
    fn need_newlines(&self, cfg: &Config) -> bool;
}

pub trait ListOfItems<Node> {
    fn items(&self) -> Option<&Vec<(Loc, Node)>>;
    fn need_newlines(&self, cfg: &Config) -> bool;
}

pub fn cfg_write_sep_list<'a, 'b, 'c, 'n: 'a + 'b + 'c, Node, Hint>(
    f: &mut String, cfg: &'n Config, buf: &str, state: &mut State, list_node: &'n Node,
) -> Result<bool, core::fmt::Error>
where
    Node: ConfiguredWrite + SepListOfItems<Node> + AnyListItem<'n, Node>,
    Hint: ConfiguredWrite + LocHintConstructible<'a, 'b>,
{
    let mut indent = false;
    match list_node.items() {
        Some(items) if !items.is_empty() => {
            let sep_opt = list_node.separator(cfg);
            let get_sep = |item: &'n (Loc, Node, Loc, String)| -> &'c str {
                match &sep_opt {
                    Some(sep) => &sep,
                    None => &item.3,
                }
            };

            let first = &items[0];
            let need_newline =
                list_node.need_newlines(cfg) && first.1.need_first_newline(list_node, f, cfg, buf, state);
            indent = indent || need_newline;

            #[cfg_attr(rustfmt, rustfmt_skip)]
            cfg_write!(f, cfg, buf, state, IfNewLine(need_newline, Hint::new(&first.0, "")), first.1,
                       Hint::new(&first.2, ""))?;

            for i in 1..items.len() {
                let prev_item_tp = &items[i - 1];
                let item = &items[i];
                write!(f, "{}", get_sep(prev_item_tp))?;

                if util::test_out_of_range(&state.pos_range, &item.0) {
                    cfg_write!(f, cfg, buf, state, item.0)?;
                } else {
                    let need_newline =
                        list_node.need_newlines(cfg) && item.1.need_newline(&prev_item_tp.1, list_node, f, cfg, buf, state);
                    indent = indent || need_newline;

                    #[cfg_attr(rustfmt, rustfmt_skip)]
                    cfg_write!(f, cfg, buf, state, IfNewLine(need_newline, Hint::new(&item.0, list_node.element_prefix_hint())))?;
                }

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, item.1, Hint::new(&item.2, ""))?;
            }

            let last = &items[items.len() - 1];
            let trailing_sep = list_node.trailing_separator(cfg);
            if trailing_sep.is_none() && !last.3.is_empty() || trailing_sep == Some(true) {
                write!(f, "{}", get_sep(&last))?;
            }
        }
        _ => {}
    }
    Ok(indent)
}

pub fn cfg_write_list<'a, 'b, 'n: 'a + 'b, Node, Hint>(
    f: &mut String, cfg: &'n Config, buf: &str, state: &mut State, list_node: &'n Node,
) -> Result<bool, core::fmt::Error>
where
    Node: ConfiguredWrite + ListOfItems<Node> + AnyListItem<'n, Node>,
    Hint: ConfiguredWrite + LocHintConstructible<'a, 'b>,
{
    let mut indent = false;
    match list_node.items() {
        Some(items) if !items.is_empty() => {
            let first = &items[0];

            // first comment block is always empty
            let need_newline =
                list_node.need_newlines(cfg) && first.1.need_first_newline(list_node, f, cfg, buf, state);
            indent = indent || need_newline;

            #[cfg_attr(rustfmt, rustfmt_skip)]
            cfg_write!(f, cfg, buf, state, IfNewLine(need_newline, Hint::new(&first.0, "")), first.1)?;

            for i in 1..items.len() {
                let item = &items[i];

                if util::test_out_of_range(&state.pos_range, &item.0) {
                    cfg_write!(f, cfg, buf, state, item.0)?;
                } else {
                    let prev_item_tp = &items[i - 1];
                    let hint = item.1.list_item_prefix_hint(cfg);
                    let need_newline =
                        list_node.need_newlines(cfg) && item.1.need_newline(&prev_item_tp.1, list_node, f, cfg, buf, state);
                    indent = indent || need_newline;

                    #[cfg_attr(rustfmt, rustfmt_skip)]
                    cfg_write!(f, cfg, buf, state, IfNewLine(need_newline, Hint::new(&items[i].0, hint)))?;
                }

                cfg_write!(f, cfg, buf, state, item.1)?;
            }
        }
        _ => {}
    }
    Ok(indent)
}
