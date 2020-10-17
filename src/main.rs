use regex::Regex;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod config;
mod file_util;
mod formatting;
mod parser;
use config::{Config, ConfiguredWrite};

fn get_options_and_filenames() -> (Vec<String>, Vec<String>) {
    let args: Vec<String> = env::args().skip(1).collect();
    let (options, mut sources): (Vec<_>, Vec<_>) = args.into_iter().partition(|arg| arg.starts_with('-'));
    sources.sort();

    (options, sources)
}

fn get_config_with_options(options: &Vec<String>) -> Config {
    let mut config = Config::default();

    for option in options.iter() {
        let re = Regex::new(r"^[-]+([a-zA-Z_0-9]+)\s*=(.*)$").unwrap();
        match re.captures_iter(option).next() {
            Some(cap) => config.set(&cap[1], &cap[2]),
            None => eprintln!("Unrecognized option `{}`", option),
        }
    }

    config
}

fn process_file(file_path: &PathBuf, config: &Config) {
    let content =
        fs::read_to_string(file_path).expect(&format!("An error occured while reading file `{}`", file_path.display()));

    println!("Process file: `{}`", file_path.display());
    match parser::parse_lua(&content) {
        Ok(node_tree) => {
            let mut outbuffer = String::new();
            let state = config::State::default();
            match node_tree.configured_write(&mut outbuffer, &config, &content, &state) {
                Ok(_) => match config.inplace {
                    Some(true) => fs::write(file_path, outbuffer)
                        .expect(&format!("An error occured while writing file `{}`", file_path.display())),
                    _ => (), //print!("{}", outbuffer),
                },
                Err(_) => println!("An error occured while formatting file `{}`: {:?}", file_path.display(), node_tree),
            };
        }
        Err(err) => println!("An error occured while parsing file `{}`: {}", file_path.display(), err),
    }
}

fn main() -> Result<(), std::io::Error> {
    let (options, rel_paths) = get_options_and_filenames();
    let config = get_config_with_options(&options);

    println!("Paths: {:?}", rel_paths);
    println!("Options: {:?}", options);
    println!("Config: {:?}\n", config);

    for rel_path in &rel_paths {
        let path_buf = Path::new(rel_path).to_path_buf();

        match file_util::get_path_files(&path_buf, config.recursive == Some(true)) {
            Ok(file_paths) => {
                for file_path in &file_paths {
                    process_file(&file_path, &config);
                }
            }
            Err(_) => println!("Unresolved path: `{}`", rel_path),
        }
    }

    Ok(())
}
