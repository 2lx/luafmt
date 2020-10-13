use std::io::{self, Read};

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

fn main() -> Result<(), std::io::Error> {
    let config = Config { field_separator: Some(","), trailing_field_separator: Some(true), ..Config::default() };
    let buffer = read_input()?;

    match parser::parse(&buffer) {
        Ok(result) => {
            let mut output = String::new();
            match result.configured_write(&mut output, &config, &buffer) {
                Err(_) => println!("An error occured while formatting: {:?}", result),
                _ => println!("{}", output),
            };
        }
        Err(err) => println!("An error occured while parsing: {}", err),
    }

    Ok(())
}
