use common::Alternatives;
use common::PersonIdx;
use common::Role;

pub struct Series {
    key: String,
    name: Alternatives<String>,
    people: Vec<(Role, PersonIdx)>,
}
