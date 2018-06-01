use common::Person;
use item::Item;
use series::Series;

pub enum ShelfError {
    InvalidReference,
}

pub type Result<T> = ::std::result::Result<T, ShelfError>;

pub struct Shelf {
    people: Vec<Person>,
    items: Vec<Item>,
    series: Vec<Series>,
}

pub struct ItemRef<'a>(&'a Shelf, &'a Item);

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
            if *person >= self.people.len() {
                return Err(ShelfError::InvalidReference);
            }
        }

        self.items.push(item);
        Ok(())
    }
}
