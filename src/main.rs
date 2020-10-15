use std::io::{self, Read};
use std::env;
use regex::Regex;

mod config;
mod parser;
mod format;
use config::{Config, ConfiguredWrite};

fn read_input() -> Result<String, std::io::Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn parse_args() -> (Vec::<String>, Vec::<String>) {
    let args: Vec<String> = env::args().skip(1).collect();
    let (params, sources): (Vec<_>, Vec<_>) = args.into_iter().partition(|arg| arg.starts_with('-'));

    println!("Params: {:?}", params);
    println!("Sources: {:?}", sources);

    (params, sources)
}

fn args_to_config(params: &Vec::<String>) -> Config {
    let mut config = Config::default();

    for param in params.iter() {
        let re = Regex::new(r"^[-]+([a-zA-Z_0-9]+)=(.*)$").unwrap();
        match re.captures_iter(param).next() {
            Some(cap) => config.set(&cap[1], &cap[2]),
            None => eprintln!("Unrecognized param: `{}`", param),
        }
    }

    config
}

fn main() -> Result<(), std::io::Error> {
    let (params, _sources) = parse_args();
    let config = args_to_config(&params);
    println!("Config: {:?}", config);

    let buffer = read_input()?;
    match parser::parse_lua(&buffer) {
        Ok(node_tree) => {
            let mut output = String::new();
            match node_tree.configured_write(&mut output, &config, &buffer) {
                Err(_) => println!("An error occured while formatting: {:?}", node_tree),
                _ => print!("{}", output),
            };
        }
        Err(err) => println!("An error occured while parsing: {}", err),
    }

    Ok(())
}
