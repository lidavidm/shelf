use std::collections::HashMap;
use std::fmt;

use chrono;
use serde::{Deserialize,Deserializer,Serialize,Serializer};
use serde::de;

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
    OVA,
    ONA,
    Music,
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum DateBool {
    False,
    True,
    Timestamp(chrono::DateTime<chrono::FixedOffset>),
    Date(chrono::naive::NaiveDate),
}

impl Default for DateBool {
    fn default() -> DateBool {
        DateBool::False
    }
}

impl Serialize for DateBool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        match self {
            DateBool::False => serializer.serialize_none(),
            DateBool::True => serializer.serialize_bool(true),
            DateBool::Timestamp(t) => t.serialize(serializer),
            DateBool::Date(t) => t.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for DateBool {
    fn deserialize<D>(deserializer: D) -> Result<DateBool, D::Error>
    where D: Deserializer<'de> {
        struct DateBoolVisitor;

        impl<'d> de::Visitor<'d> for DateBoolVisitor {
            type Value = DateBool;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a null, Boolean, or a string indicating a day or timestamp")
            }

            fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
                Ok(if v {DateBool::True} else {DateBool::False})
            }

            fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
                Ok(DateBool::False)
            }

            fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
                self.visit_none()
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                if let Ok(ts) = chrono::DateTime::parse_from_rfc3339(v) {
                    Ok(DateBool::Timestamp(ts))
                }
                else if let Ok(ts) = chrono::naive::NaiveDate::parse_from_str(v, "%Y/%m/%d") {
                    Ok(DateBool::Date(ts))
                }
                else {
                    Err(E::custom(format!("unrecognized date(time) format: {}", v)))
                }
            }
        }

        deserializer.deserialize_any(DateBoolVisitor)
    }
}
