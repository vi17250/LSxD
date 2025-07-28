use crate::get_dirs;
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

        if deepth > 0 {
            for mut child in &mut self.children {
                match child {
                    Entity::Directory(child) => child.get_children(deepth - 1),
                    Entity::File(_) => (),
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
