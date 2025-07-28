use std::fs::read_dir;
use std::path::PathBuf;

use crate::types::Directory;

pub fn command(root_dir: PathBuf) -> Vec<Directory> {
    let mut dirs: Vec<Directory> = vec![];
    if let Ok(entries) = read_dir(root_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        dirs.push(Directory::new(entry.path()));
                    }
                } else {
                    println!("Couldn't get file type for {:?}", entry.path());
                }
            }
        }
    }
    dirs
}
