use super::decoration::*;
use super::loc_hint::*;
use super::util::*;
use crate::config::*;
use crate::parser::common::*;
use crate::{cfg_write, cfg_write_helper};
use std::fmt::Write;

pub trait NoSepListItem<'a> {
    fn list_item_prefix_hint(&self, cfg: &'a Config) -> &'a str;
    fn list_item_suffix_hint(&self, cfg: &'a Config) -> &'a str;
    fn need_indent(&self, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> bool;
    fn need_first_indent(&self, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> bool;
}

pub trait SepListOfItems<Node> {
    fn items(&self) -> Option<&Vec<(Loc, Node, Loc, String)>>;
    fn element_prefix_hint(&self) -> &str;
    fn separator(&self, cfg: &Config) -> Option<String>;
    fn trailing_separator(&self, cfg: &Config) -> Option<bool>;
    fn need_indent_items(&self, cfg: &Config) -> bool;
}

pub fn cfg_write_sep_list<'a, 'b, 'c, 'n: 'a + 'b + 'c, Node, Hint>(
    f: &mut String,
    cfg: &'n Config,
    buf: &str,
    state: &mut State,
    list_node: &'n Node,
) -> Result<(), core::fmt::Error>
where
    Node: ConfiguredWrite + SepListOfItems<Node> + NoSepListItem<'n>,
    Hint: ConfiguredWrite + LocHintConstructible<'a, 'b>,
{
    match list_node.items() {
        Some(items) => {
            if !items.is_empty() {
                let sep_opt = list_node.separator(cfg);
                let get_sep = |item: &'n (Loc, Node, Loc, String)| -> &'c str {
                    match &sep_opt {
                        Some(sep) => &sep,
                        None => &item.3,
                    }
                };

                let first = &items[0];
                let need_indent = list_node.need_indent_items(cfg) && first.1.need_first_indent(f, cfg, buf, state);
                cfg_write!(f, cfg, buf, state, NewLineDecor(Hint::new(&first.0, ""), need_indent), first.1,
                           Hint::new(&first.2, ""))?;

                for i in 1..items.len() {
                    write!(f, "{}", get_sep(&items[i - 1]))?;

                    let item = &items[i];
                    let need_indent = list_node.need_indent_items(cfg) && item.1.need_indent(f, cfg, buf, state);
                    cfg_write!(f, cfg, buf, state, NewLineDecor(Hint::new(&item.0, list_node.element_prefix_hint()), need_indent), item.1,
                               Hint::new(&item.2, ""))?;
                }

                let last = &items[items.len() - 1];
                let trailing_sep = list_node.trailing_separator(cfg);
                if trailing_sep.is_none() && !last.3.is_empty() || trailing_sep == Some(true) {
                    write!(f, "{}", get_sep(&last))?;
                }
            }
        }
        None => {}
    }
    Ok(())
}

pub fn cfg_write_list_items<'a, 'b, 'c: 'a + 'b, Node, Hint>(
    f: &mut String,
    cfg: &'c Config,
    buf: &str,
    state: &mut State,
    elems: &'c Vec<(Loc, Node)>,
) -> Result<(), core::fmt::Error>
where
    Node: ConfiguredWrite + NoSepListItem<'c>,
    Hint: ConfiguredWrite + LocHintConstructible<'a, 'b>,
{
    if !elems.is_empty() {
        let first = &elems[0];

        let need_indent = first.1.need_first_indent(f, cfg, buf, state);
        cfg_write!(f, cfg, buf, state, NewLineDecor(Hint::new(&first.0, ""), need_indent), first.1)?;

        for i in 1..elems.len() {
            let suffix = elems[i - 1].1.list_item_suffix_hint(cfg);
            let prefix = elems[i].1.list_item_prefix_hint(cfg);
            let hint = longest_hint(prefix, suffix);

            let need_indent = elems[i].1.need_indent(f, cfg, buf, state);
            cfg_write!(f, cfg, buf, state, NewLineDecor(Hint::new(&elems[i].0, hint), need_indent), elems[i].1)?;
        }
    }
    Ok(())
}
