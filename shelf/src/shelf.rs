use common::Person;
use item::Item;
use series::Series;

#[derive(Debug)]
pub enum ShelfError {
    InvalidReference,
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

    pub fn query_people(&self) {
    }

    pub fn query_series(&self) {
    }

    pub fn insert_person(&mut self) {
    }

    pub fn insert_series(&mut self) {
    }

    pub fn insert_item(&mut self, item: Item) -> Result<()> {
        for (_, person) in item.people.iter() {
            let mut found = false;
            for person2 in self.people.iter() {
                if person == &person2.key {
                    found = true;
                    break;
                }
            }
            if !found {
                return Err(ShelfError::InvalidReference);
            }
        }

        self.items.push(item);
        Ok(())
    }
}
