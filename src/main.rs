use std::io::{Read, self};

mod parser;

fn main() -> Result<(), std::io::Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;

    match parser::parse(&buffer) {
        Ok(result) => println!("{}", result),
        Err(err) => println!("Error: {}", err),
    }
    Ok(())
}
