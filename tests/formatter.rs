extern crate luafmt;
use luafmt::config::Config;
use luafmt::formatter::*;

use std::fs;
use std::path::Path;

#[test]
fn test_process_file_success_1() {
    let path_buf = Path::new("tests/scripts1/file1.lua").to_path_buf();
    let config = Config::default();
    let actual = process_file(&path_buf, &config, false);
    let expected = fs::read_to_string("tests/scripts1/file1.lua.out").unwrap_or("".to_string());

    assert!(actual.as_ref().ok().is_some(), "{:?}", actual);
    assert_eq!(actual.unwrap(), expected);

    //
    let path_buf = Path::new("tests/scripts1/file1.lua").to_path_buf();
    let config = Config { line_range: Some((7, 7)), ..Config::default() };
    let actual = process_file(&path_buf, &config, false);
    let expected = fs::read_to_string("tests/scripts1/file1.lua.out2").unwrap_or("".to_string());

    assert!(actual.as_ref().ok().is_some(), "{:?}", actual);
    assert_eq!(actual.unwrap(), expected);

    //
    let config = Config { line_range: Some((4, 13)), ..Config::default() };
    let actual = process_file(&path_buf, &config, false);
    let expected = fs::read_to_string("tests/scripts1/file1.lua.out2").unwrap_or("".to_string());

    assert!(actual.as_ref().ok().is_some(), "{:?}", actual);
    assert_eq!(actual.unwrap(), expected);

    //
    let config = Config { line_range: Some((3, 13)), ..Config::default() };
    let actual = process_file(&path_buf, &config, false);
    let expected = fs::read_to_string("tests/scripts1/file1.lua.out").unwrap_or("".to_string());

    assert!(actual.as_ref().ok().is_some(), "{:?}", actual);
    assert_eq!(actual.unwrap(), expected);
}

#[test]
fn test_process_file_success_3() {
    let path_buf = Path::new("tests/scripts3/file1.lua").to_path_buf();
    let config = Config::default();
    let actual = process_file(&path_buf, &config, false);
    let expected = fs::read_to_string("tests/scripts3/file1.lua.out").unwrap_or("".to_string());

    assert!(actual.as_ref().ok().is_some(), "{:?}", actual);
    assert_eq!(actual.unwrap(), expected);

    //
    let path_buf = Path::new("tests/scripts3/file1.lua").to_path_buf();
    let config = Config { line_range: Some((54, 57)), ..Config::default() };
    let actual = process_file(&path_buf, &config, false);
    let expected = fs::read_to_string("tests/scripts3/file1.lua.out3").unwrap_or("".to_string());

    assert!(actual.as_ref().ok().is_some(), "{:?}", actual);
    assert_eq!(actual.unwrap(), expected);

    //
    let config = Config { line_range: Some((16, 17)), ..Config::default() };
    let actual = process_file(&path_buf, &config, false);
    let expected = fs::read_to_string("tests/scripts3/file1.lua.out2").unwrap_or("".to_string());

    assert!(actual.as_ref().ok().is_some(), "{:?}", actual);
    assert_eq!(actual.unwrap(), expected);

    // //
    // let config = Config { line_range: Some((3, 13)), ..Config::default() };
    // let actual = process_file(&path_buf, &config, false);
    // let expected = fs::read_to_string("tests/scripts1/file1.lua.out").unwrap_or("".to_string());
    //
    // assert!(actual.as_ref().ok().is_some(), "{:?}", actual);
    // assert_eq!(actual.unwrap(), expected);
}

#[test]
fn test_process_file_failure() {
    use FormatterError::*;

    // no config
    let path_buf = Path::new("tests/scripts2/no_err.lua").to_path_buf();
    let config = Config::default();
    let actual = process_file(&path_buf, &config, false);
    assert!(actual.is_err());
    assert!(match actual.unwrap_err() {
        NoConfigureFile => true,
        _ => false,
    });

    // invalid config
    let path_buf = Path::new("tests/scripts_err2/1.lua").to_path_buf();
    let config = Config::default();
    let actual = process_file(&path_buf, &config, false);
    assert!(actual.is_err());
    assert!(match actual.unwrap_err() {
        InvalidConfigFile(..) => true,
        _ => false,
    });

    // no such file
    let path_buf = Path::new("tests/scripts1/file0.lua").to_path_buf();
    let config = Config::default();
    let actual = process_file(&path_buf, &config, false);
    assert!(actual.is_err());
    assert!(match actual.unwrap_err() {
        ReadingError => true,
        _ => false,
    });

    // error lua-file syntax
    let path_buf = Path::new("tests/scripts_err/error.lua").to_path_buf();
    let config = Config::default();
    let actual = process_file(&path_buf, &config, false);
    assert!(actual.is_err());
    assert!(match actual.unwrap_err() {
        ParsingError(..) => true,
        _ => false,
    });
}
