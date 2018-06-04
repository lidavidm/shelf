use std::collections::HashMap;

#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct Alternatives<T> {
    pub default: String,
    pub alternatives: HashMap<String, T>,
}

#[derive(Clone,Copy,PartialEq,Eq,Serialize,Deserialize,Debug)]
pub enum Role {
    Author,
}

pub type PersonIdx = String;

#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct Person {
    pub key: PersonIdx,
    pub name: Alternatives<String>,
}

#[derive(Clone,Copy,Debug,PartialEq,Eq,Deserialize,Serialize)]
pub enum Status {
    Completed,
    InProgress,
    Planned,
    OnHold,
    Dropped,
}

#[derive(Clone,Copy,Debug,PartialEq,Eq,Deserialize,Serialize)]
pub enum Kind {
    Unknown,
    Manga,
    TV,
    Film,
    Book,
}
