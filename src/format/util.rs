use super::loc_hint::*;
use crate::config::*;
use crate::parser::basics::Loc;
use crate::{cfg_write, cfg_write_helper};
use std::fmt;
use std::cmp::Ordering;

pub trait PrefixHintInList<'a> {
    fn prefix_hint_in_list(&self, config: &'a Config) -> &'a str;
}

pub trait SuffixHintInList<'a> {
    fn suffix_hint_in_list(&self, config: &'a Config) -> &'a str;
}

pub fn longest_hint<'a>(hint1: &'a str, hint2: &'a str) -> &'a str {
    return match hint1.len().cmp(&hint2.len()) {
        Ordering::Less => hint2,
        Ordering::Greater => hint1,
        Ordering::Equal => hint1,
    }
}

pub fn cfg_write_sep_vector<'a, 'b, 'c: 'a + 'b, Node, Hint>(
    f: &mut dyn fmt::Write,
    cfg: &Config,
    buf: &str,
    state: &State,
    elems: &'c Vec<(Loc, Node, Loc, String)>,
    default_ws: &'c str,
    default_sep: &Option<String>,
    trailing_sep: Option<bool>,
) -> Result<(), core::fmt::Error>
where
    Node: ConfiguredWrite,
    Hint: ConfiguredWrite + LocHintConstructor<'a, 'b>,
{
    for i in 0..elems.len() {
        let ws = match i {
            0 => "",
            _ => default_ws,
        };
        let elem = &elems[i];
        cfg_write!(f, cfg, buf, state, Hint::new(&elem.0, ws), elem.1, Hint::new(&elem.2, ""))?;

        let need_trailing_sep =
            i != elems.len() - 1 || trailing_sep.is_none() && !elem.3.is_empty() || trailing_sep == Some(true);

        if need_trailing_sep {
            match &default_sep {
                &Some(ref s) => write!(f, "{}", s)?,
                &None => write!(f, "{}", elem.3)?,
            }
        }
    }
    Ok(())
}

pub fn cfg_write_vector<'a, 'b, 'c: 'a + 'b, Node, Hint>(
    f: &mut dyn fmt::Write,
    cfg: &'c Config,
    buf: &str,
    state: &State,
    elems: &'c Vec<(Loc, Node)>,
) -> Result<(), core::fmt::Error>
where
    Node: ConfiguredWrite + PrefixHintInList<'c> + SuffixHintInList<'c>,
    Hint: ConfiguredWrite + LocHintConstructor<'a, 'b>,
{
    if !elems.is_empty() {
        let first = &elems[0];

        cfg_write!(f, cfg, buf, state, Hint::new(&first.0, ""), first.1)?;
        for i in 1..elems.len() {
            let suffix = elems[i - 1].1.suffix_hint_in_list(cfg);
            let prefix = elems[i].1.prefix_hint_in_list(cfg);
            let hint = longest_hint(prefix, suffix);

            cfg_write!(f, cfg, buf, state, Hint::new(&elems[i].0, hint), elems[i].1)?;
        }
    }
    Ok(())
}
