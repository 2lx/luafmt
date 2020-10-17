use super::loc_hint::*;
use super::util::*;
use crate::config::*;
use crate::parser::common::*;
use crate::{cfg_write, cfg_write_helper};
use std::fmt;

pub trait NoSepListItem<'a> {
    fn list_item_prefix_hint(&self, config: &'a Config) -> &'a str;
    fn list_item_suffix_hint(&self, config: &'a Config) -> &'a str;
}

pub trait SepListOfItems<Node> {
    fn items(&self) -> Option<&Vec<(Loc, Node, Loc, String)>>;
    fn element_prefix_hint(&self) -> &str;
    fn separator(&self, cfg: &Config) -> Option<String>;
    fn trailing_separator(&self, cfg: &Config) -> Option<bool>;
}

pub fn cfg_write_sep_list<'a, 'b, 'c, 'd, 'n: 'a + 'b + 'c, Node, Hint>(
    f: &mut dyn fmt::Write,
    cfg: &'d Config,
    buf: &str,
    state: &State,
    list_node: &'n Node,
) -> Result<(), core::fmt::Error>
where
    Node: ConfiguredWrite + SepListOfItems<Node>,
    Hint: ConfiguredWrite + LocHintConstructor<'a, 'b>,
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
                cfg_write!(f, cfg, buf, state, Hint::new(&first.0, ""), first.1, Hint::new(&first.2, ""))?;

                for i in 1..items.len() {
                    write!(f, "{}", get_sep(&items[i - 1]))?;

                    let item = &items[i];
                    cfg_write!(
                        f,
                        cfg,
                        buf,
                        state,
                        Hint::new(&item.0, list_node.element_prefix_hint()),
                        item.1,
                        Hint::new(&item.2, "")
                    )?;
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

pub fn cfg_write_no_sep_list_items<'a, 'b, 'c: 'a + 'b, Node, Hint>(
    f: &mut dyn fmt::Write,
    cfg: &'c Config,
    buf: &str,
    state: &State,
    elems: &'c Vec<(Loc, Node)>,
) -> Result<(), core::fmt::Error>
where
    Node: ConfiguredWrite + NoSepListItem<'c>,
    Hint: ConfiguredWrite + LocHintConstructor<'a, 'b>,
{
    if !elems.is_empty() {
        let first = &elems[0];

        cfg_write!(f, cfg, buf, state, Hint::new(&first.0, ""), first.1)?;
        for i in 1..elems.len() {
            let suffix = elems[i - 1].1.list_item_suffix_hint(cfg);
            let prefix = elems[i].1.list_item_prefix_hint(cfg);
            let hint = longest_hint(prefix, suffix);

            cfg_write!(f, cfg, buf, state, Hint::new(&elems[i].0, hint), elems[i].1)?;
        }
    }
    Ok(())
}
