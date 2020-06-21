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

use hyper::service::Service;
use hyper::{Request, Response};
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

struct Growler {
    router: Arc<crate::router::Router>,
}

struct MakeService {
    router: Arc<crate::router::Router>,
}

impl Service<Request<hyper::Body>> for Growler {
    type Response = Response<hyper::Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<hyper::Body>) -> Self::Future {
        let handler = self.router.route(req.method(), req.uri().path());
        let context = crate::handler::RequestContext { raw_request: req };
        let resp: Response<hyper::Body> = handler.handle(&context);
        let fut = async { Ok(resp) };
        Box::pin(fut)
    }
}

impl<T> Service<T> for MakeService {
    type Response = Growler;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let router = self.router.clone();
        let fut = async move { Ok(Growler { router }) };
        Box::pin(fut)
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    println!("Shutdown");
}

pub struct Server {
    router: Arc<crate::router::Router>,
}

impl Server {
    pub fn new(router: crate::router::Router) -> Server {
        Server {
            router: Arc::new(router),
        }
    }

    pub async fn run_forever(
        &mut self,
        address: &SocketAddr,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let server = hyper::Server::bind(address).serve(MakeService {
            router: self.router.clone(),
        });
        let graceful = server.with_graceful_shutdown(shutdown_signal());
        if let Err(e) = graceful.await {
            eprintln!("server error: {}", e);
            Err(Into::into(e))
        } else {
            eprintln!("Exiting");
            Ok(())
        }
    }
}
