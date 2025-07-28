use crate::get_dirs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Directory {
    pub path: PathBuf,
    pub children: Vec<Directory>,
}

impl Directory {
    pub fn new(path: PathBuf) -> Directory {
        Directory {
            path: path.clone(),
            children: Vec::new(),
        }
    }

    pub fn get_children(&mut self, deepth: u8) {
        self.children = get_dirs::command(self.path.clone());

        if deepth > 0 {
            for mut child in &mut self.children {
                child.get_children(deepth - 1);
            }
        }
    }
}
