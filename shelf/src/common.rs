use std::collections::HashMap;

pub struct Alternatives<T> {
    default: T,
    alternatives: HashMap<String, T>,
}

pub enum Role {
    Author,
}

pub struct Person {
    name: Alternatives<String>,
}

pub type PersonIdx = usize;
