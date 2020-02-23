extern crate app_dirs;
extern crate serde;
extern crate serde_yaml;
extern crate shelf;
extern crate warp;

use std::sync::Arc;
use tokio::sync::Mutex;

use warp::Filter;

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

mod model {
    use std::sync::Arc;
    use tokio::sync::Mutex;

    pub struct AppState {
        pub shelf: shelf::Shelf,
        pub saver: shelf::save::DirectoryShelf,
    }

    impl AppState {
        pub fn save(&mut self) -> Result<(), warp::Rejection> {
            if let Err(err) = self.saver.save(&mut self.shelf) {
                log::error!(target: crate::LOG_NAME, "Error while saving: {}", err);
                Err(warp::reject::custom(InternalServerError::new(format!(
                    "Error while saving: {}",
                    err
                ))))
            } else {
                Ok(())
            }
        }
    }

    pub type AppStateRef = Arc<Mutex<AppState>>;

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

    #[derive(Debug)]
    pub struct BadRequest {
        pub error: String,
    }

    impl warp::reject::Reject for BadRequest {}

    #[derive(Debug)]
    pub struct InternalServerError {
        pub error: String,
    }

    impl InternalServerError {
        pub fn new<S: Into<String>>(s: S) -> InternalServerError {
            InternalServerError { error: s.into() }
        }
    }

    impl warp::reject::Reject for InternalServerError {}

    #[derive(Debug, serde_derive::Serialize)]
    pub enum CreateStatus {
        Created,
        Updated,
    }

    #[derive(Debug, serde_derive::Serialize)]
    pub struct CreateResponse {
        pub key: String,
    }
}

mod routes {
    use crate::{handlers, model};
    use warp::Filter;

    pub fn api(
        shelf: model::AppStateRef,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("hello" / String)
            .map(|name| format!("Hello, {}!", name))
            .or(item_list(shelf.clone()))
            .or(item_get(shelf.clone()))
            .or(item_post(shelf.clone()))
            .or(person_list(shelf.clone()))
            .or(person_create(shelf.clone()))
            .or(series_list(shelf.clone()))
            .or(series_create(shelf.clone()))
            .or(proxy())
            .recover(error_handler)
    }

    fn with_shelf(
        shelf: model::AppStateRef,
    ) -> impl Filter<Extract = (model::AppStateRef,), Error = std::convert::Infallible> + Clone
    {
        warp::any().map(move || shelf.clone())
    }

    pub fn item_list(
        shelf: model::AppStateRef,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("item")
            .and(warp::get())
            .and(with_shelf(shelf))
            .and_then(handlers::item_list)
    }

    pub fn item_get(
        shelf: model::AppStateRef,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("item" / String)
            .and(warp::get())
            .and(with_shelf(shelf))
            .and_then(handlers::item_get)
    }

    pub fn item_post(
        shelf: model::AppStateRef,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("item" / String)
            .and(warp::post())
            .and(warp::body::content_length_limit(4 * 1024 * 1024).and(warp::body::json()))
            .and(with_shelf(shelf))
            .and_then(handlers::item_post)
    }

    pub fn person_list(
        shelf: model::AppStateRef,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("person")
            .and(warp::get())
            .and(with_shelf(shelf))
            .and_then(handlers::person_list)
    }

    pub fn person_create(
        shelf: model::AppStateRef,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("person")
            .and(warp::put())
            .and(warp::body::content_length_limit(16 * 1024).and(warp::body::json()))
            .and(with_shelf(shelf))
            .and_then(handlers::person_create)
    }

    pub fn series_list(
        shelf: model::AppStateRef,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("series")
            .and(warp::get())
            .and(with_shelf(shelf))
            .and_then(handlers::series_list)
    }

    pub fn series_create(
        shelf: model::AppStateRef,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("series")
            .and(warp::put())
            .and(warp::body::content_length_limit(16 * 1024).and(warp::body::json()))
            .and(with_shelf(shelf))
            .and_then(handlers::series_create)
    }

    pub fn proxy() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("proxy")
            .and(warp::get())
            .and(warp::query::<model::ProxyParams>())
            .and_then(handlers::proxy)
    }

    async fn error_handler(rej: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
        Ok(if let Some(model::ReqwestError { error }) = rej.find() {
            warp::reply::with_status(error.into(), warp::http::StatusCode::BAD_GATEWAY)
        } else if let Some(model::InternalServerError { error }) = rej.find() {
            warp::reply::with_status(error.into(), warp::http::StatusCode::INTERNAL_SERVER_ERROR)
        } else if let Some(model::BadRequest { error }) = rej.find() {
            warp::reply::with_status(error.into(), warp::http::StatusCode::BAD_REQUEST)
        } else if let Some(_) = rej.find::<warp::reject::UnsupportedMediaType>() {
            warp::reply::with_status(
                format!("{:?}", rej),
                warp::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
            )
        } else if let Some(err) = rej.find::<warp::filters::body::BodyDeserializeError>() {
            warp::reply::with_status(format!("{:?}", err), warp::http::StatusCode::BAD_REQUEST)
        } else if rej.is_not_found() {
            warp::reply::with_status(format!("{:?}", rej), warp::http::StatusCode::NOT_FOUND)
        } else {
            warp::reply::with_status(
                format!("{:?}", rej),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        })
    }
}

mod handlers {
    use crate::model;
    use std::convert::Infallible;

    pub async fn item_list(shelf: model::AppStateRef) -> Result<impl warp::Reply, Infallible> {
        let shelf = &shelf.lock().await.shelf;
        let items: Vec<shelf::item::Item> = shelf.query_items().map(|x| x.1.clone()).collect();
        Ok(warp::reply::json(&items))
    }

    pub async fn item_get(
        key: String,
        shelf: model::AppStateRef,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let shelf = &shelf.lock().await.shelf;
        let item = shelf
            .query_items()
            .filter(|x| &x.1.key == &key)
            .map(|x| x.1.clone())
            .next();
        if let Some(rec) = item {
            Ok(warp::reply::json(&rec))
        } else {
            Err(warp::reject::not_found())
        }
    }

    pub async fn item_post(
        key: String,
        item: shelf::item::Item,
        shelf: model::AppStateRef,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        use shelf::shelf::ShelfError;

        if key != item.key {
            // TODO: for unicode, key needs to be decoded
            return Err(warp::reject::custom(model::BadRequest {
                error: format!(
                    "Item has key '{}' but this route is for key '{}'",
                    item.key, key
                ),
            }));
        }

        let mut state = shelf.lock().await;
        state.shelf.replace_item(item).map_err(|e| match e {
            ShelfError::InvalidReference(r) => warp::reject::custom(model::BadRequest {
                error: format!("Unrecognized reference to entity {}", r),
            }),
        })?;

        state.save()?;

        let response = model::CreateResponse {
            // status: model::CreateStatus::Updated,
            key: key,
        };
        Ok(warp::reply::with_status(
            warp::reply::json(&response),
            warp::http::StatusCode::ACCEPTED,
        ))
    }

    pub async fn person_list(shelf: model::AppStateRef) -> Result<impl warp::Reply, Infallible> {
        let shelf = &shelf.lock().await.shelf;
        let items: Vec<shelf::common::Person> = shelf.query_people().map(|p| p.clone()).collect();
        Ok(warp::reply::json(&items))
    }

    pub async fn person_create(
        person: shelf::common::Person,
        shelf: model::AppStateRef,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let mut state = shelf.lock().await;
        let created = state.shelf.insert_person(person.clone());
        let response = if created {
            model::CreateResponse {
                // status: model::CreateStatus::Created,
                key: person.key,
            };
        } else {
            model::CreateResponse {
                // status: model::CreateStatus::Updated,
                key: person.key,
            };
        };
        state.save()?;
        Ok(warp::reply::with_status(
            warp::reply::json(&response),
            if created {
                warp::http::StatusCode::CREATED
            } else {
                warp::http::StatusCode::ACCEPTED
            },
        ))
    }

    pub async fn series_list(shelf: model::AppStateRef) -> Result<impl warp::Reply, Infallible> {
        let state = shelf.lock().await;
        let items: Vec<shelf::series::Series> =
            state.shelf.query_series().map(|p| p.clone()).collect();
        Ok(warp::reply::json(&items))
    }

    pub async fn series_create(
        series: shelf::series::Series,
        shelf: model::AppStateRef,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let mut state = shelf.lock().await;
        let created = state.shelf.insert_series(series.clone());
        let response = if created {
            model::CreateResponse {
                // status: model::CreateStatus::Created,
                key: series.key,
            };
        } else {
            model::CreateResponse {
                // status: model::CreateStatus::Updated,
                key: series.key,
            };
        };
        state.save()?;
        Ok(warp::reply::with_status(
            warp::reply::json(&response),
            if created {
                warp::http::StatusCode::CREATED
            } else {
                warp::http::StatusCode::ACCEPTED
            },
        ))
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
