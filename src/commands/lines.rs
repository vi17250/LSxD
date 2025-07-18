use fs_extra::dir::get_size;
use std::{fs, path::Path};

use crate::types::Line;

pub fn command(directory: Box<Path>) -> Vec<Line> {
    let mut files: Vec<Line> = Vec::new();
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if let Ok(metadata) = entry.metadata() {
                        let path: Box<Path> = entry.path().into();
                        let size = get_size(&path).expect("Failed while get_size");
                        let children = match file_type.is_file() {
                            true => None,
                            _ => Some(Vec::new()),
                        };
                        files.push(Line {
                            path,
                            size,
                            children,
                        });
                    }
                } else {
                    eprintln!("Couldn't get file type for {:?}", entry.path());
                }
            }
        }
    }
    files
}
