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

fn handle_root(ctx: &handler::RequestContext) -> hyper::Response<hyper::Body> {
    hyper::Response::builder()
        .status(hyper::StatusCode::OK)
        .body(hyper::Body::from("Welcome to Shelf"))
        .expect("Unable to create `http::Response`")
}

const APP_INFO: app_dirs::AppInfo = app_dirs::AppInfo {
    name: "shelf",
    author: "lidavidm",
};

const LOG_NAME: &'static str = "shelf-server";

struct AppState {
    shelf: shelf::Shelf,
    saver: shelf::save::DirectoryShelf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
    let shelf_ref = Arc::new(Mutex::new(AppState { shelf, saver }));

    let mut routes = growler::router::Router::new();
    routes.add(hyper::Method::GET, "/", handle_root);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let mut server = growler::server::Server::new(routes);
    server.run_forever(&addr).await
}
