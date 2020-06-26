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

use crate::handler::{self, Handler};

struct Route {
    method: hyper::Method,
    path: String,
    handler: Box<dyn Handler>,
}

pub struct Router {
    default_handler: Box<dyn Handler>,
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            default_handler: handler::simple(|ctx: crate::handler::RequestContext| async move {
                Ok(hyper::Response::builder()
                    .status(hyper::StatusCode::NOT_FOUND)
                    .body(hyper::Body::from(format!(
                        "404 NOT FOUND: {:?} {}\n",
                        ctx.raw_request.method(),
                        ctx.raw_request.uri().path()
                    )))?)
            }),
            routes: Vec::new(),
        }
    }

    pub fn add<H: Handler + 'static>(
        &mut self,
        method: hyper::Method,
        path: &str,
        handler: H,
    ) -> &mut Router {
        self.routes.push(Route {
            method: method.clone(),
            path: path.to_owned(),
            handler: Box::new(handler),
        });
        self
    }

    pub fn route(&self, method: &hyper::Method, path: &str) -> &Box<dyn Handler> {
        for route in self.routes.iter() {
            if method == route.method && path == route.path {
                return &route.handler;
            }
        }
        &self.default_handler
    }
}
