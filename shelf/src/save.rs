use std::fs;
use std::fs::File;
use std::io;
use std::path;

use git2;
use serde_yaml;

use shelf::Shelf;

pub struct DirectoryShelf {
    directory: path::PathBuf,
    repository: git2::Repository,
}

#[derive(Debug)]
pub enum SaveError {
    DirectoryError(String),
    GitError(String),
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

impl From<git2::Error> for SaveError {
    fn from(err: git2::Error) -> Self {
        SaveError::GitError(format!("{}", err))
    }
}

impl From<path::StripPrefixError> for SaveError {
    fn from(err: path::StripPrefixError) -> Self {
        SaveError::DirectoryError(format!("{}", err))
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

        let _ = fs::create_dir_all(&path);

        let repo = match git2::Repository::open(&path) {
            Ok(repo) => repo,
            Err(_) => {
                let repo = git2::Repository::init(&path)?;

                // https://github.com/alexcrichton/git2-rs/blob/master/examples/init.rs
                let sig = repo.signature()?;

                let tree_id = {
                    let mut index = repo.index()?;
                    index.write_tree()?
                };

                {
                    let tree = repo.find_tree(tree_id)?;
                    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;
                }

                repo
            },
        };

        Ok(DirectoryShelf {
            directory: path,
            repository: repo,
        })
    }

    #[must_use]
    pub fn save(&self, shelf: &mut Shelf) -> Result<usize, SaveError> {
        // TODO: ERROR HANDLING!!
        let sig = self.repository.signature()?;
        let mut index = self.repository.index()?;

        let prev_head_ref = self.repository.head()?;
        let prev_head = self.repository.find_commit(
            prev_head_ref.target().ok_or_else(|| SaveError::GitError("Can't find target of HEAD".to_owned()))?
        )?;

        let wrote = {
            let mut updated = vec![];

            for person in shelf.query_people() {
                if !shelf.is_dirty(&person.key) {
                    continue;
                }
                let filename = format!("person--{}.yaml", person.key);
                let path = self.directory.join(filename);
                let mut file = File::create(&path)?;
                serde_yaml::to_writer(file, person)?;

                index.add_path(&path)?;

                updated.push(&person.key);
            }

            for item in shelf.query_items() {
                if !shelf.is_dirty(&item.1.key) {
                    continue;
                }
                let filename = format!("item--{}.yaml", item.1.key);
                let path = self.directory.join(filename);
                let mut file = File::create(&path)?;
                serde_yaml::to_writer(file, item.1)?;

                index.add_path(path.strip_prefix(&self.directory)?)?;

                updated.push(&item.1.key);
            }

            let tree_id = index.write_tree()?;

            let tree = self.repository.find_tree(tree_id)?;
            if updated.len() > 0 {
                let message = if updated.len() == 1 {
                    format!("Updated \"{}\"", updated[0])
                } else {
                    format!("Update shelf ({} items)", updated.len())
                };
                self.repository.commit(Some("HEAD"), &sig, &sig, &message, &tree, &[&prev_head])?;
            }

            // Preserve the index so that future commits don't forget what
            // happened here
            index.write()?;

            updated.len()
        };

        shelf.clear_all_dirty();

        // TODO: return this info
        println!("Wrote {} entries", wrote);
        Ok(wrote)
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

        shelf.clear_all_dirty();

        Ok(())
    }
}
