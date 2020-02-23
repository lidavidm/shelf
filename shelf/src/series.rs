use crate::common::Alternatives;
use crate::common::PersonIdx;
use crate::common::Role;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Series {
    pub key: String,
    pub name: Alternatives<String>,
    pub people: Vec<(Role, PersonIdx)>,
}
