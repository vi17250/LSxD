#[derive(Debug, PartialEq)]
pub enum Entity {
    Directory,
    File,
}

impl Entity {
    pub fn display(&self) -> String {
        match &self {
            Entity::File => String::from("📑"),
            Entity::Directory => String::from("📁"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct File {
    pub name: String,
    pub size: String,
}

#[derive(Debug)]
pub struct Directory {
    pub path: String,
    pub size: String,
    pub directories: Vec<Directory>,
    pub files: Vec<File>,
}
