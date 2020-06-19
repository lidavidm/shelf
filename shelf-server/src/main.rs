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

use std::sync::Arc;
use tokio::sync::Mutex;

use warp::Filter;

mod handlers;
mod model;
mod routes;

const APP_INFO: app_dirs::AppInfo = app_dirs::AppInfo {
    name: "shelf",
    author: "lidavidm",
};

const LOG_NAME: &'static str = "shelf-server";

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=shelf-server=debug` to see debug logs, this only
        // shows access logs.
        std::env::set_var("RUST_LOG", "shelf-server=info");
    }

    pretty_env_logger::init();

    let library_root = app_dirs::app_dir(app_dirs::AppDataType::UserConfig, &APP_INFO, "shelf")
        .expect("Could not find library root!");

    let mut shelf = shelf::Shelf::new();
    log::info!(
        target: LOG_NAME,
        "Opening shelf: {}",
        library_root.to_string_lossy()
    );
    let saver = shelf::save::DirectoryShelf::new(&library_root).expect("Could not open library");
    saver.load(&mut shelf).expect("Could not load shelf");
    log::info!(
        target: LOG_NAME,
        "Loaded shelf with {} items",
        shelf.all_items().len()
    );
    let shelf_ref = Arc::new(Mutex::new(model::AppState { shelf, saver }));

    let api = routes::api(shelf_ref).with(warp::log(LOG_NAME));
    let static_files = warp::path("static").and(warp::fs::dir("./static"));
    log::info!(target: LOG_NAME, "Starting server on port 8088");
    warp::serve(static_files.or(api))
        .run(([127, 0, 0, 1], 8088))
        .await;

    // TODO: warp::Server::bind_with_graceful_shutdown
    log::info!(target: LOG_NAME, "Exiting...");
}
