extern crate actix;
extern crate actix_web;
extern crate shelf;

use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};

use actix_web::Result as ActixResult;
use actix_web::{App, HttpRequest, Json, error, http, server};

struct AppState {
    shelf: Arc<RwLock<shelf::Shelf>>,
}

impl AppState {
    fn read_shelf<'a>(&'a self) -> ActixResult<impl Deref<Target=shelf::Shelf> + 'a> {
        self.shelf.read().map_err(|_| error::ErrorInternalServerError("Could not acquire lock on shelf"))
    }

    fn write_shelf<'a>(&'a self) -> ActixResult<impl DerefMut<Target=shelf::Shelf> + 'a> {
        self.shelf.write().map_err(|_| error::ErrorInternalServerError("Could not acquire lock on shelf"))
    }
}

fn shutdown(_req: HttpRequest<AppState>) -> String {
    actix::Arbiter::system().do_send(actix::msgs::SystemExit(0));

    "Goodbye!".to_owned()
}

fn begin(params: (Json<shelf::item::Item>, HttpRequest<AppState>)) -> ActixResult<String> {
    let (item, req) = params;
    let mut shelf = req.state().write_shelf()?;
    shelf.insert_item(item.clone()).unwrap();
    Ok("created".to_owned())
}

fn items(req: HttpRequest<AppState>) -> ActixResult<Json<Vec<shelf::item::Item>>> {
    let shelf = req.state().read_shelf()?;
    let items = shelf.query_items().map(|x| x.1.clone()).collect();
    Ok(Json(items))
}

fn main() {
    let shelf_ref = Arc::new(RwLock::new(shelf::Shelf::new()));
    server::new(
        move || App::with_state(AppState { shelf: shelf_ref.clone() })
            .resource("/shutdown", |r| r.method(http::Method::POST).f(shutdown))
            .resource("/item", |r| {
                r.method(http::Method::PUT).with(begin);
                r.method(http::Method::GET).with(items);
            })
    ).bind("127.0.0.1:8088").expect("Could not bind to port 8088").run();
}
