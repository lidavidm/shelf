extern crate actix;
extern crate actix_web;
extern crate shelf;

use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};

use actix_web::Result as ActixResult;
use actix_web::{App, HttpRequest, Json, Path, error, http, server};

struct AppState {
    shelf: Arc<RwLock<shelf::Shelf>>,
    saver: Arc<RwLock<shelf::save::DirectoryShelf>>,
}

impl AppState {
    fn read_shelf<'a>(&'a self) -> ActixResult<impl Deref<Target=shelf::Shelf> + 'a> {
        self.shelf.read().map_err(|_| error::ErrorInternalServerError("Could not acquire lock on shelf"))
    }

    fn write_shelf<'a>(&'a self) -> ActixResult<impl DerefMut<Target=shelf::Shelf> + 'a> {
        self.shelf.write().map_err(|_| error::ErrorInternalServerError("Could not acquire lock on shelf"))
    }

    fn save(&self) {
        let shelf = self.read_shelf().unwrap();
        let saver = self.saver.read().unwrap();
        saver.save(&shelf);
    }
}

fn shutdown(_req: HttpRequest<AppState>) -> String {
    actix::Arbiter::system().do_send(actix::msgs::SystemExit(0));

    "Goodbye!".to_owned()
}

fn begin(params: (Json<shelf::item::Item>, HttpRequest<AppState>)) -> ActixResult<String> {
    use shelf::shelf::ShelfError;
    let (item, req) = params;
    let result = {
        let mut shelf = req.state().write_shelf()?;
        shelf.insert_item(item.clone())
            .map_err(|e| match e {
                ShelfError::InvalidReference(r) =>
                    error::ErrorInternalServerError(format!("Unrecognized reference to entity {}", r)),
            })
            .map(|_| "created".to_owned())
    };
    req.state().save();

    result
}

fn items(req: HttpRequest<AppState>) -> ActixResult<Json<Vec<shelf::item::Item>>> {
    let shelf = req.state().read_shelf()?;
    let items = shelf.query_items().map(|x| x.1.clone()).collect();
    Ok(Json(items))
}

fn item(params: (Path<(String,)>, HttpRequest<AppState>)) -> ActixResult<Json<shelf::item::Item>> {
    let (path, req) = params;
    let key = &path.0;
    let shelf = req.state().read_shelf()?;
    let item = shelf.query_items()
        .filter(|x| &x.1.key == key)
        .map(|x| x.1.clone()).next();
    if let Some(rec) = item {
        Ok(Json(rec))
    }
    else {
        Err(error::ErrorNotFound(format!("Key {} not found", key)))
    }
}

fn edit_item(params: (Path<(String,)>,
                      Json<shelf::item::Item>,
                      HttpRequest<AppState>)) -> ActixResult<String> {
    use shelf::shelf::ShelfError;
    let (_, item, req) = params;
    let result = {
        let mut shelf = req.state().write_shelf()?;
        shelf.replace_item(item.clone())
        // TODO: factor this out
            .map_err(|e| match e {
                ShelfError::InvalidReference(r) =>
                    error::ErrorInternalServerError(format!("Unrecognized reference to entity {}", r)),
            })
            .map(|_| "updated".to_owned())
    };

    req.state().save();

    result
}


fn people(req: HttpRequest<AppState>) -> ActixResult<Json<Vec<shelf::common::Person>>> {
    let shelf = req.state().read_shelf()?;
    let items = shelf.query_people().map(|p| p.clone()).collect();
    Ok(Json(items))
}

fn put_person(params: (Json<shelf::common::Person>, HttpRequest<AppState>)) -> ActixResult<String> {
    let (person, req) = params;
    let mut shelf = req.state().write_shelf()?;
    shelf.insert_person(person.clone());
    Ok("created".to_owned())
}

fn main() -> Result<(), Box<::std::error::Error>> {
    let mut shelf = shelf::Shelf::new();
    let saver = shelf::save::DirectoryShelf::new("/home/lidavidm/Code/shelf/shelf-server/temp/")?;
    saver.load(&mut shelf);
    let shelf_ref = Arc::new(RwLock::new(shelf));
    let saver_ref = Arc::new(RwLock::new(saver));
    server::new(
        move || App::with_state(AppState { shelf: shelf_ref.clone(), saver: saver_ref.clone() })
            .handler(
                "/static",
                actix_web::fs::StaticFiles::new("./static"))
            .resource("/shutdown", |r| r.method(http::Method::POST).f(shutdown))
            .resource("/item/{key}", |r| {
                r.method(http::Method::GET).with(item);
                r.method(http::Method::POST).with(edit_item);
            })
            .resource("/item", |r| {
                r.method(http::Method::PUT).with(begin);
                r.method(http::Method::GET).with(items);
            })
            .resource("/person", |r| {
                r.method(http::Method::PUT).with(put_person);
                r.method(http::Method::GET).with(people);
            })
    ).bind("127.0.0.1:8088").expect("Could not bind to port 8088").run();

    Ok(())
}
