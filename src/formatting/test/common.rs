use crate::config::*;
use crate::parser::parse_lua;

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum TestError {
    ErrorWhileParsing,
    ErrorWhileWriting,
}

#[allow(dead_code)]
pub fn ts_base(source: &str, cfg: &Config) -> Result<String, TestError> {
    match parse_lua(source) {
        Err(_) => Err(TestError::ErrorWhileParsing),
        Ok(result) => {
            let mut output = String::new();
            let mut state = State::default();

            match result.configured_write(&mut output, cfg, source, &mut state) {
                Ok(_) => Ok(output),
                _ => Err(TestError::ErrorWhileWriting),
            }
        }
    }
}
