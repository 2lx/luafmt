use std::ffi::OsStr;
use std::fs;
use std::io::{self};
use std::path::PathBuf;

pub fn get_path_files(path: &PathBuf, recursive: bool) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let inner_path = entry?.path();

            if inner_path.is_dir() && recursive {
                paths.append(&mut get_path_files(&inner_path, true)?);
            } else if inner_path.is_file() && inner_path.extension().and_then(OsStr::to_str) == Some("lua") {
                paths.push(inner_path);
            }
        }
    } else if path.is_file() {
        paths.push(path.to_path_buf());
    }

    Ok(paths)
}
