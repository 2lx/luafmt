use std::io::{self, Read};

mod config;
mod parser;
use config::ConfiguredWrite;

fn read_input() -> Result<String, std::io::Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn main() -> Result<(), std::io::Error> {
    let config = config::Config { indent_width: 4, keep_comments: true };
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
