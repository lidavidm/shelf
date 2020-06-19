// Copyright 2020 David Li <li.davidm96@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
//     Unless required by applicable law or agreed to in writing, software
//     distributed under the License is distributed on an "AS IS" BASIS,
//     WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//     See the License for the specific language governing permissions and
//     limitations under the License.

use std::fs;
use std::fs::File;
use std::io;
use std::path;

use git2;
use serde_yaml;

use crate::shelf::Shelf;

pub struct DirectoryShelf {
    directory: path::PathBuf,
    repository: git2::Repository,
}

#[derive(Debug)]
pub enum SaveError {
    DirectoryError(String),
    GitError(String),
    SerializationError(String),
    MissingBlob(String),
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

impl ::std::error::Error for SaveError {}

impl DirectoryShelf {
    pub fn new<P: Into<path::PathBuf>>(p: P) -> Result<DirectoryShelf, SaveError> {
        let path = p.into();

        if !path.is_dir() {
            return Err(SaveError::DirectoryError(format!(
                "Path {} is not a directory.",
                path.to_string_lossy()
            )));
        }
        if !path.is_absolute() {
            return Err(SaveError::DirectoryError(format!(
                "Path {} should be an absolute path.",
                path.to_string_lossy()
            )));
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
            }
        };

        Ok(DirectoryShelf {
            directory: path,
            repository: repo,
        })
    }

    /// Save a binary blob into the shelf.
    ///
    /// Given an identifier, returns the absolute path where the blob should be saved.
    /// The key must already be normalized (a valid path).
    #[must_use]
    pub fn insert_blob(&self, key: &str) -> Result<path::PathBuf, SaveError> {
        let blobs = self.directory.join("blobs");
        fs::create_dir_all(&blobs)?;
        let path = blobs.join(key);
        Ok(path)
    }

    #[must_use]
    pub fn save(&self, shelf: &mut Shelf) -> Result<usize, SaveError> {
        let sig = self.repository.signature()?;
        let mut index = self.repository.index()?;

        let prev_head_ref = self.repository.head()?;
        let prev_head = self.repository.find_commit(
            prev_head_ref
                .target()
                .ok_or_else(|| SaveError::GitError("Can't find target of HEAD".to_owned()))?,
        )?;

        let wrote = {
            let mut updated = vec![];

            for person in shelf.query_people() {
                if !shelf.is_dirty(&person.key) {
                    continue;
                }
                let filename = format!("person--{}.yaml", person.key);
                let path = self.directory.join(filename);
                let file = File::create(&path)?;
                serde_yaml::to_writer(&file, person)?;
                file.sync_all()?;

                index.add_path(path.strip_prefix(&self.directory)?)?;

                updated.push(&person.key);
            }

            for series in shelf.query_series() {
                if !shelf.is_dirty(&series.key) {
                    continue;
                }
                let filename = format!("series--{}.yaml", series.key);
                let path = self.directory.join(filename);
                let file = File::create(&path)?;
                serde_yaml::to_writer(&file, series)?;
                file.sync_all()?;

                index.add_path(path.strip_prefix(&self.directory)?)?;

                updated.push(&series.key);
            }

            for item in shelf.query_items() {
                if !shelf.is_dirty(&item.1.key) {
                    continue;
                }
                let filename = format!("item--{}.yaml", item.1.key);
                let path = self.directory.join(filename);
                let file = File::create(&path)?;
                serde_yaml::to_writer(&file, item.1)?;
                file.sync_all()?;

                index.add_path(path.strip_prefix(&self.directory)?)?;

                updated.push(&item.1.key);
            }

            for blob in shelf.query_blobs() {
                if !shelf.is_dirty(blob) {
                    continue;
                }
                let filename = self.insert_blob(blob)?;
                if !filename.exists() {
                    return Err(SaveError::MissingBlob(blob.to_owned()));
                }
                index.add_path(filename.strip_prefix(&self.directory)?)?;
                updated.push(blob);
            }

            let tree_id = index.write_tree()?;

            let tree = self.repository.find_tree(tree_id)?;
            if !updated.is_empty() {
                let message = if updated.len() == 1 {
                    format!("Updated \"{}\"", updated[0])
                } else {
                    let mut buf = String::new();
                    buf.push_str(&format!("Update shelf ({} items)\n\n", updated.len()));
                    for key in updated.iter() {
                        buf.push_str(&format!("- {}\n", key));
                    }
                    buf
                };
                self.repository
                    .commit(Some("HEAD"), &sig, &sig, &message, &tree, &[&prev_head])?;
            }

            // Preserve the index so that future commits don't forget what
            // happened here
            index.write()?;

            updated.len()
        };

        shelf.clear_all_dirty();

        log::info!("Wrote {} entries", wrote);
        Ok(wrote)
    }

    pub fn load(&self, shelf: &mut Shelf) -> Result<(), SaveError> {
        let mut people: Vec<crate::common::Person> = vec![];
        let mut items: Vec<crate::item::Item> = vec![];
        let mut series: Vec<crate::series::Series> = vec![];

        for entry in self.directory.read_dir()? {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with("person--") {
                        let file = File::open(entry.path())?;
                        people.push(serde_yaml::from_reader(file)?);
                    } else if name.starts_with("item--") {
                        let file = File::open(entry.path())?;
                        items.push(serde_yaml::from_reader(file)?);
                    } else if name.starts_with("series--") {
                        let file = File::open(entry.path())?;
                        series.push(serde_yaml::from_reader(file)?);
                    }
                }
            }
        }

        people.into_iter().for_each(|p| {
            let _ = shelf.insert_person(p);
        });
        series.into_iter().for_each(|p| {
            let _ = shelf.insert_series(p);
        });
        items
            .into_iter()
            .for_each(|p| shelf.insert_item(p).unwrap());

        shelf.clear_all_dirty();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::DirectoryShelf;
    use crate::shelf::Shelf;
    use std::fs::File;
    use tempfile::Builder;

    #[test]
    fn insert_blob() {
        let tmp_dir = Builder::new()
            .prefix("shelf-test-")
            .tempdir()
            .expect("Could not make temp dir");
        let saver = DirectoryShelf::new(tmp_dir.path()).expect("Could not make temp shelf");
        let blob = saver
            .insert_blob("cover-foo")
            .expect("Could not insert blob");
        assert!(blob.ends_with("blobs/cover-foo"));

        let mut shelf = Shelf::new();
        assert!(saver.save(&mut shelf).is_ok());
        shelf.insert_blob("cover-foo");
        assert!(saver.save(&mut shelf).is_err());
        File::create(&blob)
            .expect(&format!("Could not create {:?}", blob))
            .sync_all()
            .expect(&format!("Could not create {:?}", blob));
        assert!(saver.save(&mut shelf).is_ok());
    }
}
