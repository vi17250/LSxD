use std::fs::read_dir;
use std::path::PathBuf;

use crate::types::{Directory, Entity, File};

pub fn list(path: PathBuf) -> Vec<Entity> {
    let mut entities: Vec<Entity> = Vec::new();
    if let Ok(entries) = read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        let directory = Directory::new(entry.path());
                        entities.push(Entity::Directory(directory));
                    } else if file_type.is_file() {
                        let file = File::new(entry.path());
                        entities.push(Entity::File(file));
                    }
                } else {
                    println!("Couldn't get file type for {:?}", entry.path());
                }
            }
        }
    }
    entities
}
