use regex::Regex;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use luafmt::config::Config;
use luafmt::file_util;
use luafmt::formatter;

fn get_options_and_filenames() -> (Vec<String>, Vec<String>) {
    let args: Vec<String> = env::args().skip(1).collect();
    let (options, mut sources): (Vec<_>, Vec<_>) = args.into_iter().partition(|arg| arg.starts_with('-'));
    sources.sort();

    (options, sources)
}

#[derive(Debug, PartialEq)]
pub struct ProgramOpts {
    pub inplace: bool,
    pub recursive: bool,
}

impl ProgramOpts {
    pub const fn default() -> Self {
        ProgramOpts { inplace: false, recursive: false }
    }
}

fn parse_options(options: &Vec<String>) -> (Config, ProgramOpts) {
    let mut config = Config::default();
    let mut program_opts = ProgramOpts::default();

    for option in options.iter() {
        let re_config_opt = Regex::new(r"^[-]+([a-zA-Z_0-9]+)\s*=(.*)$").unwrap();
        let re_program_opt = Regex::new(r"^[-]+([a-zA-Z_0-9]+)$").unwrap();

        match re_config_opt.captures_iter(option).next() {
            Some(cap) => config.set(&cap[1], &cap[2]),
            None => match re_program_opt.captures_iter(option).next() {
                Some(cap) if &cap[1] == "i" || &cap[1] == "inplace" => program_opts.inplace = true,
                Some(cap) if &cap[1] == "r" || &cap[1] == "recursive" => program_opts.recursive = true,
                _ => eprintln!("Unrecognized option `{}`", option),
            },
        }
    }

    (config, program_opts)
}

fn process_file_path(file_path: &PathBuf, config: &Config, program_opts: &ProgramOpts) {
    match formatter::process_file(&file_path, &config) {
        Ok(output) => match program_opts.inplace {
            true => match fs::write(file_path, output) {
                Ok(..) => {}
                Err(err) => {
                    eprintln!("{}", format!("An error occured while writing file `{}`: {}", file_path.display(), err))
                }
            },
            false => print!("\n{}", output),
        },
        Err(msg) => eprintln!("{:?}", msg),
    }
}

fn main() {
    let (options, rel_paths) = get_options_and_filenames();
    let (config, program_opts) = parse_options(&options);

    println!("Paths: {:?}", rel_paths);
    println!("Program options: {:?}", program_opts);

    for rel_path in &rel_paths {
        let path_buf = Path::new(rel_path).to_path_buf();

        match file_util::get_path_files(&path_buf, program_opts.recursive, "lua", luafmt::CFG_PREFIX) {
            Ok(file_paths) => {
                for file_path in &file_paths {
                    process_file_path(file_path, &config, &program_opts);
                }
            }
            Err(_) => println!("Unresolved path: `{}`", rel_path),
        }
    }
}

#[test]
fn test_parse_options() {
    let options = vec![
        "--field_separator=,".to_string(),
        "--inplace".to_string(),
        "file1.lua".to_string(),
        "./file".to_string(),
        "-r".to_string(),
        "/home/files/file.txt".to_string(),
    ];
    let cfg = Config { field_separator: Some(",".to_string()), ..Config::default() };
    let po = ProgramOpts { inplace: true, recursive: true };
    assert_eq!(parse_options(&options), (cfg, po));

    let options = vec!["-i".to_string(), "--recursive".to_string(), "--if_indent_format=1".to_string()];
    let cfg = Config { if_indent_format: Some(1), ..Config::default() };
    let po = ProgramOpts { inplace: true, recursive: true };
    assert_eq!(parse_options(&options), (cfg, po));

    let options = vec![];
    let cfg = Config { ..Config::default() };
    let po = ProgramOpts { inplace: false, recursive: false };
    assert_eq!(parse_options(&options), (cfg, po));
}
