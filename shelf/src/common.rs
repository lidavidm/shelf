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

#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct Person {
    pub name: Alternatives<String>,
}

pub type PersonIdx = usize;
