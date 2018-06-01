use common::Alternatives;
use common::PersonIdx;
use common::Role;

pub struct Item {
    pub key: String,
    pub name: Alternatives<String>,
    pub people: Vec<(Role, PersonIdx)>,
    pub season: Option<String>,
    pub entries: Vec<Entry>,
    // rating
}

pub struct Entry {
    name: Alternatives<String>,
    number: Option<u32>,
    volume: Option<u32>,
    // started, completed
}
