use fs_extra::dir::get_size;
use std::fs::read_dir;
use std::path::PathBuf;

use crate::types::{Entity, File};

pub fn list(path: PathBuf) -> Vec<Entity> {
    let mut files: Vec<Entity> = Vec::new();
    if let Ok(entries) = read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        let file = File::new(entry.path());
                        files.push(Entity::File(file));
                    }
                } else {
                    println!("Couldn't get file type for {:?}", entry.path());
                }
            }
        }
    }
    files
}

pub fn size(path: PathBuf) -> usize {
    let size = get_size(path)
        .expect("Failed to get size")
        .try_into()
        .expect("Failed to parse size");
    size
}
