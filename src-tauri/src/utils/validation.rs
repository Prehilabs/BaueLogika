use std::{fs::read_dir, path::PathBuf};

pub fn problem_path_is_valid(path : &PathBuf) -> bool
{
    let dir_files = read_dir(path);
    return dir_files.is_ok();
}