extern crate actix;
extern crate actix_web;
extern crate shelf;

use std::sync::{Arc, RwLock};

use actix_web::{App, HttpRequest, http, server};

struct AppState {
    shelf: Arc<RwLock<shelf::Shelf>>,
}

fn shutdown(req: HttpRequest<AppState>) -> String {
    actix::Arbiter::system().do_send(actix::msgs::SystemExit(0));

    "Goodbye!".to_owned()
}

fn main() {
    server::new(
        || App::with_state(AppState { shelf: Arc::new(RwLock::new(shelf::Shelf::new())) })
            .resource("/shutdown", |r| r.method(http::Method::POST).f(shutdown))
    ).bind("127.0.0.1:8088").expect("Could not bind to port 8088").run();
}
