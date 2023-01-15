use crate::cli::Cli;
use clap::Parser;
use hyper::server::conn::Http;
use tower::make::Shared;
use tower::ServiceBuilder;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::level_filters::LevelFilter;
use tracing::Level;

mod cli;
mod egress;
mod ingress;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    let cli = Cli::parse();

    let addr = "127.0.0.1:443".parse().unwrap();

    let ingress = ingress::TlsAcceptor::new(addr, &cli.cert_path);
    let egress = egress::EgressService::new();

    let trace = TraceLayer::new_for_http()
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let service = ServiceBuilder::new().layer(trace).service(egress);

    let make_service = Shared::new(service);

    let server = hyper::server::Builder::new(ingress, Http::new()).serve(make_service);

    tracing::info!(?addr, "Serving proxy");

    server.await.unwrap();
}
