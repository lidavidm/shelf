#[macro_use]
extern crate serde_derive;

use std::sync::{Arc, RwLock};
use warp::Filter;

const APP_INFO: app_dirs::AppInfo = app_dirs::AppInfo {
    name: "shelf",
    author: "lidavidm",
};

struct WithTemplate {
    name: &'static str,
    value: tera::Context,
}

fn render(template: WithTemplate, templates: Arc<RwLock<tera::Tera>>) -> impl warp::Reply {
    // TODO
    let _ = templates.write().unwrap().full_reload();
    warp::reply::html(
        templates
            .read()
            .unwrap()
            .render(template.name, &template.value)
            .unwrap_or_else(|err| err.description().to_owned()),
    )
}

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

fn home(shelf: Arc<RwLock<shelf::Shelf>>) -> WithTemplate {
    let mut value = tera::Context::new();
    value.insert("items", shelf.read().unwrap().all_items());
    WithTemplate {
        name: "home.html",
        value,
    }
}

fn in_progress(shelf: Arc<RwLock<shelf::Shelf>>) -> WithTemplate {
    let mut value = tera::Context::new();
    let mut items: Vec<shelf::item::Item> = shelf
        .read()
        .unwrap()
        .query_items()
        .map(|item_ref| item_ref.1)
        .filter(|item| item.status == shelf::common::Status::InProgress)
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
    value.insert("items", &items);
    WithTemplate {
        name: "in-progress.html",
        value,
    }
}

#[derive(Debug, Deserialize)]
struct ItemOptions {
    key: String,
}

fn item(params: ItemOptions, shelf: Arc<RwLock<shelf::Shelf>>) -> WithTemplate {
    let mut value = tera::Context::new();
    let item = shelf
        .read()
        .unwrap()
        .query_items()
        .map(|item_ref| item_ref.1)
        .filter(|item| item.key == params.key)
        .cloned()
        .next();
    if let Some(item) = item {
        value.insert("item", &item);
        WithTemplate {
            name: "item.html",
            value,
        }
    } else {
        WithTemplate {
            name: "404.html",
            value,
        }
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let library_root = app_dirs::app_dir(app_dirs::AppDataType::UserConfig, &APP_INFO, "shelf")?;
    // let path = library_root.clone();

    let mut shelf = shelf::Shelf::new();

    // Load the shelf, but discard the saver, since it can't be moved
    // across threads
    {
        println!("Opening shelf: {}", library_root.to_string_lossy());
        let saver = shelf::save::DirectoryShelf::new(&library_root)?;
        saver.load(&mut shelf)?;
    }

    let shelf_ref = Arc::new(RwLock::new(shelf));
    let shelf_ref = warp::any().map(move || shelf_ref.clone());

    let templates = Arc::new(RwLock::new(tera::compile_templates!("templates/**/*")));
    templates
        .write()
        .unwrap()
        .register_filter("count_completed", count_completed);
    let templates = warp::any().map(move || templates.clone());

    let root = warp::path::end();
    let view_home = warp::get2()
        .and(root)
        .and(shelf_ref.clone())
        .map(home)
        .and(templates.clone())
        .map(render);
    let view_in_progress = warp::get2()
        .and(warp::path("in-progress").and(warp::path::end()))
        .and(shelf_ref.clone())
        .map(in_progress)
        .and(templates.clone())
        .map(render);
    let view_item = warp::get2()
        .and(warp::path("item").and(warp::path::end()))
        .and(warp::query::<ItemOptions>())
        .and(shelf_ref.clone())
        .map(item)
        .and(templates.clone())
        .map(render);
    let views = view_home.or(view_in_progress).or(view_item);

    let assets = warp::path("static").and(warp::fs::dir("static"));

    warp::serve(views.or(assets)).run(([127, 0, 0, 1], 8081));

    Ok(())
}
