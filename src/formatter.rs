use std::fmt;
use std::fs;
use std::path::PathBuf;

use crate::config;
use crate::config::{Config, ConfiguredWrite};
use crate::file_util;
use crate::formatting::reconstruction;
use crate::formatting::util;
use crate::parser;

#[derive(Debug)]
pub enum FormatterError {
    ReadingError,
    NoConfigureFile,
    InvalidConfigFile(String),
    ParsingError(String),
    FormattingError(String),
}

impl fmt::Display for FormatterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use FormatterError::*;
        match self {
            ReadingError => write!(f, "cannot read file"),
            NoConfigureFile => write!(f, "configure file was not found"),
            InvalidConfigFile(conf_fname) => write!(f, "invalid configure file: {}", conf_fname),
            ParsingError(err) => write!(f, "parsing error: {}", err),
            FormattingError(err) => write!(f, "formatting error: {}", err),
        }
    }
}

pub fn process_file(file_path: &PathBuf, cfg: &Config, verbose: bool) -> Result<String, FormatterError> {
    if !file_path.is_file() {
        return Err(ReadingError);
    }

    if verbose {
        println!("Process file: `{}`", file_path.display());
    }

    let buffer: String;
    match fs::read_to_string(file_path) {
        Ok(content) => {
            buffer = content;
        }
        _ => return Err(ReadingError),
    }

    use FormatterError::*;
    match cfg.has_empty_format() {
        true => match file_util::get_file_config(file_path, crate::CFG_PREFIX) {
            Some(file_config) => match cfg.reload_format_from_file(&file_config) {
                Ok(new_cfg) => process_buffer_with_config(&buffer, &new_cfg, verbose),
                Err(err) => Err(InvalidConfigFile(err)),
            },
            None => Err(NoConfigureFile),
        },
        false => process_buffer_with_config(&buffer, &cfg, verbose),
    }
}

pub fn process_buffer_with_config(content: &String, cfg: &Config, verbose: bool) -> Result<String, FormatterError> {
    if verbose {
        println!("Format options: {}", cfg);
    }

    use FormatterError::*;
    match parser::parse_lua(&content) {
        Ok(mut node_tree) => {
            let mut outbuffer = String::new();
            let mut state = config::State::default();

            // process the tree
            state.pos_range = util::line_range_to_pos_range(&content, cfg.line_range);
            reconstruction::reconstruct_node_tree(&mut node_tree, cfg, &mut state);

            match node_tree.configured_write(&mut outbuffer, &cfg, &content, &mut state) {
                Ok(_) => Ok(outbuffer),
                Err(_) => Err(FormattingError(format!("{:?}", node_tree))),
            }
        }
        Err(err) => Err(ParsingError(format!("{}", err))),
    }
}
