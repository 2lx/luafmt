use regex::Regex;
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use luapp::config::Config;
use luapp::file_util;
use luapp::formatter;

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
    pub verbose: bool,
    pub lines: Option<(usize, usize)>,
}

impl ProgramOpts {
    pub const fn default() -> Self {
        ProgramOpts { inplace: false, recursive: false, verbose: false, lines: None }
    }
}

fn parse_options(options: &Vec<String>) -> (Config, ProgramOpts) {
    let mut config = Config::default();
    let mut program_opts = ProgramOpts::default();

    for option in options.iter() {
        let re_lines_opt = Regex::new(r"^[-]+lines\s*=([0-9]+):([0-9]+)$").unwrap();
        let re_config_opt = Regex::new(r"^[-]+([a-zA-Z_0-9]+)\s*=(.*)$").unwrap();
        let re_program_opt = Regex::new(r"^[-]+([a-zA-Z_0-9]+)$").unwrap();

        match re_lines_opt.captures_iter(option).next() {
            Some(cap) => match (cap[1].parse(), cap[2].parse()) {
                (Ok(l1), Ok(l2)) => program_opts.lines = Some((l1, l2)),
                _ => eprintln!("Invalid `lines` option value"),
            },
            None => match re_config_opt.captures_iter(option).next() {
                Some(cap) => config.set(&cap[1], &cap[2]),
                None => match re_program_opt.captures_iter(option).next() {
                    Some(cap) if &cap[1] == "i" || &cap[1] == "inplace" => program_opts.inplace = true,
                    Some(cap) if &cap[1] == "r" || &cap[1] == "recursive" => program_opts.recursive = true,
                    Some(cap) if &cap[1] == "v" || &cap[1] == "verbose" => program_opts.verbose = true,
                    _ => eprintln!("Unrecognized option `{}`", option),
                },
            },
        }
    }

    (config, program_opts)
}

fn process_file_path(file_path: &PathBuf, config: &Config, program_opts: &ProgramOpts) {
    match formatter::process_file(&file_path, &config, program_opts.verbose) {
        Ok(output) => match program_opts.inplace {
            true => match fs::write(file_path, output) {
                Ok(..) => {}
                Err(err) => {
                    eprintln!("{}", format!("An error occured while writing file `{}`: {}", file_path.display(), err))
                }
            },
            false => print!("{}", output),
        },
        Err(err) => {
            eprintln!("{}", format!("An error occured while processing file `{}`: {}", file_path.display(), err))
        }
    }
}

fn main() {
    let (options, rel_paths) = get_options_and_filenames();
    let (config, program_opts) = parse_options(&options);

    if program_opts.verbose {
        println!("Paths: {:?}", rel_paths);
        println!("Program options: {:?}", program_opts);
    }

    if rel_paths.is_empty() {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();

        match formatter::process_buffer_with_config(&buffer, &config, program_opts.verbose) {
            Ok(output) => print!("{}", output),
            Err(msg) => eprintln!("{}", format!("An error occured while processing buffer: {}", msg)),
        }
    } else {
        for rel_path in &rel_paths {
            let path_buf = Path::new(rel_path).to_path_buf();

            match file_util::get_path_files(&path_buf, program_opts.recursive, "lua", luapp::CFG_PREFIX) {
                Ok(file_paths) => {
                    for file_path in &file_paths {
                        process_file_path(file_path, &config, &program_opts);
                    }
                }
                Err(_) => eprintln!("Unresolved path: `{}`", rel_path),
            }
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
    let po = ProgramOpts { inplace: true, recursive: true, ..ProgramOpts::default() };
    assert_eq!(parse_options(&options), (cfg, po));

    let options = vec!["-i".to_string(), "--recursive".to_string(), "--newline_format_if=1".to_string()];
    let cfg = Config { newline_format_if: Some(1), ..Config::default() };
    let po = ProgramOpts { inplace: true, recursive: true, ..ProgramOpts::default() };
    assert_eq!(parse_options(&options), (cfg, po));

    let options = vec!["--lines=1:324".to_string()];
    let cfg = Config { ..Config::default() };
    let po = ProgramOpts { lines: Some((1, 324)), ..ProgramOpts::default() };
    assert_eq!(parse_options(&options), (cfg, po));

    let options = vec![];
    let cfg = Config { ..Config::default() };
    let po = ProgramOpts::default();
    assert_eq!(parse_options(&options), (cfg, po));
}
