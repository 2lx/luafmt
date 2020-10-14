use crate::config::{Config, ConfiguredWrite};
use super::parse_comment;

#[allow(dead_code)]
static CFG_DEFAULT: Config = Config::default();

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
enum TestError {
    ErrorWhileParsing,
    ErrorWhileWriting,
}

#[allow(dead_code)]
fn ts_base(source: &str, cfg: &Config) -> Result<String, TestError> {
    match parse_comment(source) {
        Err(_) => Err(TestError::ErrorWhileParsing),
        Ok(result) => {
            let mut output = String::new();

            match result.configured_write(&mut output, cfg, source) {
                Ok(_) => Ok(output),
                _ => Err(TestError::ErrorWhileWriting),
            }
        }
    }
}

#[allow(dead_code)]
fn tsdef(source: &str) -> Result<String, TestError> {
    ts_base(source, &CFG_DEFAULT)
}

#[test]
fn test_errors() {
    // use lalrpop_util::ParseError;

    // let result = parse_lua("a = 3 + 22 * ? + 65");
    // assert!(result.is_err(), "{:?}", result);
    // match result.unwrap_err() {
    //     #[allow(unused_variables)]
    //     ParseError::User { error } => (),
    //     _ => assert!(false, "wrong error type"),
    // };
    //
    // let result = parse_lua("1++2");
    // assert!(result.is_err(), "{:?}", result);
    // match result.unwrap_err() {
    //     #[allow(unused_variables)]
    //     ParseError::UnrecognizedToken { token: (l, token, r), expected } => (),
    //     _ => assert!(false, "wrong error type"),
    // };
    //
    // let result = parse_lua("1 2");
    // assert!(result.is_err(), "{:?}", result);
    // match result.unwrap_err() {
    //     #[allow(unused_variables)]
    //     ParseError::UnrecognizedToken { token: (_, _, _), expected } => (),
    //     _ => assert!(false, "wrong error type"),
    // };
    //
    // let result = parse_lua("1+");
    // assert!(result.is_err(), "{:?}", result);
    // match result.unwrap_err() {
    //     #[allow(unused_variables)]
    //     ParseError::UnrecognizedToken { token: (_, _, _), expected } => (),
    //     _ => assert!(false, "wrong error type"),
    // };
}

#[test]
fn test_comments() {
    for str in vec![
        "  \n\n --[[123]]   --[[]]   --\n\n\n --324\n "
    ] {
        assert_eq!(tsdef(str), Ok(str.to_string()));
    }
}
