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

use std::collections::HashMap;
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
    pub referrer: Option<String>,
    pub cookies: Option<HashMap<String, String>>,
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

#[derive(Debug, serde_derive::Serialize)]
pub struct MultiCreateResponse {
    pub keys: Vec<String>,
}
