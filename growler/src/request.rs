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

pub async fn from_json<T>(body: hyper::Body) -> Result<T, crate::handler::Error>
where
    T: serde::de::DeserializeOwned,
{
    let bytes = hyper::body::to_bytes(body).await?.to_vec();
    Ok(serde_json::from_slice(&bytes)
        .map_err(|err| crate::handler::Error::Request(Box::new(err)))?)
}
