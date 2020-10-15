use super::loc_hint::*;
use crate::config::{Config, ConfiguredWrite};
use crate::parser::basics::Loc;
use crate::{cfg_write, cfg_write_helper};
use std::fmt;

pub trait PrefixHintInNoSepList {
    fn prefix_hint_in_no_sep_list(&self, config: &Config) -> &str;
}

pub fn cfg_write_sep_vector<'a, 'b, 'c: 'a + 'b, Node, Hint>(
    f: &mut dyn fmt::Write,
    cfg: &Config,
    buf: &str,
    elems: &'c Vec<(Loc, Node, Loc, String)>,
    default_ws: &'static str,
    default_sep: Option<&str>,
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
        cfg_write!(f, cfg, buf, Hint::new(&elem.0, ws), elem.1, Hint::new(&elem.2, ""))?;

        let need_trailing_sep =
            i != elems.len() - 1 || trailing_sep.is_none() && !elem.3.is_empty() || trailing_sep == Some(true);

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

pub fn cfg_write_vector<'a, 'b, 'c: 'a + 'b, Node, Hint>(
    f: &mut dyn fmt::Write,
    cfg: &Config,
    buf: &str,
    elems: &'c Vec<(Loc, Node)>,
) -> Result<(), core::fmt::Error>
where
    Node: ConfiguredWrite + PrefixHintInNoSepList,
    Hint: ConfiguredWrite + LocHintConstructor<'a, 'b>,
{
    if !elems.is_empty() {
        let first = &elems[0];
        cfg_write!(f, cfg, buf, Hint::new(&first.0, ""), first.1)?;

        for elem in &elems[1..elems.len()] {
            cfg_write!(f, cfg, buf, Hint::new(&elem.0, elem.1.prefix_hint_in_no_sep_list(cfg)), elem.1)?;
        }
    }
    Ok(())
}
