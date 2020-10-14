use std::fmt;
use crate::parser::basics::Loc;
use crate::config::{Config, ConfiguredWrite};
use crate::{cfg_write, cfg_write_helper};
use super::loc_hint::*;

pub trait PrefixHintInNoSepList {
    fn prefix_hint_in_no_sep_list(&self, config: &Config) -> &str;
}

pub fn cfg_write_sep_vector<Node: ConfiguredWrite>(
    f: &mut dyn fmt::Write,
    cfg: &Config,
    buf: &str,
    elems: &Vec<(Loc, Node, Loc, String)>,
    default_ws: &'static str,
    default_sep: Option<&str>,
    trailing_sep: Option<bool>,
) -> Result<(), core::fmt::Error> {
    for i in 0..elems.len() {
        let ws = match i { 0 => "", _ => default_ws, };
        let elem = &elems[i];
        cfg_write!(f, cfg, buf, LocHint(&elem.0, ws), elem.1, LocHint(&elem.2, ""))?;

        let need_trailing_sep = i != elems.len() - 1
            || trailing_sep.is_none() && !elem.3.is_empty()
            || trailing_sep == Some(true);

        if need_trailing_sep {
            if default_sep.is_none() {
                write!(f, "{}", elem.3)?;
            } else {
                write!(f, "{}", default_sep.unwrap())?;
            }
        }
    }
    Ok(())
}

pub fn cfg_write_vector<Node: ConfiguredWrite + PrefixHintInNoSepList>(
    f: &mut dyn fmt::Write,
    cfg: &Config,
    buf: &str,
    elems: &Vec<(Loc, Node)>,
) -> Result<(), core::fmt::Error> {
    if !elems.is_empty() {
        let first = &elems[0];
        cfg_write!(f, cfg, buf, LocHint(&first.0, ""), first.1)?;

        for elem in &elems[1..elems.len()] {
            cfg_write!(f, cfg, buf, LocHint(&elem.0, elem.1.prefix_hint_in_no_sep_list(cfg)), elem.1)?;
        }
    }
    Ok(())
}

pub fn cfg_write_vector_comments<Node: ConfiguredWrite + PrefixHintInNoSepList>(
    f: &mut dyn fmt::Write,
    cfg: &Config,
    buf: &str,
    elems: &Vec<(Loc, Node)>,
) -> Result<(), core::fmt::Error> {
    if !elems.is_empty() {
        let first = &elems[0];
        cfg_write!(f, cfg, buf, SpaceLocHint(&first.0, ""), first.1)?;

        for elem in &elems[1..elems.len()] {
            cfg_write!(f, cfg, buf, SpaceLocHint(&elem.0, elem.1.prefix_hint_in_no_sep_list(cfg)), elem.1)?;
        }
    }
    Ok(())
}
