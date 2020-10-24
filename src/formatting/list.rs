use super::decoration::*;
use super::loc_hint::*;
use super::util::*;
use crate::config::*;
use crate::parser::common::*;
use crate::{cfg_write, cfg_write_helper};
use std::fmt::Write;

pub trait NoSepListItem<'a, Node> {
    fn list_item_prefix_hint(&self, cfg: &'a Config) -> &'a str;
    fn list_item_suffix_hint(&self, cfg: &'a Config) -> &'a str;
    fn need_newline(&self, parent: &Node, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> bool;
    fn need_first_newline(&self, parent: &Node, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> bool;
}

pub trait SepListOfItems<Node> {
    fn items(&self) -> Option<&Vec<(Loc, Node, Loc, String)>>;
    fn element_prefix_hint(&self) -> &str;
    fn separator(&self, cfg: &Config) -> Option<String>;
    fn trailing_separator(&self, cfg: &Config) -> Option<bool>;
    fn need_newlines(&self, cfg: &Config) -> bool;
    fn need_indent(&self, cfg: &Config) -> bool;
}

pub trait ListOfItems<Node> {
    fn items(&self) -> Option<&Vec<(Loc, Node)>>;
    fn element_prefix_hint(&self) -> &str;
    fn need_newlines(&self, cfg: &Config) -> bool;
    fn need_indent(&self, cfg: &Config) -> bool;
}

pub fn cfg_write_sep_list<'a, 'b, 'c, 'n: 'a + 'b + 'c, Node, Hint>(
    f: &mut String,
    cfg: &'n Config,
    buf: &str,
    state: &mut State,
    list_node: &'n Node,
) -> Result<(), core::fmt::Error>
where
    Node: ConfiguredWrite + SepListOfItems<Node> + NoSepListItem<'n, Node>,
    Hint: ConfiguredWrite + LocHintConstructible<'a, 'b>,
{
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
            let need_newline = list_node.need_newlines(cfg) && first.1.need_first_newline(list_node, f, cfg, buf, state);
            let need_indent = list_node.need_indent(cfg);
            cfg_write!(f, cfg, buf, state, If(need_indent, &IncIndent(None)),
                       IfNewLine(need_newline, Hint::new(&first.0, "")), first.1,
                       Hint::new(&first.2, ""))?;

            for i in 1..items.len() {
                write!(f, "{}", get_sep(&items[i - 1]))?;

                let item = &items[i];
                let need_newline = list_node.need_newlines(cfg) && item.1.need_newline(list_node, f, cfg, buf, state);
                cfg_write!(f, cfg, buf, state, IfNewLine(need_newline, Hint::new(&item.0, list_node.element_prefix_hint())),
                           item.1, Hint::new(&item.2, ""))?;
            }

            let last = &items[items.len() - 1];
            let trailing_sep = list_node.trailing_separator(cfg);
            if trailing_sep.is_none() && !last.3.is_empty() || trailing_sep == Some(true) {
                write!(f, "{}", get_sep(&last))?;
            }
            cfg_write!(f, cfg, buf, state, If(need_indent, &DecIndent()))?;
        }
        _ => {}
    }
    Ok(())
}

pub fn cfg_write_list_items<'a, 'b, 'n: 'a + 'b, Node, Hint>(
    f: &mut String,
    cfg: &'n Config,
    buf: &str,
    state: &mut State,
    list_node: &'n Node,
) -> Result<(), core::fmt::Error>
where
    Node: ConfiguredWrite + ListOfItems<Node> + NoSepListItem<'n, Node>,
    Hint: ConfiguredWrite + LocHintConstructible<'a, 'b>,
{
    match list_node.items() {
        Some(items) if !items.is_empty() => {
            let first = &items[0];

            let need_newline = list_node.need_newlines(cfg) && first.1.need_first_newline(list_node, f, cfg, buf, state);
            let need_indent = list_node.need_indent(cfg);
            cfg_write!(f, cfg, buf, state, If(need_indent, &IncIndent(None)),
                       IfNewLine(need_newline, Hint::new(&first.0, "")), first.1)?;

            for i in 1..items.len() {
                let suffix = items[i - 1].1.list_item_suffix_hint(cfg);
                let prefix = items[i].1.list_item_prefix_hint(cfg);
                let hint = longest_hint(prefix, suffix);

                let need_newline = list_node.need_newlines(cfg) && items[i].1.need_newline(list_node, f, cfg, buf, state);
                cfg_write!(f, cfg, buf, state, IfNewLine(need_newline, Hint::new(&items[i].0, hint)), items[i].1)?;
            }

            cfg_write!(f, cfg, buf, state, If(need_indent, &DecIndent()))?;
        }
        _ => {}
    }
    Ok(())
}
