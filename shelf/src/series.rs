use common::Alternatives;
use common::PersonIdx;
use common::Role;

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Series {
    pub key: String,
    pub name: Alternatives<String>,
    pub people: Vec<(Role, PersonIdx)>,
}
