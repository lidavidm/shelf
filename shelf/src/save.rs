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

    pub fn load(&self, shelf: &mut Shelf) {
        // TODO: error handling, denesting
        let mut people: Vec<::common::Person> = vec![];
        let mut items: Vec<::item::Item> = vec![];

        for entry in self.directory.read_dir().unwrap() {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with("person--") {
                        let file = File::open(entry.path()).unwrap();
                        people.push(serde_yaml::from_reader(file).unwrap());
                    }
                    else if name.starts_with("item--") {
                        let file = File::open(entry.path()).unwrap();
                        items.push(serde_yaml::from_reader(file).unwrap());
                    }
                }
            }
        }

        people.into_iter().for_each(|p| shelf.insert_person(p));
        items.into_iter().for_each(|p| shelf.insert_item(p).unwrap());
    }
}
