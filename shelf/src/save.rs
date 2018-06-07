use std::fs;
use std::fs::File;
use std::io;
use std::path;

use serde_yaml;

use shelf::Shelf;

pub struct DirectoryShelf {
    directory: path::PathBuf,
}

#[derive(Debug)]
pub enum SaveError {
    DirectoryError(String),
    SerializationError(String),
}

impl From<io::Error> for SaveError {
    fn from(err: io::Error) -> Self {
        SaveError::DirectoryError(format!("{}", err))
    }
}

impl From<serde_yaml::Error> for SaveError {
    fn from(err: serde_yaml::Error) -> Self {
        SaveError::SerializationError(format!("{}", err))
    }
}

impl ::std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ::std::error::Error for SaveError {
}

impl DirectoryShelf {
    pub fn new<P: Into<path::PathBuf>>(p: P) -> Result<DirectoryShelf, SaveError> {
        let path = p.into();

        if !path.is_dir() {
            return Err(SaveError::DirectoryError(
                format!("Path {} is not a directory.", path.to_string_lossy())
            ));
        }
        if !path.is_absolute() {
            return Err(SaveError::DirectoryError(
                format!("Path {} should be an absolute path.", path.to_string_lossy())
            ));
        }

        let path = path.canonicalize()?;

        fs::create_dir_all(&path);

        Ok(DirectoryShelf {
            directory: path,
        })
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

    pub fn load(&self, shelf: &mut Shelf) -> Result<(), SaveError> {
        let mut people: Vec<::common::Person> = vec![];
        let mut items: Vec<::item::Item> = vec![];

        for entry in self.directory.read_dir()? {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with("person--") {
                        let file = File::open(entry.path())?;
                        people.push(serde_yaml::from_reader(file)?);
                    }
                    else if name.starts_with("item--") {
                        let file = File::open(entry.path())?;
                        items.push(serde_yaml::from_reader(file)?);
                    }
                }
            }
        }

        people.into_iter().for_each(|p| shelf.insert_person(p));
        items.into_iter().for_each(|p| shelf.insert_item(p).unwrap());

        Ok(())
    }
}
