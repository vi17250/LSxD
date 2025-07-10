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

pub type Lines = Vec<(Entity, String, String)>;
