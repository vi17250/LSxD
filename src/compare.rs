use crate::types::Entity;
use std::cmp::Ordering;

impl Eq for Entity {}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.get_size() == other.get_size()
    }
}

impl Ord for Entity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_size().cmp(&other.get_size())
    }
}

impl PartialOrd for Entity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.get_size().cmp(&other.get_size()))
    }
}

#[allow(warnings)]
mod test {
    use std::path::PathBuf;

    use crate::types::{Directory, Entity};

    #[test]
    fn it_sorts_entities() {
        let entity_1: Directory = Directory {
            path: PathBuf::new(),
            human_size: "sub_dir_1".to_string(),
            size: 1,
            children: Vec::new(),
        };

        let entity_2: Directory = Directory {
            path: PathBuf::new(),
            human_size: "sub_dir_2".to_string(),
            size: 2,
            children: Vec::new(),
        };

        let entities_1 = vec![
            Entity::Directory(entity_1.clone()),
            Entity::Directory(entity_2.clone()),
        ];

        let mut entities_2 = vec![
            Entity::Directory(entity_2.clone()),
            Entity::Directory(entity_1.clone()),
        ];

        entities_2.sort();

        assert_eq!(entities_1, entities_2);
    }
}
