use crate::config::*;
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
            let mut state = State::default();

            match result.configured_write(&mut output, cfg, source, &mut state) {
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
    use lalrpop_util::ParseError;

    let result = parse_comment("a");
    assert!(result.is_err(), "{:?}", result);
    match result.unwrap_err() {
        #[allow(unused_variables)]
        ParseError::User { error } => (),
        _ => assert!(false, "wrong error type"),
    };

    let result = parse_comment("--[[asd");
    assert!(result.is_err(), "{:?}", result);
    match result.unwrap_err() {
        #[allow(unused_variables)]
        ParseError::User { error } => (),
        _ => assert!(false, "wrong error type"),
    };
}

#[test]
fn test_comments() {
    for str in vec![
        "",
        "     ",
        "\n",
        "   \n    ",
        "\n  \n\n\n\n \n\n   \n",
        "--\n",
        "    --\n    ",
        "--2str[][]\n",
        "--[[trtstrst]]",
        "--[=[trtstrst]=]",
        "--[=[]=]",
        "--[========[trtstrst]========]",
        "--[===[trtstrst]====]==]==]=]]]========]==]========]===]",
        "  \n\n --[[123]]   --[[]]   --\n\n\n --324\n ",
        "\n\n --[[123]]   --[[]]   --\n\n\n --324\n",
        "\n\n--[[123]]--[[]]--\n\n\n--324\n\n\n",
        "\n\n--[[123]]--[[]]--\n\n\n--324\n",
    ] {
        assert_eq!(tsdef(str), Ok(str.to_string()));
    }

    // special case
    assert_eq!(tsdef("\n\n--[[123]]--[[]]--\n\n\n--324"), Ok("\n\n--[[123]]--[[]]--\n\n\n--324\n".to_string()));
}

