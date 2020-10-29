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

use crate::{handlers, model};
use warp::Filter;

pub fn api(
    shelf: model::AppStateRef,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    item_list(shelf.clone())
        .or(item_get(shelf.clone()))
        .boxed()
        .or(item_post(shelf.clone()))
        .boxed()
        .or(person_list(shelf.clone()))
        .boxed()
        .or(person_create(shelf.clone()))
        .boxed()
        .or(series_list(shelf.clone()))
        .boxed()
        .or(series_create(shelf.clone()))
        .boxed()
        .or(tag_list(shelf.clone()))
        .boxed()
        .or(blob_list(shelf.clone()))
        .boxed()
        .or(blob_create(shelf.clone()))
        .boxed()
        .or(proxy())
        .boxed()
        .recover(error_handler)
}

fn with_shelf(shelf: model::AppStateRef) -> warp::filters::BoxedFilter<(model::AppStateRef,)> {
    warp::any().map(move || shelf.clone()).boxed()
}

pub fn item_list(shelf: model::AppStateRef) -> warp::filters::BoxedFilter<(warp::reply::Json,)> {
    warp::path!("item")
        .and(warp::get())
        .boxed()
        .and(with_shelf(shelf))
        .boxed()
        .and_then(handlers::item_list)
        .boxed()
}

pub fn item_get(
    shelf: model::AppStateRef,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("item" / String)
        .and(warp::get())
        .boxed()
        .and(with_shelf(shelf))
        .boxed()
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

pub fn tag_list(
    shelf: model::AppStateRef,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("tag")
        .and(warp::get())
        .and(with_shelf(shelf))
        .and_then(handlers::tag_list)
}

pub fn blob_list(
    shelf: model::AppStateRef,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("blob")
        .and(warp::get())
        .and(with_shelf(shelf))
        .and_then(handlers::blob_list)
}

pub fn blob_create(
    shelf: model::AppStateRef,
) -> warp::filters::BoxedFilter<(warp::reply::WithStatus<warp::reply::Json>,)> {
    warp::path!("blob")
        .and(warp::put())
        .and(with_shelf(shelf))
        .and(warp::filters::multipart::form())
        .and_then(handlers::blob_create)
        .boxed()
}

pub fn proxy() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("proxy")
        .and(warp::get())
        .and(warp::query::<model::ProxyParams>())
        .and_then(handlers::proxy)
}

async fn error_handler(rej: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!(target: crate::LOG_NAME, "Processing rejection: {:?}", rej);
    Ok(if let Some(model::ReqwestError { error }) = rej.find() {
        warp::reply::with_status(error.into(), warp::http::StatusCode::BAD_GATEWAY)
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
    } else if let Some(model::InternalServerError { error }) = rej.find() {
        warp::reply::with_status(error.into(), warp::http::StatusCode::INTERNAL_SERVER_ERROR)
    } else {
        warp::reply::with_status(
            format!("{:?}", rej),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        )
    })
}
