extern crate app_dirs;
extern crate serde;
extern crate serde_yaml;
extern crate shelf;
extern crate warp;

use std::sync::Arc;
use tokio::sync::Mutex;

use warp::Filter;

// fn shutdown(_req: &HttpRequest<AppState>) -> String {
//     actix::Arbiter::system().do_send(actix::msgs::SystemExit(0));

//     "Goodbye!".to_owned()
// }

// fn begin(params: (Json<shelf::item::Item>, HttpRequest<AppState>)) -> ActixResult<String> {
//     use shelf::shelf::ShelfError;
//     let (item, req) = params;
//     let result = {
//         let mut shelf = req.state().write_shelf()?;
//         shelf
//             .insert_item(item.clone())
//             .map_err(|e| match e {
//                 ShelfError::InvalidReference(r) => error::ErrorInternalServerError(format!(
//                     "Unrecognized reference to entity {}",
//                     r
//                 )),
//             })
//             .map(|_| "created".to_owned())
//     };

//     req.state().save()?;

//     result
// }

// fn item(params: (Path<(String,)>, HttpRequest<AppState>)) -> ActixResult<Json<shelf::item::Item>> {
//     let (path, req) = params;
//     let key = &path.0;
//     let shelf = req.state().read_shelf()?;
//     let item = shelf
//         .query_items()
//         .filter(|x| &x.1.key == key)
//         .map(|x| x.1.clone())
//         .next();
//     if let Some(rec) = item {
//         Ok(Json(rec))
//     } else {
//         Err(error::ErrorNotFound(format!("Key {} not found", key)))
//     }
// }

// fn edit_item(
//     params: (
//         Path<(String,)>,
//         Json<shelf::item::Item>,
//         HttpRequest<AppState>,
//     ),
// ) -> ActixResult<String> {
//     use shelf::shelf::ShelfError;
//     let (_, item, req) = params;
//     let result = {
//         let mut shelf = req.state().write_shelf()?;
//         shelf
//             .replace_item(item.clone())
//             // TODO: factor this out
//             .map_err(|e| match e {
//                 ShelfError::InvalidReference(r) => error::ErrorInternalServerError(format!(
//                     "Unrecognized reference to entity {}",
//                     r
//                 )),
//             })
//             .map(|_| "updated".to_owned())
//     };

//     req.state().save()?;

//     result
// }

// fn put_person(params: (Json<shelf::common::Person>, HttpRequest<AppState>)) -> ActixResult<String> {
//     let (person, req) = params;
//     {
//         let mut shelf = req.state().write_shelf()?;
//         shelf.insert_person(person.clone());
//     }
//     req.state().save()?;
//     Ok("created".to_owned())
// }

// fn put_series(params: (Json<shelf::series::Series>, HttpRequest<AppState>)) -> ActixResult<String> {
//     let (series, req) = params;
//     {
//         let mut shelf = req.state().write_shelf()?;
//         shelf.insert_series(series.clone());
//     }
//     req.state().save()?;
//     Ok("created".to_owned())
// }

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
    // Load the shelf, but discard the saver, since it can't be moved
    // across threads
    {
        log::info!(
            target: LOG_NAME,
            "Opening shelf: {}",
            library_root.to_string_lossy()
        );
        let saver =
            shelf::save::DirectoryShelf::new(&library_root).expect("Could not open library");
        saver.load(&mut shelf).expect("Could not load shelf");
        log::info!(
            target: LOG_NAME,
            "Loaded shelf with {} items",
            shelf.all_items().len()
        );
    }
    let shelf_ref = Arc::new(Mutex::new(shelf));

    let api = routes::api(shelf_ref).with(warp::log(LOG_NAME));
    let static_files = warp::path("static").and(warp::fs::dir("./static"));
    log::info!(target: LOG_NAME, "Starting server on port 8088");
    warp::serve(api.or(static_files))
        .run(([127, 0, 0, 1], 8088))
        .await;

    // TODO: warp::Server::bind_with_graceful_shutdown
    log::info!(target: LOG_NAME, "Exiting...");
}

mod model {
    use std::sync::Arc;
    use tokio::sync::Mutex;
    pub type ShelfDb = Arc<Mutex<shelf::Shelf>>;

    /// The query parameters for /proxy.
    #[derive(serde_derive::Deserialize)]
    pub struct ProxyParams {
        pub url: String,
    }

    #[derive(Debug)]
    pub struct ReqwestError {
        pub error: String,
    }

    impl warp::reject::Reject for ReqwestError {}
}

mod routes {
    use crate::{handlers, model};
    use warp::Filter;

    pub fn api(
        shelf: model::ShelfDb,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        return warp::path!("hello" / String)
            .map(|name| format!("Hello, {}!", name))
            .or(item_list(shelf.clone()))
            .or(person_list(shelf.clone()))
            .or(series_list(shelf.clone()))
            .or(proxy());
    }

    fn with_shelf(
        shelf: model::ShelfDb,
    ) -> impl Filter<Extract = (model::ShelfDb,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || shelf.clone())
    }

    pub fn item_list(
        shelf: model::ShelfDb,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("item")
            .and(warp::get())
            .and(with_shelf(shelf))
            .and_then(handlers::item_list)
    }

    pub fn person_list(
        shelf: model::ShelfDb,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("person")
            .and(warp::get())
            .and(with_shelf(shelf))
            .and_then(handlers::person_list)
    }

    pub fn series_list(
        shelf: model::ShelfDb,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("series")
            .and(warp::get())
            .and(with_shelf(shelf))
            .and_then(handlers::series_list)
    }

    pub fn proxy() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("proxy")
            .and(warp::get())
            .and(warp::query::<model::ProxyParams>())
            .and_then(handlers::proxy)
            .recover(|rej: warp::Rejection| {
                async move {
                    Ok(if let Some(model::ReqwestError { error }) = rej.find() {
                        warp::reply::with_status(error.into(), warp::http::StatusCode::BAD_GATEWAY)
                    } else {
                        warp::reply::with_status(
                            format!("{:?}", rej),
                            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                        )
                    })
                }
            })
    }
}

mod handlers {
    use crate::model;
    use std::convert::Infallible;

    pub async fn item_list(shelf: model::ShelfDb) -> Result<impl warp::Reply, Infallible> {
        let shelf = shelf.lock().await;
        let items: Vec<shelf::item::Item> = shelf.query_items().map(|x| x.1.clone()).collect();
        Ok(warp::reply::json(&items))
    }

    pub async fn person_list(shelf: model::ShelfDb) -> Result<impl warp::Reply, Infallible> {
        let shelf = shelf.lock().await;
        let items: Vec<shelf::common::Person> = shelf.query_people().map(|p| p.clone()).collect();
        Ok(warp::reply::json(&items))
    }

    pub async fn series_list(shelf: model::ShelfDb) -> Result<impl warp::Reply, Infallible> {
        let shelf = shelf.lock().await;
        let items: Vec<shelf::series::Series> = shelf.query_series().map(|p| p.clone()).collect();
        Ok(warp::reply::json(&items))
    }

    pub async fn proxy(params: model::ProxyParams) -> Result<impl warp::Reply, warp::Rejection> {
        log::info!(target: crate::LOG_NAME, "GET /proxy URL: {}", params.url);
        reqwest::get(&params.url)
            .await
            .map_err(|err| {
                warp::reject::custom(model::ReqwestError {
                    error: format!("{}", err),
                })
            })?
            .text()
            .await
            .map_err(|err| {
                warp::reject::custom(model::ReqwestError {
                    error: format!("{}", err),
                })
            })
    }
}

// fn main() -> Result<(), Box<::std::error::Error>> {
//     let library_root = app_dirs::app_dir(app_dirs::AppDataType::UserConfig, &APP_INFO, "shelf")?;
//     let path = library_root.clone();

//     let mut shelf = shelf::Shelf::new();

//     // Load the shelf, but discard the saver, since it can't be moved
//     // across threads
//     {
//         println!("Opening shelf: {}", library_root.to_string_lossy());
//         let saver = shelf::save::DirectoryShelf::new(&library_root)?;
//         saver.load(&mut shelf)?;
//     }

//     let shelf_ref = Arc::new(RwLock::new(shelf));

//     let shr = shelf_ref.clone();

//     server::new(move || {
//         .resource("/shutdown", |r| r.method(http::Method::POST).f(shutdown))
//         .resource("/item/{key}", |r| {
//             r.method(http::Method::GET).with(item);
//             r.method(http::Method::POST).with(edit_item);
//             // .1.error_handler(|err, req| {
//             //     println!("{:?}", err);
//             //     error::ErrorBadRequest(format!("{:?}", err))
//             // });
//         })
//         .resource("/item", |r| {
//             r.method(http::Method::PUT).with(begin);
//             // .0.error_handler(|err, req| {
//             //     println!("{:?}", err);
//             //     error::ErrorBadRequest(format!("{:?}", err))
//             // });
//             r.method(http::Method::GET).with(items);
//         })
//         .resource("/person", |r| {
//             r.method(http::Method::PUT).with(put_person);
//         })
//         .resource("/series", |r| {
//             r.method(http::Method::PUT).with(put_series);
//         })
//     })

//     {
//         let saver = shelf::save::DirectoryShelf::new(&library_root)?;
//         saver.save(&mut shr.write().unwrap())?;
//     }

//     Ok(())
// }
