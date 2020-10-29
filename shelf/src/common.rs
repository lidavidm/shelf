// Copyright 2020 David Li <li.davidm96@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
//     Unless required by applicable law or agreed to in writing, software
//     distributed under the License is distributed on an "AS IS" BASIS,
//     WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//     See the License for the specific language governing permissions and
//     limitations under the License.

use std::collections::HashMap;
use std::fmt;

use chrono;
use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A reusable container for multiple named values for the same key.
///
/// For instance, for multi-lingual titles, this can be used to store
/// different translations of the same title.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Alternatives<T: PartialEq> {
    pub default: String,
    pub alternatives: HashMap<String, T>,
}

impl<T: PartialEq> Alternatives<T> {
    pub fn new<S, S0>(default: S, value: S0) -> Alternatives<T>
    where
        S: Into<String>,
        S0: Into<T>,
    {
        let default = default.into();
        let mut alternatives = HashMap::new();
        alternatives.insert(default.clone(), value.into());
        Alternatives {
            default,
            alternatives,
        }
    }
}

/// The role for a person associated with a work.
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Role {
    Artist,
    Author,
    Director,
    Translator,
}

pub type PersonIdx = String;

/// A person associated with potentially many works.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Person {
    pub key: PersonIdx,
    pub name: Alternatives<String>,
}

/// The read/watch status of an item.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Status {
    Completed,
    InProgress,
    Planned,
    OnHold,
    Dropped,
}

/// The type of a work.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Kind {
    Unknown,
    Manga,
    TV,
    Film,
    Novel,
    /// Original Video Animation
    OVA,
    /// Original Net Animation
    ONA,
    /// Music Video
    Music,
    /// Stage Play
    Play,
    /// A collection of works.
    Collection,
    /// A short story, published standalone.
    ShortStory,
    Musical,
    VisualNovel,
    NonFiction,
}

/// A binary blob stored in a shelf (for things like cover images).
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Blob {
    pub key: String,
    pub mime_type: String,
}

impl Blob {
    pub fn new_with_mime(key: String, mime_type: String) -> Blob {
        Blob { key, mime_type }
    }
}

/// A hybrid date or Boolean, used to import data from sites that
/// offer only a Boolean completed flag instead of a completion date,
/// or a less granular completion date.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateBool {
    False,
    True,
    Timestamp(chrono::DateTime<chrono::FixedOffset>),
    Date(chrono::naive::NaiveDate),
    YearMonth(u32, u32),
}

impl Default for DateBool {
    fn default() -> DateBool {
        DateBool::False
    }
}

impl Serialize for DateBool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DateBool::False => serializer.serialize_none(),
            DateBool::True => serializer.serialize_bool(true),
            DateBool::Timestamp(t) => t.serialize(serializer),
            DateBool::Date(t) => t.serialize(serializer),
            DateBool::YearMonth(year, month) => {
                serializer.serialize_str(&format!("{}-{:02}-00", year, month))
            }
        }
    }
}

impl<'de> Deserialize<'de> for DateBool {
    fn deserialize<D>(deserializer: D) -> Result<DateBool, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DateBoolVisitor;

        impl<'d> de::Visitor<'d> for DateBoolVisitor {
            type Value = DateBool;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a null, Boolean, or a string indicating a day or timestamp")
            }

            fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
                Ok(if v { DateBool::True } else { DateBool::False })
            }

            fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
                Ok(DateBool::False)
            }

            fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
                self.visit_none()
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                if v == "0000-00-00" {
                    // MyAnimeList export does this
                    Ok(DateBool::False)
                } else if let Ok(ts) = chrono::DateTime::parse_from_rfc3339(v) {
                    Ok(DateBool::Timestamp(ts))
                } else if let Ok(ts) = chrono::naive::NaiveDate::parse_from_str(v, "%Y/%m/%d") {
                    Ok(DateBool::Date(ts))
                } else if let Ok(ts) = chrono::naive::NaiveDate::parse_from_str(v, "%Y-%m-%d") {
                    Ok(DateBool::Date(ts))
                } else {
                    let parts: Vec<&str> = v.split('-').collect();
                    if parts.len() == 3 {
                        let year = parts[0].parse::<u32>();
                        let month = parts[1].parse::<u32>();
                        let day = parts[2].parse::<u32>();
                        if let (Ok(y), Ok(m), Ok(d)) = (year, month, day) {
                            if y <= 9999 && m >= 1 && m <= 12 && d == 0 {
                                return Ok(DateBool::YearMonth(y, m));
                            }
                        }
                    }
                    Err(E::custom(format!("unrecognized date(time) format: {}", v)))
                }
            }
        }

        deserializer.deserialize_any(DateBoolVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::DateBool;
    use chrono::{DateTime, NaiveDate};
    use serde_test::{assert_de_tokens, assert_tokens, Token};

    fn assert_ser_de<'de, T>(value: &T, tokens: &'de [Token])
    where
        T: serde::Serialize + serde::Deserialize<'de> + PartialEq + std::fmt::Debug,
    {
        assert_tokens(value, tokens);
        assert_de_tokens(value, tokens);
    }

    #[test]
    fn test_datebool_ser_de() {
        assert_ser_de(&DateBool::False, &[Token::None]);
        assert_ser_de(&DateBool::True, &[Token::Bool(true)]);
        assert_de_tokens(&DateBool::False, &[Token::String("0000-00-00")]);
        assert_de_tokens(&DateBool::False, &[Token::Bool(false)]);
        assert_de_tokens(&DateBool::False, &[Token::Unit]);

        assert_ser_de(
            &DateBool::Timestamp(
                DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00").unwrap(),
            ),
            &[Token::String("1996-12-19T16:39:57-08:00")],
        );
        assert_ser_de(
            &DateBool::Date(NaiveDate::from_ymd(2018, 1, 1)),
            &[Token::String("2018-01-01")],
        );
        assert_ser_de(
            &DateBool::YearMonth(2018, 1),
            &[Token::String("2018-01-00")],
        );
    }
}
