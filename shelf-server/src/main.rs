extern crate actix;
extern crate actix_web;
extern crate app_dirs;
extern crate clap;
extern crate futures;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;
extern crate shelf;

use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};

use actix_web::Result as ActixResult;
use actix_web::{App, AsyncResponder, HttpMessage, HttpRequest, HttpResponse, Json, Path, error, http, server};
use futures::Future;

struct AppState {
    shelf: Arc<RwLock<shelf::Shelf>>,
    path: std::path::PathBuf,
}

impl AppState {
    fn read_shelf<'a>(&'a self) -> ActixResult<impl Deref<Target=shelf::Shelf> + 'a> {
        self.shelf.read().map_err(|_| error::ErrorInternalServerError("Could not acquire lock on shelf"))
    }

    fn write_shelf<'a>(&'a self) -> ActixResult<impl DerefMut<Target=shelf::Shelf> + 'a> {
        self.shelf.write().map_err(|_| error::ErrorInternalServerError("Could not acquire lock on shelf"))
    }

    fn save<'a>(&'a self) -> ActixResult<()> {
        let mut shelf_ref = self.write_shelf().unwrap();
        match shelf::save::DirectoryShelf::new(&self.path) {
            Ok(saver) => {
                if let Err(err) = saver.save(&mut shelf_ref) {
                    println!("Error while saving: {:?}", err);
                    Err(error::ErrorInternalServerError(format!("Error while saving: {:?}", err)))
                }
                else {
                    Ok(())
                }
            },
            Err(err) => {
                println!("Error while saving: {:?}", err);
                Err(error::ErrorInternalServerError(format!("Error while saving: {:?}", err)))
            },
        }
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

    req.state().save()?;

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

    req.state().save()?;

    result
}

fn people(req: HttpRequest<AppState>) -> ActixResult<Json<Vec<shelf::common::Person>>> {
    let shelf = req.state().read_shelf()?;
    let items = shelf.query_people().map(|p| p.clone()).collect();
    Ok(Json(items))
}

fn put_person(params: (Json<shelf::common::Person>, HttpRequest<AppState>)) -> ActixResult<String> {
    let (person, req) = params;
    {
        let mut shelf = req.state().write_shelf()?;
        shelf.insert_person(person.clone());
    }
    req.state().save()?;
    Ok("created".to_owned())
}

#[derive(Deserialize)]
struct ProxyParams {
    url: String,
}

fn proxy(url: actix_web::Query<ProxyParams>) -> Box<Future<Item = HttpResponse, Error = error::Error>> {
    actix_web::client::ClientRequest::get(url.into_inner().url)
        .finish().unwrap()
        .send()
        .map_err(|e| {
            println!("{:?}", e);
            error::Error::from(e)
        })
        .and_then(
            |resp| resp.body()
                .from_err()
                .and_then(|body| {
                    Ok(HttpResponse::Ok().body(body))
                }))
        .responder()
}

const APP_INFO : app_dirs::AppInfo = app_dirs::AppInfo {
    name: "shelf",
    author: "lidavidm",
};

fn main() -> Result<(), Box<::std::error::Error>> {
    let library_root = app_dirs::app_dir(app_dirs::AppDataType::UserConfig, &APP_INFO, "shelf")?;
    let path = library_root.clone();

    let mut shelf = shelf::Shelf::new();

    // Load the shelf, but discard the saver, since it can't be moved
    // across threads
    {
        println!("Opening shelf: {}", library_root.to_string_lossy());
        let saver = shelf::save::DirectoryShelf::new(&library_root)?;
        saver.load(&mut shelf)?;
    }

    let shelf_ref = Arc::new(RwLock::new(shelf));

    let shr = shelf_ref.clone();

    server::new(
        move || App::with_state(AppState { shelf: shelf_ref.clone(), path: path.clone() })
            .handler(
                "/static",
                actix_web::fs::StaticFiles::new("./static").index_file("index.html"))
            .resource("/shutdown", |r| r.method(http::Method::POST).f(shutdown))
            .resource("/proxy", |r| r.method(http::Method::GET).with(proxy))
            .resource("/item/{key}", |r| {
                r.method(http::Method::GET).with(item);
                r.method(http::Method::POST).with(edit_item)
                    .1.error_handler(|err, req| {
                        println!("{:?}", err);
                        error::ErrorBadRequest(format!("{:?}", err))
                    });
            })
            .resource("/item", |r| {
                r.method(http::Method::PUT).with(begin)
                    .0.error_handler(|err, req| {
                        println!("{:?}", err);
                        error::ErrorBadRequest(format!("{:?}", err))
                    });
                r.method(http::Method::GET).with(items);
            })
            .resource("/person", |r| {
                r.method(http::Method::PUT).with(put_person);
                r.method(http::Method::GET).with(people);
            })
    ).bind("127.0.0.1:8088").expect("Could not bind to port 8088").run();

    {
        let saver = shelf::save::DirectoryShelf::new(&library_root)?;
        saver.save(&mut shr.write().unwrap())?;
    }

    Ok(())
}
