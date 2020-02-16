use std::collections::HashMap;

const APP_INFO: app_dirs::AppInfo = app_dirs::AppInfo {
    name: "shelf",
    author: "lidavidm",
};

fn count_completed(
    value: tera::Value,
    _args: std::collections::HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let entries = match value {
        tera::Value::Array(ref entries) => entries,
        tera::Value::Object(ref item) => {
            if let Some(tera::Value::Array(entries)) = item.get("entries") {
                entries
            } else {
                return tera::Result::Err(tera::Error::from_kind(tera::ErrorKind::Msg(
                    "Object has no entries".to_owned(),
                )));
            }
        }
        _ => {
            return tera::Result::Err(tera::Error::from_kind(tera::ErrorKind::Msg(
                "Object has no entries".to_owned(),
            )))
        }
    };
    let mut count = 0;
    for entry in entries {
        if let tera::Value::Object(obj) = entry {
            if let Some(val) = obj.get("completed") {
                match val {
                    tera::Value::Bool(false) | tera::Value::Null => {}
                    _ => count += 1,
                }
            }
        }
    }
    Ok(tera::Value::Number(count.into()))
}

fn make_err(s: impl Into<String>) -> tera::Error {
    tera::Error::from_kind(tera::ErrorKind::Msg(s.into()))
}

fn lookup(args: std::collections::HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let collection = match args
        .get("collection")
        .ok_or_else(|| make_err("lookup: must provide collection"))?
    {
        tera::Value::Object(obj) => Ok(obj),
        _ => Err(make_err("lookup: collection must be an object")),
    }?;
    let key = match args
        .get("key")
        .ok_or_else(|| make_err("lookup: must provide key"))?
    {
        tera::Value::String(key) => Ok(key),
        _ => Err(make_err("lookup: key must be a string")),
    }?;
    Ok(collection
        .get(key)
        .ok_or_else(|| make_err("lookup: key not found"))?
        .clone())
}

fn format_kind(args: std::collections::HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let kind = match args
        .get("kind")
        .ok_or_else(|| make_err("format_kind: must provide kind"))?
    {
        tera::Value::String(s) => Ok(s),
        _ => Err(make_err("format_kind: kind must be a string")),
    }?;

    let kind = match kind.as_ref() {
        "ShortStory" => "Short Story".to_owned(),
        "VisualNovel" => "Visual Novel".to_owned(),
        otherwise => otherwise.to_owned(),
    };

    Ok(tera::Value::String(kind))
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let library_root = app_dirs::app_dir(app_dirs::AppDataType::UserConfig, &APP_INFO, "shelf")?;
    let mut shelf = shelf::Shelf::new();

    // Load the shelf, but discard the saver, since it can't be moved
    // across threads
    {
        eprintln!("Opening shelf: {}", library_root.to_string_lossy());
        let saver = shelf::save::DirectoryShelf::new(&library_root)?;
        saver.load(&mut shelf)?;
    }

    let mut templates = tera::compile_templates!("templates/**/*");
    templates.register_filter("count_completed", count_completed);
    templates.register_function("lookup", Box::new(lookup));
    templates.register_function("format_kind", Box::new(format_kind));

    let mut value = tera::Context::new();
    let mut items: Vec<shelf::item::Item> = shelf
        .query_items()
        .map(|item_ref| item_ref.1)
        .filter(|item| {
            item.status == shelf::common::Status::Completed
                && (item.kind == shelf::common::Kind::Novel
                    || item.kind == shelf::common::Kind::Play
                    || item.kind == shelf::common::Kind::Play
                    || item.kind == shelf::common::Kind::Collection
                    || item.kind == shelf::common::Kind::ShortStory)
        })
        .cloned()
        .collect();
    items.sort_by_key(|item| {
        let def = &item.name.default;
        (
            format!("{:?}", item.kind),
            item.name
                .alternatives
                .get(def)
                .map(Clone::clone)
                .unwrap_or("".to_owned()),
        )
    });

    let mut people: HashMap<String, shelf::common::Person> = HashMap::new();
    for person in shelf.query_people() {
        people.insert(person.key.clone(), person.clone());
    }

    value.insert("items", &items);
    value.insert("people", &people);
    println!("{}", templates.render("bookshelf.md", &value)?);

    Ok(())
}
