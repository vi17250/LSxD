use crate::commands::{get_dirs, get_files};
use std::path::PathBuf;

#[derive(Debug)]
pub enum Entity {
    Directory(Directory),
    File(File),
}

#[derive(Debug)]
pub struct Directory {
    pub path: PathBuf,
    pub size: usize,
    pub children: Vec<Entity>,
}

impl Directory {
    pub fn new(path: PathBuf) -> Directory {
        Directory {
            path: path.clone(),
            size: get_dirs::size(path.clone()),
            children: Vec::new(),
        }
    }

    pub fn get_children(&mut self, deepth: u8) {
        self.children = get_dirs::list(self.path.clone());
        let mut files = get_files::list(self.path.clone());

        self.children.append(&mut files);

        if deepth > 0 {
            for child in &mut self.children {
                if let Entity::Directory(child) = child {
                    child.get_children(deepth - 1);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct File {
    pub path: PathBuf,
    pub size: usize,
}

impl File {
    pub fn new(path: PathBuf) -> File {
        File {
            path: path.clone(),
            size: get_files::size(path.clone()),
        }
    }
}
