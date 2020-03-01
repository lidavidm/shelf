use chrono;
use serde_yaml;

use crate::common::Alternatives;
use crate::common::DateBool;
use crate::common::Kind;
use crate::common::PersonIdx;
use crate::common::Role;
use crate::common::Status;

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum PublicationStatus {
    Publishing,
    Complete,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Item {
    pub key: String,
    pub kind: Kind,
    pub name: Alternatives<String>,
    pub people: Vec<(Role, PersonIdx)>,
    pub season: Option<String>,
    pub entries: Vec<Entry>,
    pub status: Status,
    pub rating: Option<u32>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub added: chrono::DateTime<chrono::FixedOffset>,
    #[serde(default)]
    pub started: DateBool,
    #[serde(default)]
    pub completed: DateBool,
    #[serde(default = "default_extras")]
    pub extra: serde_yaml::Value,
    pub publication_status: PublicationStatus,
    // series key, series index/entry name
    pub series: Option<(String, Option<String>)>,
}

impl Default for Item {
    fn default() -> Self {
        Item {
            key: "".into(),
            kind: Kind::Manga,
            name: Alternatives::new("English", ""),
            people: Vec::new(),
            season: None,
            entries: Vec::new(),
            status: Status::Planned,
            rating: None,
            tags: Vec::new(),
            added: chrono::prelude::Local::now().into(),
            started: DateBool::False,
            completed: DateBool::False,
            extra: serde_yaml::Value::Null,
            publication_status: PublicationStatus::Publishing,
            series: None,
        }
    }
}

fn default_extras() -> serde_yaml::Value {
    serde_yaml::Value::Null
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Entry {
    pub name: Option<Alternatives<String>>,
    pub number: Option<u32>,
    pub volume: Option<u32>,
    pub completed: DateBool,
    #[serde(default = "default_extras")]
    pub extra: serde_yaml::Value,
}
