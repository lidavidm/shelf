use std::fs;
use std::fs::File;
use std::path;

use serde_yaml;

use shelf::Shelf;

pub struct DirectoryShelf {
    directory: path::PathBuf,
}

impl DirectoryShelf {
    pub fn new<P: Into<path::PathBuf>>(p: P) -> DirectoryShelf {
        // TODO: check is_dir, is_absolute, canonicalize, etc.
        DirectoryShelf {
            directory: p.into(),
        }
    }

    pub fn save(&self, shelf: &Shelf) {
        fs::create_dir_all(&self.directory);

        for person in shelf.query_people() {
            let filename = format!("person--{}.yaml", person.key);
            let path = self.directory.join(filename);
            let mut file = File::create(path).unwrap();
            serde_yaml::to_writer(file, person).unwrap();
        }

        for item in shelf.query_items() {
            let filename = format!("item--{}.yaml", item.1.key);
            let path = self.directory.join(filename);
            let mut file = File::create(path).unwrap();
            serde_yaml::to_writer(file, item.1).unwrap();
        }
    }
}
