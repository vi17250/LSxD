use crate::commands::{get_list, get_size};
use core::cmp::Ordering;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub enum Entity {
    Directory(Directory),
    File(File),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Directory {
    pub path: PathBuf,
    pub human_size: String,
    pub size: usize,
    pub children: Vec<Entity>,
}

impl Directory {
    pub fn new(path: PathBuf) -> Directory {
        let (size, human_size) = get_size::directory(path.clone());
        Directory {
            path: path.clone(),
            human_size,
            size,
            children: Vec::new(),
        }
    }

    pub fn get_children(&mut self, deepth: u8) {
        self.children = get_list::list(self.path.clone());

        if deepth > 0 {
            for child in &mut self.children {
                if let Entity::Directory(child) = child {
                    child.get_children(deepth - 1);
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub struct File {
    pub path: PathBuf,
    pub human_size: String,
    pub size: usize,
}

impl File {
    pub fn new(path: PathBuf) -> File {
        let (size, human_size) = get_size::file(path.clone());
        File {
            path: path.clone(),
            human_size,
            size,
        }
    }
}
