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
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;

pub async fn item_list(shelf: model::AppStateRef) -> Result<warp::reply::Json, Infallible> {
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
        ShelfError::InvalidKey(r) => warp::reject::custom(model::BadRequest {
            error: format!("Invalid key '{}'", r),
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

pub async fn blob_list(shelf: model::AppStateRef) -> Result<impl warp::Reply, Infallible> {
    let shelf = &shelf.lock().await.shelf;
    let mut blobs = HashMap::new();
    for blob in shelf.query_blobs() {
        blobs.insert(blob.key.clone(), blob.clone());
    }
    Ok(warp::reply::json(&blobs))
}

pub async fn blob_get_contents(
    key: String,
    shelf: model::AppStateRef,
    if_none_match: Option<String>,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    if if_none_match.is_some() {
        return Ok(Box::new(warp::reply::with_header(
            warp::http::StatusCode::NOT_MODIFIED,
            "ETag",
            "\"foo\"",
        )));
    }

    use std::io::Read;
    let state = &shelf.lock().await;
    for blob in state.shelf.query_blobs() {
        log::info!(target: crate::LOG_NAME, "Blob: {:?}", blob);
    }
    let blob = state
        .shelf
        .get_blob(&key)
        .ok_or_else(|| warp::reject::not_found())?;
    let path = state.saver.get_blob(&blob.key);

    let mut data = Vec::new();
    std::fs::File::open(path)
        .map_err(to_internal_err)?
        .read_to_end(&mut data)
        .map_err(to_internal_err)?;

    Ok(Box::new(warp::reply::with_header(
        warp::reply::with_header(
            warp::reply::with_header(data, "content-type", &blob.mime_type),
            "cache-control",
            "public, max-age=604800, immutable",
        ),
        "ETag",
        "\"foo\"",
    )))
}

pub async fn blob_create(
    shelf: model::AppStateRef,
    form: warp::multipart::FormData,
) -> Result<warp::reply::WithStatus<warp::reply::Json>, warp::Rejection> {
    use bytes::BufMut;
    use futures::{TryFutureExt, TryStreamExt};
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;
    let raw_part: Result<Vec<(String, Option<String>, Vec<u8>)>, warp::Rejection> = form
        .and_then(|part| {
            let name = part.name().to_string();
            let content_type = part.content_type().map(|t| t.to_string());
            part.stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .map_ok(move |vec| (name, content_type, vec))
        })
        .try_collect()
        .await
        .map_err(to_internal_err);
    let parts: Vec<(String, String, Vec<u8>)> = raw_part?
        .into_iter()
        .map(|(name, content_type, buf)| match content_type {
            Some(typ) => Ok((name, typ, buf)),
            None => Err(warp::reject::custom(model::BadRequest {
                error: format!("Blob {} is missing a content-type", name),
            })),
        })
        .collect::<Result<Vec<_>, warp::Rejection>>()?;

    let mut state = shelf.lock().await;
    let mut keys = Vec::new();
    for raw_blob in parts {
        keys.push(raw_blob.0.clone());
        let path = state.saver.insert_blob(&raw_blob.0).map_err(|err| {
            warp::reject::custom(model::BadRequest {
                error: format!("Could not save blob: {:?}", err),
            })
        })?;
        let blob = shelf::common::Blob::new_with_mime(raw_blob.0, raw_blob.1);
        state.shelf.insert_blob(blob.clone()).map_err(|err| {
            warp::reject::custom(model::BadRequest {
                error: format!("Could not insert blob: {}", err),
            })
        })?;
        let mut file = File::create(&path).await.map_err(to_internal_err)?;
        file.write_all(&raw_blob.2).await.map_err(to_internal_err)?;
        file.sync_all().await.map_err(to_internal_err)?;
        log::info!(
            target: crate::LOG_NAME,
            "blob_create: created {:?} of length {}",
            &blob,
            raw_blob.2.len()
        );
    }
    state.save()?;
    let response = model::MultiCreateResponse { keys };
    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        warp::http::StatusCode::ACCEPTED,
    ))
}

pub async fn proxy(
    client: reqwest::Client,
    params: model::ProxyParams,
) -> Result<impl warp::Reply, warp::Rejection> {
    use reqwest::Method;
    log::info!(target: crate::LOG_NAME, "GET /proxy URL: {}", params.url);
    let mut request = client.request(Method::GET, &params.url);
    if let Some(referrer) = params.referrer {
        request = request.header(reqwest::header::REFERER, referrer);
    }
    for (name, value) in params.cookies.iter() {
        let cookie = cookie::Cookie::new(name, value);
        request = request.header(reqwest::header::SET_COOKIE, format!("{}", cookie));
    }
    let response = request.send().await.map_err(|err| {
        warp::reject::custom(model::ReqwestError {
            error: format!("{}", err),
        })
    })?;
    let header = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .cloned()
        .unwrap_or_else(|| reqwest::header::HeaderValue::from_static("text/plain"));
    let content_type = header.to_str().unwrap_or("text/plain");
    Ok(warp::reply::with_header(
        response
            .bytes()
            .await
            .map(|bytes| bytes.as_ref().to_vec())
            .map_err(|err| {
                warp::reject::custom(model::ReqwestError {
                    error: format!("{}", err),
                })
            })?,
        "content-type",
        content_type,
    ))
}

fn to_internal_err<E: std::fmt::Display>(err: E) -> warp::Rejection {
    warp::reject::custom(model::InternalServerError {
        error: format!("{}", err),
    })
}
