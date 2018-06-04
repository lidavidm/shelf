extern crate actix;
extern crate actix_web;
extern crate shelf;

use std::sync::{Arc, RwLock};

use actix_web::{App, HttpRequest, Json, http, server};

struct AppState {
    shelf: Arc<RwLock<shelf::Shelf>>,
}

fn shutdown(req: HttpRequest<AppState>) -> String {
    actix::Arbiter::system().do_send(actix::msgs::SystemExit(0));

    "Goodbye!".to_owned()
}

fn begin(params: (Json<shelf::item::Item>, HttpRequest<AppState>)) -> String {
    let (item, req) = params;
    req.state().shelf.write().unwrap().insert_item(item.clone()).unwrap();
    "created".to_owned()
}

fn items(req: HttpRequest<AppState>) -> Json<Vec<shelf::item::Item>> {
    let shelf = req.state().shelf.read().unwrap();
    let items = shelf.query_items().map(|x| x.1.clone()).collect();
    Json(items)
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
