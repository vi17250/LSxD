use std::fs::read_dir;
use std::path::PathBuf;
use std::process::Command;
use std::str::from_utf8;

use crate::types::{Directory, Entity};

pub fn list(path: PathBuf) -> Vec<Entity> {
    let mut dirs: Vec<Entity> = Vec::new();
    if let Ok(entries) = read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        let directory = Directory::new(entry.path());
                        dirs.push(Entity::Directory(directory));
                    }
                } else {
                    println!("Couldn't get file type for {:?}", entry.path());
                }
            }
        }
    }
    dirs
}

pub fn size(path: PathBuf) -> usize {
    let program = "du";
    let command = Command::new(program)
        .arg(path)
        .arg("-d 0")
        .arg("--block-size=1")
        .output()
        .expect("WTF")
        .stdout;

    let line = from_utf8(&command)
        .expect("Failed to read command")
        .split_once("\t")
        .expect("Failes to read size");

    let size = line.0.parse::<usize>().expect("Failed to parse size");
    size
}
