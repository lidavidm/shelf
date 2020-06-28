// Copyright 2020 David Li <li.davidm96@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
//     Unless required by applicable law or agreed to in writing, software
//     distributed under the License is distributed on an "AS IS" BASIS,
//     WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//     See the License for the specific language governing permissions and
//     limitations under the License.

use growler::handler;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

async fn handle_root(_ctx: handler::RequestContext) -> handler::Result {
    Ok(hyper::Response::builder()
        .status(hyper::StatusCode::OK)
        .body(hyper::Body::from("Welcome to Shelf"))?)
}

async fn item_list(_ctx: handler::RequestContext, state: Arc<Mutex<AppState>>) -> handler::Result {
    let shelf = &state.lock().await.shelf;
    let items: Vec<shelf::item::Item> = shelf.query_items().map(|x| x.1.clone()).collect();
    growler::response::json::json(&items)
}

async fn item_get(ctx: handler::RequestContext, state: Arc<Mutex<AppState>>) -> handler::Result {
    if let Some(key) = ctx.route_parts.first() {
        let shelf = &state.lock().await.shelf;
        let item = shelf
            .query_items()
            .filter(|x| &x.1.key == key)
            .map(|x| x.1.clone())
            .next();
        if let Some(item) = item {
            growler::response::json::json(&item)
        } else {
            Ok(hyper::Response::builder()
                .status(hyper::StatusCode::NOT_FOUND)
                .body(hyper::Body::from(format!("Key not found: {}\n", key)))?)
        }
    } else {
        Ok(hyper::Response::builder()
            .status(hyper::StatusCode::BAD_REQUEST)
            .body(hyper::Body::from("Missing key"))?)
    }
}

fn format_error(err: growler::handler::Error) -> handler::Result {
    Ok(hyper::Response::builder()
        .status(hyper::StatusCode::INTERNAL_SERVER_ERROR)
        .body(hyper::Body::from(format!(
            "Could not serialize response: {}",
            err
        )))?)
}

const APP_INFO: app_dirs::AppInfo = app_dirs::AppInfo {
    name: "shelf",
    author: "lidavidm",
};

const LOG_NAME: &'static str = "shelf-server";

type AnyError = Box<dyn ::std::error::Error + Send + Sync>;

struct AppState {
    shelf: shelf::Shelf,
    saver: shelf::save::DirectoryShelf,
}

fn routes(shelf_ref: Arc<Mutex<AppState>>) -> Result<growler::router::Router, AnyError> {
    let mut builder = growler::router::Builder::new();
    builder.add(hyper::Method::GET, "/", handler::simple(handle_root))?;
    builder.add(
        hyper::Method::GET,
        "/item",
        handler::simple(handler::with_error(
            handler::with_state(item_list, shelf_ref.clone()),
            format_error,
        )),
    )?;
    builder.add(
        hyper::Method::GET,
        "/item/?",
        handler::simple(handler::with_error(
            handler::with_state(item_get, shelf_ref.clone()),
            format_error,
        )),
    )?;
    Ok(builder.build())
}

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    if std::env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=shelf-server=debug` to see debug logs, this only
        // shows access logs.
        std::env::set_var("RUST_LOG", "shelf-server=info");
    }
    pretty_env_logger::init();

    let library_root = app_dirs::app_dir(app_dirs::AppDataType::UserConfig, &APP_INFO, "shelf")?;
    let mut shelf = shelf::Shelf::new();
    log::info!(
        target: LOG_NAME,
        "Opening shelf: {}",
        library_root.to_string_lossy()
    );
    let saver = shelf::save::DirectoryShelf::new(&library_root)?;
    saver.load(&mut shelf)?;
    log::info!(
        target: LOG_NAME,
        "Loaded shelf with {} items",
        shelf.all_items().len()
    );
    let shelf_ref = Arc::new(Mutex::new(AppState { shelf, saver }));
    let routes = routes(shelf_ref)?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let mut server = growler::server::Server::new(routes);
    server.run_forever(&addr).await
}

#[cfg(test)]
mod tests {
    use super::{AnyError, AppState};
    use std::sync::Arc;
    use tokio::sync::Mutex;

    struct Shelf {
        tempdir: tempfile::TempDir,
    }

    impl Shelf {
        fn new() -> Result<Shelf, AnyError> {
            Ok(Shelf {
                tempdir: tempfile::Builder::new()
                    .prefix("shelf-server-test")
                    .tempdir()?,
            })
        }

        fn load(&self) -> Result<Arc<Mutex<AppState>>, AnyError> {
            let mut shelf = shelf::Shelf::new();
            let saver = shelf::save::DirectoryShelf::new(&self.tempdir.path())?;
            saver.load(&mut shelf)?;
            Ok(Arc::new(Mutex::new(AppState { shelf, saver })))
        }
    }

    #[test]
    fn item_404() -> Result<(), AnyError> {
        let shelf = Shelf::new()?.load()?;
        let router = super::routes(shelf)?;
        let response = growler::testing::run(&router, hyper::Method::GET, "/item/foo")?;
        assert_eq!(hyper::StatusCode::NOT_FOUND, response.status());
        Ok(())
    }
}
