use std::collections::HashSet;

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
    people: Vec<Person>,
    items: Vec<Item>,
    series: Vec<Series>,
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
        self.people.iter()
    }

    pub fn query_series(&self) -> impl Iterator<Item = &Series> {
        self.series.iter()
    }

    pub fn insert_person(&mut self, person: Person) -> bool {
        self.dirty.insert(person.key.clone());
        for p in self.people.iter_mut() {
            if p.key == person.key {
                *p = person;
                return false;
            }
        }
        self.people.push(person);
        true
    }

    pub fn insert_series(&mut self, series: Series) -> bool {
        self.dirty.insert(series.key.clone());
        for s in self.series.iter_mut() {
            if s.key == series.key {
                *s = series;
                return false;
            }
        }
        self.series.push(series);
        true
    }

    pub fn validate_item(&self, item: &Item) -> Result<()> {
        for (_, person) in item.people.iter() {
            let mut found = false;
            for person2 in self.people.iter() {
                if person == &person2.key {
                    found = true;
                    break;
                }
            }
            if !found {
                return Err(ShelfError::InvalidReference(person.to_owned()));
            }
        }

        if let Some((ref key, _)) = item.series {
            let mut found = false;
            for series in self.series.iter() {
                if key == &series.key {
                    found = true;
                    break;
                }
            }
            if !found {
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
}
