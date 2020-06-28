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

pub fn json<T>(value: &T) -> crate::handler::Result
where
    T: serde::Serialize,
{
    serde_json::ser::to_vec(value)
        .map_err(|err| crate::handler::Error::Other(Box::new(err)))
        .and_then(|body| {
            hyper::Response::builder()
                .status(hyper::StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(hyper::Body::from(body))
                .map_err(|err| crate::handler::Error::Http(err))
        })
}
