use chrono;
use serde_yaml;

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
    #[serde(default = "default_extras")]
    pub extra: serde_yaml::Value,
}

fn default_extras() -> serde_yaml::Value {
    serde_yaml::Value::Null
}

#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct Entry {
    pub name: Option<Alternatives<String>>,
    pub number: Option<u32>,
    pub volume: Option<u32>,
    pub completed: DateBool,
    #[serde(default = "default_extras")]
    pub extra: serde_yaml::Value,
}
