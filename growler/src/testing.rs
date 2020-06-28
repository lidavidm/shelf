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

use crate::router;

pub fn run(
    router: &router::Router,
    method: hyper::Method,
    path: &str,
) -> Result<hyper::Response<hyper::Body>, Box<dyn std::error::Error + Send + Sync>> {
    let request = hyper::Request::builder()
        .uri(format!("http://growler.rs{}", path))
        .method(method)
        .body(hyper::Body::empty())?;
    req(router, request)
}

pub fn req(
    router: &router::Router,
    req: hyper::Request<hyper::Body>,
) -> Result<hyper::Response<hyper::Body>, Box<dyn std::error::Error + Send + Sync>> {
    // TODO: share this code with server.rs
    let (handler, parts) = router.route(req.method(), req.uri().path());
    let context = crate::handler::RequestContext {
        raw_request: req,
        route_parts: parts,
    };
    Ok(futures::executor::block_on(handler.handle(context))?)
}

pub fn into_bytes(body: hyper::Body) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    let bytes = futures::executor::block_on(hyper::body::to_bytes(body))?;
    Ok(bytes.to_vec())
}

pub fn into_json<T>(body: hyper::Body) -> Result<T, Box<dyn std::error::Error + Send + Sync>>
where
    T: serde::de::DeserializeOwned,
{
    let bytes = into_bytes(body)?;
    Ok(serde_json::from_slice(&bytes)?)
}
