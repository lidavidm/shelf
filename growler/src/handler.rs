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

use futures::FutureExt;
use std::future::Future;
use std::pin::Pin;

pub struct RequestContext {
    pub raw_request: hyper::Request<hyper::Body>,
}

type Response = hyper::Response<hyper::Body>;
type Error = hyper::Error;
type ResponseFuture = dyn Future<Output = Result<Response, Error>> + Send;

pub trait Handler: Sync + Send {
    fn handle(&self, context: RequestContext) -> Pin<Box<ResponseFuture>>;
}

impl<H> Handler for Box<H>
where
    H: Handler + ?Sized,
{
    fn handle(&self, context: RequestContext) -> Pin<Box<ResponseFuture>> {
        (**self).handle(context)
    }
}

pub struct SimpleHandler<F>
where
    F: Future<Output = Response> + Send + 'static,
{
    f: Box<dyn Fn(RequestContext) -> F + Send + Sync>,
}

impl<F> Handler for SimpleHandler<F>
where
    F: Future<Output = Response> + Send + 'static,
{
    fn handle(&self, context: RequestContext) -> Pin<Box<ResponseFuture>> {
        Box::pin((self.f)(context).map(|x| Ok(x)))
    }
}

pub fn simple<F, U>(f: F) -> Box<dyn Handler>
where
    F: Fn(RequestContext) -> U + Send + Sync + 'static,
    U: Future<Output = Response> + Send + 'static,
{
    Box::new(SimpleHandler { f: Box::new(f) })
}

pub fn with_state<F, R, S>(f: F, state: S) -> impl Fn(RequestContext) -> R + Clone
where
    F: Fn(RequestContext, S) -> R + Clone + Send + Sync + 'static,
    S: Clone + Send + Sync + 'static,
{
    move |ctx| f(ctx, state.clone())
}

pub fn with_error<F, U, H, E>(
    f: F,
    error_handler: H,
) -> impl Fn(RequestContext) -> Pin<Box<dyn Future<Output = Response> + Send>> + Send + Sync + 'static
where
    F: Fn(RequestContext) -> U + Clone + Send + Sync + 'static,
    U: Future<Output = Result<Response, E>> + Send + 'static,
    H: Fn(E) -> Response + Clone + Send + Sync + 'static,
{
    move |ctx| {
        let f = f.clone();
        let h = error_handler.clone();
        Box::pin(async move { f(ctx).await.map_or_else(|e| h(e), |x| x) })
    }
}
