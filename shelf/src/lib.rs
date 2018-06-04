extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

pub mod common;
pub mod item;
pub mod series;
pub mod shelf;

pub use shelf::Shelf;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use ::item;
        use ::common;
        use std::collections::HashMap;
        use serde_yaml;

        let mut alts = HashMap::new();
        alts.insert("Japanese (Romaji)".to_owned(), "Hourou Musuko".to_owned());
        alts.insert("English".to_owned(), "Wandering Son".to_owned());
        let item = item::Item {
            key: "hourou-musuko".to_owned(),
            name: common::Alternatives {
                default: "Japanese (Romaji)".to_owned(),
                alternatives: alts,
            },
            people: vec![],
            season: Some("1".to_owned()),
            entries: vec![],
        };
        println!("{}", serde_yaml::to_string(&item).unwrap());
        assert_eq!(2 + 2, 4);
    }
}
