use hyper::client::{HttpConnector, ResponseFuture};
use hyper::service::Service;
use hyper::{Body, Client, Request, Response};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use std::sync::Arc;
use std::task::{Context, Poll};

#[derive(Clone)]
pub struct ProxyService {
    client: Arc<Client<HttpsConnector<HttpConnector>>>,
}

impl ProxyService {
    pub fn new() -> Self {
        let builder = HttpsConnectorBuilder::new();

        let connector = builder
            .with_webpki_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build();

        let client = Arc::new(Client::builder().set_host(false).build(connector));

        Self { client }
    }
}

impl Service<Request<Body>> for ProxyService {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = ResponseFuture;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        self.client.request(req)
    }
}
