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
    regex: regex::Regex,
    handler: Box<dyn Handler>,
}

pub struct Router {
    default_handler: Box<dyn Handler>,
    routes: Vec<Route>,
}

pub struct Builder {
    default_handler: Box<dyn Handler>,
    routes: Vec<Route>,
}

impl Router {
    pub fn route(&self, method: &hyper::Method, path: &str) -> (&dyn Handler, Vec<String>) {
        for route in self.routes.iter() {
            if method == route.method {
                if let Some(captures) = route.regex.captures(path) {
                    let parts = captures
                        .iter()
                        .skip(1)
                        .flatten()
                        .map(|m| m.as_str().to_owned())
                        .collect();
                    return (&*route.handler, parts);
                }
            }
        }
        (&*self.default_handler, Vec::new())
    }
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
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
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: error enum; validate the path
        let mut path_regex = String::new();
        path_regex += "^";
        path_regex += &path.replace("?", "([^/]*)");
        path_regex += "$";
        self.routes.push(Route {
            method: method.clone(),
            path: path.to_owned(),
            regex: regex::Regex::new(&path_regex)?,
            handler: Box::new(handler),
        });
        Ok(())
    }

    pub fn build(self) -> Router {
        Router {
            default_handler: self.default_handler,
            routes: self.routes,
        }
    }
}
