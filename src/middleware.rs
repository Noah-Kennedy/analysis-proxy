use crate::egress;
use hyper::client::ResponseFuture;
use hyper::header::HOST;
use hyper::{Body, Request, Response};
use std::task::{Context, Poll};
use tower::Service;

#[derive(Clone)]
pub struct MiddlewareService {
    egress: egress::ProxyService,
}

impl MiddlewareService {
    pub fn new(egress: egress::ProxyService) -> Self {
        Self { egress }
    }
}

impl Service<Request<Body>> for MiddlewareService {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = ResponseFuture;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.egress.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let uri = req.uri();
        let host = req.headers().get(HOST);

        tracing::info!(?host, ?uri, "Received request");

        self.egress.call(req)
    }
}
