use crate::types::{Directory, Entity, File};
use std::{cmp::Ordering, fmt::Display};

impl Ord for Entity {
    fn cmp(&self, other: &Self) -> Ordering {
        Ordering::Less
    }
}

#[allow(warnings)]
mod test {
    use std::path::PathBuf;

    use crate::types::{Directory, Entity};

    #[test]
    fn it_returns_itself() {
        let sub_dir_1: Directory = Directory {
            path: PathBuf::new(),
            human_size: "sub_dir_1".to_string(),
            size: 1,
            children: Vec::new(),
        };

        let sub_dir_2: Directory = Directory {
            path: PathBuf::new(),
            human_size: "sub_dir_2".to_string(),
            size: 2,
            children: Vec::new(),
        };

        let dir: Entity = Entity::Directory(Directory {
            path: PathBuf::new(),
            human_size: "Human size".to_string(),
            size: 1,
            children: vec![
                Entity::Directory(sub_dir_2.clone()),
                Entity::Directory(sub_dir_1.clone()),
            ],
        });

        let sorted: Entity = Entity::Directory(Directory {
            path: PathBuf::new(),
            human_size: "Human size".to_string(),
            size: 1,
            children: vec![Entity::Directory(sub_dir_2), Entity::Directory(sub_dir_1)],
        });

        assert_eq!(dir, sorted);
    }
}
