extern crate luafmt;

use std::path::Path;

#[test]
fn test_get_path_files() {
    let path_buf = Path::new("tests/scripts1").to_path_buf();
    let mut actual =
        luafmt::file_util::get_path_files(&path_buf, false, "lua", luafmt::CFG_PREFIX).ok().unwrap_or(vec![]);
    actual.sort();
    assert_eq!(
        actual,
        vec![Path::new("tests/scripts1/file1.lua").to_path_buf(), Path::new("tests/scripts1/file2.lua").to_path_buf(),]
    );

    let path_buf = Path::new("tests/scripts1/subdir1/").to_path_buf();
    let mut actual =
        luafmt::file_util::get_path_files(&path_buf, false, "lua", luafmt::CFG_PREFIX).ok().unwrap_or(vec![]);
    actual.sort();
    assert_eq!(actual, vec![Path::new("tests/scripts1/subdir1/file3.lua").to_path_buf()]);

    let path_buf = Path::new("tests/scripts1/subdir1/").to_path_buf();
    let mut actual =
        luafmt::file_util::get_path_files(&path_buf, true, "lua", luafmt::CFG_PREFIX).ok().unwrap_or(vec![]);
    actual.sort();
    assert_eq!(
        actual,
        vec![
            Path::new("tests/scripts1/subdir1/file3.lua").to_path_buf(),
            Path::new("tests/scripts1/subdir1/subdir2/file4.lua").to_path_buf()
        ]
    );

    let path_buf = Path::new("tests/scripts1").to_path_buf();
    let mut actual =
        luafmt::file_util::get_path_files(&path_buf, true, "lua", luafmt::CFG_PREFIX).ok().unwrap_or(vec![]);
    actual.sort();
    assert_eq!(
        actual,
        vec![
            Path::new("tests/scripts1/file1.lua").to_path_buf(),
            Path::new("tests/scripts1/file2.lua").to_path_buf(),
            Path::new("tests/scripts1/subdir1/file3.lua").to_path_buf(),
            Path::new("tests/scripts1/subdir1/subdir2/file4.lua").to_path_buf(),
        ]
    );
}

#[test]
fn test_get_file_config() {
    let path_buf = Path::new("tests/scripts1/file1.lua").to_path_buf();
    let actual = luafmt::file_util::get_file_config(&path_buf, luafmt::CFG_PREFIX);
    assert!(
        actual.as_ref().unwrap().ends_with("tests/scripts1/.luafmt.lua"),
        "Actual path: `{}`",
        actual.as_ref().unwrap().display()
    );

    let path_buf = Path::new("tests/scripts1/subdir1/file3.lua").to_path_buf();
    let actual = luafmt::file_util::get_file_config(&path_buf, luafmt::CFG_PREFIX);
    assert!(
        actual.as_ref().unwrap().ends_with("tests/scripts1/.luafmt.lua"),
        "Actual path: `{}`",
        actual.as_ref().unwrap().display()
    );

    let path_buf = Path::new("tests/scripts1/subdir1/subdir2/file4.lua").to_path_buf();
    let actual = luafmt::file_util::get_file_config(&path_buf, luafmt::CFG_PREFIX);
    assert!(
        actual.as_ref().unwrap().ends_with("tests/scripts1/subdir1/subdir2/.luafmt_inner.lua"),
        "Actual path: `{}`",
        actual.as_ref().unwrap().display()
    );
}
