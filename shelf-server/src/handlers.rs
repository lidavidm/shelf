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

use crate::model;
use std::collections::HashSet;
use std::convert::Infallible;

pub async fn item_list(shelf: model::AppStateRef) -> Result<impl warp::Reply, Infallible> {
    let shelf = &shelf.lock().await.shelf;
    let items: Vec<shelf::item::Item> = shelf.query_items().map(|x| x.1.clone()).collect();
    Ok(warp::reply::json(&items))
}

// The key for a blank template item.
static ITEM_TEMPLATE_KEY: &'static str = ":template:";

// Helper to percent-decode keys
pub fn decode_key(key: &str) -> Result<std::borrow::Cow<str>, warp::Rejection> {
    percent_encoding::percent_decode_str(&key)
        .decode_utf8()
        .map_err(|e| {
            warp::reject::custom(model::BadRequest {
                error: format!("Could not percent-decode item {:?}: {}", &key, e),
            })
        })
}

pub async fn item_get(
    key: String,
    shelf: model::AppStateRef,
) -> Result<impl warp::Reply, warp::Rejection> {
    if key == ITEM_TEMPLATE_KEY {
        log::info!(
            target: crate::LOG_NAME,
            "GET /item KEY: {} (item template)",
            key
        );
        return Ok(warp::reply::json::<shelf::item::Item>(&Default::default()));
    }

    let decoded_key = decode_key(&key)?;

    let shelf = &shelf.lock().await.shelf;
    let item = shelf
        .query_items()
        .filter(|x| &x.1.key == &decoded_key)
        .map(|x| x.1.clone())
        .next();
    if let Some(rec) = item {
        log::info!(
            target: crate::LOG_NAME,
            "GET /item KEY: {} (found)",
            decoded_key
        );
        Ok(warp::reply::json(&rec))
    } else {
        log::info!(
            target: crate::LOG_NAME,
            "GET /item KEY: {} (not found)",
            decoded_key
        );
        Err(warp::reject::not_found())
    }
}

pub async fn item_post(
    key: String,
    item: shelf::item::Item,
    shelf: model::AppStateRef,
) -> Result<impl warp::Reply, warp::Rejection> {
    use shelf::shelf::ShelfError;

    let decoded_key = decode_key(&key)?;
    log::info!(target: crate::LOG_NAME, "POST /item KEY: {}", item.key);

    if decoded_key != item.key {
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
        key: decoded_key.to_string(),
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
    let items: Vec<shelf::series::Series> = state.shelf.query_series().map(|p| p.clone()).collect();
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

pub async fn tag_list(shelf: model::AppStateRef) -> Result<impl warp::Reply, Infallible> {
    let shelf = &shelf.lock().await.shelf;
    let mut tags = HashSet::new();
    for item in shelf.query_items() {
        for tag in item.1.tags.iter() {
            tags.insert(tag);
        }
    }
    Ok(warp::reply::json(&tags))
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
