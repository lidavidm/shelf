use chrono;
use chrono::DateTime;

use common::Alternatives;
use common::PersonIdx;
use common::Role;
use common::Status;

#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct Item {
    pub key: String,
    pub name: Alternatives<String>,
    pub people: Vec<(Role, PersonIdx)>,
    pub season: Option<String>,
    pub entries: Vec<Entry>,
    pub status: Status,
    pub rating: Option<u32>,
    pub added: DateTime<chrono::FixedOffset>,
}

#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct Entry {
    pub name: Alternatives<String>,
    pub number: Option<u32>,
    pub volume: Option<u32>,
    // started, completed
}
