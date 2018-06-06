use chrono;

use common::Alternatives;
use common::DateBool;
use common::Kind;
use common::PersonIdx;
use common::Role;
use common::Status;

#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct Item {
    pub key: String,
    pub kind: Kind,
    pub name: Alternatives<String>,
    pub people: Vec<(Role, PersonIdx)>,
    pub season: Option<String>,
    pub entries: Vec<Entry>,
    pub status: Status,
    pub rating: Option<u32>,
    pub added: chrono::DateTime<chrono::FixedOffset>,
    #[serde(default)]
    pub started: DateBool,
    #[serde(default)]
    pub completed: DateBool,
}

#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct Entry {
    pub name: Option<Alternatives<String>>,
    pub number: Option<u32>,
    pub volume: Option<u32>,
    pub completed: DateBool,
    // started, completed
}
