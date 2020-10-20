use crate::config;
use crate::config::{Config, ConfiguredWrite};
use crate::file_util;
use crate::parser;
use std::fs;
use std::path::PathBuf;

pub fn process_file(file_path: &PathBuf, cfg: &Config, write_inplace: bool) {
    println!("Process file: `{}`", file_path.display());
    if cfg.is_empty() {
        match file_util::get_file_config(file_path, crate::CFG_PREFIX) {
            Some(file_config) => {
                let cfg = Config::load_from_file(&file_config);
                process_file_with_config(&file_path, &cfg, write_inplace);
            }
            None => println!("Configure file was not found"),
        }
    } else {
        process_file_with_config(&file_path, &cfg, write_inplace);
    }
}

fn process_file_with_config(file_path: &PathBuf, cfg: &Config, write_inplace: bool) {
    println!("Format options: {}", cfg);

    let content =
        fs::read_to_string(file_path).expect(&format!("An error occured while reading file `{}`", file_path.display()));

    match parser::parse_lua(&content) {
        Ok(node_tree) => {
            let mut outbuffer = String::new();
            let mut state = config::State::default();
            match node_tree.configured_write(&mut outbuffer, &cfg, &content, &mut state) {
                Ok(_) => match write_inplace {
                    true => fs::write(file_path, outbuffer)
                        .expect(&format!("An error occured while writing file `{}`", file_path.display())),
                    false => print!("\n{}", outbuffer),
                },
                Err(_) => println!("An error occured while formatting file `{}`: {:?}", file_path.display(), node_tree),
            };
        }
        Err(err) => println!("An error occured while parsing file `{}`: {}", file_path.display(), err),
    }
}
