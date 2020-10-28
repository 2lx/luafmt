use crate::config;
use crate::config::{Config, ConfiguredWrite};
use crate::file_util;
use crate::formatting::reconstruction;
use crate::parser;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub enum FormatterError {
    NoConfigureFile(String),
    InvalidConfigFile(String),
    ReadingError(String),
    ParsingError(String),
    FormattingError(String),
}

pub fn process_file(file_path: &PathBuf, cfg: &Config, verbose: bool) -> Result<String, FormatterError> {
    if !file_path.is_file() {
        return Err(ReadingError(format!("An error occured while reading file `{}`", file_path.display())));
    }

    if verbose {
        println!("Process file: `{}`", file_path.display());
    }

    let buffer: String;
    match fs::read_to_string(file_path) {
        Ok(content) => { buffer = content; }
        _ => return Err(ReadingError(format!("An error occured while reading file `{}`", file_path.display()))),
    }

    use FormatterError::*;
    match cfg.is_empty() {
        true => match file_util::get_file_config(file_path, crate::CFG_PREFIX) {
            Some(file_config) => match Config::load_from_file(&file_config) {
                Ok(cfg) => process_buffer_with_config(&buffer, &cfg, verbose),
                Err(err) => Err(InvalidConfigFile(err)),
            }
            None => Err(NoConfigureFile("Configure file was not found".to_string())),
        }
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
            reconstruction::reconstruct_node_tree(&mut node_tree, cfg);

            match node_tree.configured_write(&mut outbuffer, &cfg, &content, &mut state) {
                Ok(_) => Ok(outbuffer),
                Err(_) => Err(FormattingError(format!("An error occured while formatting: {:?}", node_tree))),
            }
        }
        Err(err) => {
            Err(ParsingError(format!("An error occured while parsing file: {}", err)))
        }
    }
}
