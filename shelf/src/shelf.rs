use common::Person;
use item::Item;
use series::Series;

#[derive(Debug)]
pub enum ShelfError {
    InvalidReference(String),
}

pub type Result<T> = ::std::result::Result<T, ShelfError>;

pub struct Shelf {
    people: Vec<Person>,
    items: Vec<Item>,
    series: Vec<Series>,
}

pub struct ItemRef<'a>(pub &'a Shelf, pub &'a Item);

impl Shelf {
    pub fn new() -> Shelf {
        Shelf {
            people: Vec::new(),
            items: Vec::new(),
            series: Vec::new(),
        }
    }

    pub fn query_items(&self) -> impl Iterator<Item=ItemRef> {
        self.items.iter().map(move |item| {
            ItemRef(self, item)
        })
    }

    pub fn query_people(&self) -> impl Iterator<Item=&Person> {
        self.people.iter()
    }

    pub fn query_series(&self) {
    }

    pub fn insert_person(&mut self, person: Person) {
        self.people.push(person);
    }

    pub fn insert_series(&mut self) {
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
        Ok(())
    }

    pub fn insert_item(&mut self, item: Item) -> Result<()> {
        self.validate_item(&item)?;
        self.items.push(item);
        Ok(())
    }

    pub fn replace_item(&mut self, item: Item) -> Result<()> {
        self.validate_item(&item)?;
        let idx = self.items.iter()
            .enumerate()
            .find(|(_, candidate)| item.key == candidate.key)
            .map(|(idx, _)| idx);
        if let Some(idx) = idx {
            self.items[idx] = item;
        }
        else {
            self.insert_item(item);
        }
        Ok(())
    }
}
