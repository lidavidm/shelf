use common::Alternatives;
use common::PersonIdx;
use common::Role;

pub struct Series {
    name: Alternatives<String>,
    people: Vec<(Role, PersonIdx)>,
}
