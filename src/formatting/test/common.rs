use crate::config::*;
use crate::formatting::reconstruction;
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
        Ok(mut node_tree) => {
            let mut state = State::default();

            reconstruction::update_indexes(&source, &mut state);
            // println!("{:?}", state.chars_to_bytes);
            reconstruction::reconstruct_node_tree(&mut node_tree, cfg, &mut state);

            let mut output = String::new();

            match node_tree.configured_write(&mut output, cfg, source, &mut state) {
                Ok(_) => Ok(output),
                _ => Err(TestError::ErrorWhileWriting),
            }
        }
    }
}
