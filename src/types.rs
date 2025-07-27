use std::path::PathBuf;

use crate::get_dirs;

#[derive(Clone, Debug)]
pub struct Directory {
    pub path: PathBuf,
    pub children: Vec<Directory>,
}

impl Directory {
    pub fn get_children(&mut self, deepth: i8) {
        let mut deepth = deepth;
        if deepth == 0 {
            return;
        }
        deepth = deepth - 1;
        for child in &mut self.children {
            child.children = get_dirs(child.path.clone());
        }

        if deepth > 0 {
            for mut p in &mut self.children {
                p.get_children(deepth);
            }
        }
    }
}
