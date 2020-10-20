use crate::config;
use crate::config::{Config, ConfiguredWrite};
use crate::file_util;
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

pub fn process_file(file_path: &PathBuf, cfg: &Config) -> Result<String, FormatterError> {
    println!("Process file: `{}`", file_path.display());
    if !file_path.is_file() {
        return Err(ReadingError(format!("An error occured while reading file `{}`", file_path.display())));
    }

    use FormatterError::*;
    match cfg.is_empty() {
        true => match file_util::get_file_config(file_path, crate::CFG_PREFIX) {
            Some(file_config) => match Config::load_from_file(&file_config) {
                Ok(cfg) => process_file_with_config(&file_path, &cfg),
                Err(err) => Err(InvalidConfigFile(err)),
            },
            None => Err(NoConfigureFile("Configure file was not found".to_string())),
        },
        false => process_file_with_config(&file_path, &cfg),
    }
}

fn process_file_with_config(file_path: &PathBuf, cfg: &Config) -> Result<String, FormatterError> {
    println!("Format options: {}", cfg);

    use FormatterError::*;
    match fs::read_to_string(file_path) {
        Ok(content) => match parser::parse_lua(&content) {
            Ok(node_tree) => {
                let mut outbuffer = String::new();
                let mut state = config::State::default();

                match node_tree.configured_write(&mut outbuffer, &cfg, &content, &mut state) {
                    Ok(_) => Ok(outbuffer),
                    Err(_) => Err(FormattingError(format!(
                        "An error occured while formatting file `{}`: {:?}",
                        file_path.display(),
                        node_tree
                    ))),
                }
            }
            Err(err) => {
                Err(ParsingError(format!("An error occured while parsing file `{}`: {}", file_path.display(), err)))
            }
        },
        _ => Err(ReadingError(format!("An error occured while reading file `{}`", file_path.display()))),
    }
}
