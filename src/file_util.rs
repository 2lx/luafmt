use std::ffi::OsStr;
use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::{Path, PathBuf};

pub fn get_path_files(path: &PathBuf, ext: &str, recursive: bool) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let inner_path = entry?.path();

            if inner_path.is_dir() && recursive {
                paths.append(&mut get_path_files(&inner_path, ext, true)?);
            } else if inner_path.is_file() && inner_path.extension().and_then(OsStr::to_str) == Some(ext) {
                paths.push(inner_path);
            }
        }
    } else if path.is_file() {
        paths.push(path.to_path_buf());
    } else {
        return Err(Error::new(ErrorKind::Other, ""));
    }

    Ok(paths)
}

pub fn test_file_in_dir(path: &Path, file_prefix: &str, file_ext: &str) -> io::Result<Option<PathBuf>> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let inner_path = entry?.path();
            if inner_path.is_file()
                && inner_path.file_name().and_then(OsStr::to_str).and_then(|s| Some(s.starts_with(file_prefix)))
                    == Some(true)
                && inner_path.extension().and_then(OsStr::to_str) == Some(file_ext)
            {
                return Ok(Some(inner_path));
            }
        }
    }
    Ok(None)
}

pub fn get_file_config(file_path: &PathBuf) -> Option<PathBuf> {
    match fs::canonicalize(file_path) {
        Ok(mut cur_file_path) => {
            while let Some(parent_path) = cur_file_path.parent() {
                match test_file_in_dir(&parent_path, ".luafmt", "lua") {
                    Ok(Some(path)) => return Some(path),
                    _ => cur_file_path = parent_path.to_path_buf(),
                }
            };
            None
        }
        _ => None,
    }
}
