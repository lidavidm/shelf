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

use std::collections::{HashMap, HashSet};

use crate::common::Person;
use crate::item::Item;
use crate::series::Series;

#[derive(Debug)]
pub enum ShelfError {
    InvalidReference(String),
}

pub type Result<T> = ::std::result::Result<T, ShelfError>;

#[derive(Default)]
pub struct Shelf {
    people: HashMap<String, Person>,
    items: Vec<Item>,
    series: HashMap<String, Series>,
    dirty: HashSet<String>,
}

pub struct ItemRef<'a>(pub &'a Shelf, pub &'a Item);

impl Shelf {
    pub fn new() -> Shelf {
        Default::default()
    }

    pub fn all_items(&self) -> &[Item] {
        &self.items
    }

    pub fn query_items(&self) -> impl Iterator<Item = ItemRef> {
        self.items.iter().map(move |item| ItemRef(self, item))
    }

    pub fn query_people(&self) -> impl Iterator<Item = &Person> {
        self.people.values()
    }

    pub fn query_series(&self) -> impl Iterator<Item = &Series> {
        self.series.values()
    }

    /// Insert or update a person.
    ///
    /// Returns true if the person did not previously exist.
    pub fn insert_person(&mut self, person: Person) -> bool {
        self.dirty.insert(person.key.clone());
        self.people.insert(person.key.clone(), person).is_none()
    }

    /// Insert or update a series.
    ///
    /// Returns true if the series did not previously exist.
    pub fn insert_series(&mut self, series: Series) -> bool {
        // TODO: validate people
        self.dirty.insert(series.key.clone());
        self.series.insert(series.key.clone(), series).is_none()
    }

    pub fn validate_item(&self, item: &Item) -> Result<()> {
        for (_, person) in item.people.iter() {
            if !self.people.contains_key(person) {
                return Err(ShelfError::InvalidReference(person.to_owned()));
            }
        }

        if let Some((ref key, _)) = item.series {
            if !self.series.contains_key(key) {
                return Err(ShelfError::InvalidReference(key.to_owned()));
            }
        }

        Ok(())
    }

    pub fn insert_item(&mut self, item: Item) -> Result<()> {
        self.validate_item(&item)?;
        self.dirty.insert(item.key.clone());
        self.items.push(item);
        Ok(())
    }

    pub fn replace_item(&mut self, item: Item) -> Result<()> {
        self.validate_item(&item)?;
        let idx = self
            .items
            .iter()
            .enumerate()
            .find(|(_, candidate)| item.key == candidate.key)
            .map(|(idx, _)| idx);
        if let Some(idx) = idx {
            self.dirty.insert(item.key.clone());
            self.items[idx] = item;
        } else {
            return self.insert_item(item);
        }
        Ok(())
    }

    pub fn is_dirty(&self, key: &str) -> bool {
        self.dirty.contains(key)
    }

    pub fn clear_dirty(&mut self, key: &str) {
        self.dirty.remove(key);
    }

    pub fn clear_all_dirty(&mut self) {
        self.dirty.clear()
    }
}

#[cfg(test)]
mod tests {
    use super::Shelf;
    use crate::common::{Alternatives, Person};
    use crate::series::Series;

    #[test]
    fn test_shelf_insert_person() {
        let mut shelf = Shelf::new();
        let person = Person {
            key: "person-makoto-shinkai".into(),
            name: Alternatives::new("English", "Makoto Shinkai"),
        };

        assert_eq!(0, shelf.query_people().count());

        assert!(shelf.insert_person(person.clone()));
        assert_eq!(1, shelf.query_people().count());
        assert_eq!(Some(&person), shelf.query_people().next());

        // Overwrite, don't append
        assert!(!shelf.insert_person(person.clone()));
        assert_eq!(1, shelf.query_people().count());
        assert_eq!(Some(&person), shelf.query_people().next());

        // Overwrite with new person
        let person_updated = Person {
            key: "person-makoto-shinkai".into(),
            name: Alternatives::new("English", "The Best Director"),
        };
        assert!(!shelf.insert_person(person_updated.clone()));
        assert_eq!(1, shelf.query_people().count());
        assert_eq!(Some(&person_updated), shelf.query_people().next());

        // Add new person
        let someone_else = Person {
            key: "person-mizu-sahara".into(),
            name: Alternatives::new("English", "The Best Mangaka"),
        };
        assert!(shelf.insert_person(someone_else.clone()));
        assert_eq!(2, shelf.query_people().count());
    }

    #[test]
    fn test_shelf_insert_series() {
        let mut shelf = Shelf::new();
        let series = Series {
            key: "series-the-best".into(),
            name: Alternatives::new("Japanese (Romaji)", "Kara no Kyoukai"),
            people: Vec::new(),
        };

        assert_eq!(0, shelf.query_series().count());

        assert!(shelf.insert_series(series.clone()));
        assert_eq!(1, shelf.query_series().count());
        assert_eq!(Some(&series), shelf.query_series().next());

        // Overwrite, don't append
        assert!(!shelf.insert_series(series.clone()));
        assert_eq!(1, shelf.query_series().count());
        assert_eq!(Some(&series), shelf.query_series().next());

        // Overwrite with new series
        let series_updated = Series {
            key: "series-the-best".into(),
            name: Alternatives::new("English", "Garden of Sinners"),
            people: Vec::new(),
        };
        assert!(!shelf.insert_series(series_updated.clone()));
        assert_eq!(1, shelf.query_series().count());
        assert_eq!(Some(&series_updated), shelf.query_series().next());

        // Add new series
        let someone_else = Series {
            key: "series-the-poppy-war".into(),
            name: Alternatives::new("English", "The Poppy War"),
            people: Vec::new(),
        };
        assert!(shelf.insert_series(someone_else.clone()));
        assert_eq!(2, shelf.query_series().count());
    }
}
