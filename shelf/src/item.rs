use common::Alternatives;
use common::PersonIdx;
use common::Role;

#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct Item {
    pub key: String,
    pub name: Alternatives<String>,
    pub people: Vec<(Role, PersonIdx)>,
    pub season: Option<String>,
    pub entries: Vec<Entry>,
    // rating
}

#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct Entry {
    pub name: Alternatives<String>,
    pub number: Option<u32>,
    pub volume: Option<u32>,
    // started, completed
}
