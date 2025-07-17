use std::fs;

use crate::types::File;

pub fn command(directory: String) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Ok(metadata) = entry.metadata() {
                            files.push(File {
                                name: entry
                                    .file_name()
                                    .into_string()
                                    .expect("Error while parsing OsString to String"),
                                size: metadata.len().to_string(),
                            });
                        }
                    }
                } else {
                    eprintln!("Couldn't get file type for {:?}", entry.path());
                }
            }
        }
    }
    files
}
