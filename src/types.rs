pub enum Entity {
    Directory,
    File,
}

pub type Lines = Vec<(Entity, String, String)>;
