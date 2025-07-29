use crate::output_color::Colored;
use crate::types::{Directory, Entity};

impl Directory {
    pub fn display(&self, increment: u8) {
        let path = self.path.display();
        let size = self.human_size.coloring();
        let picto = String::from("📁");
        tabs(increment);
        println!("{picto} {path} {size} ");
        for child in &self.children {
            match child {
                Entity::Directory(c) => c.display(increment + 1),
                Entity::File(file) => {
                    tabs(increment + 1);
                    let path = file.path.display();
                    let size = file.human_size.coloring();
                    let picto = String::from("📑");
                    println!("{picto} {path} {size} ");
                }
            };
        }
    }
}

fn tabs(increment: u8) {
    let mut i: u8 = 0;
    let separator = "\t";
    while i < increment {
        print!("{}", separator);
        i = i + 1;
    }
}
