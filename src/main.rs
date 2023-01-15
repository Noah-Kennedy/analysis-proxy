use crate::cli::Cli;
use clap::Parser;
use hyper::server::conn::Http;

mod cli;
mod egress;
mod ingress;
mod middleware;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt::fmt().pretty().init();

    let cli = Cli::parse();

    let addr = "127.0.0.1:443".parse().unwrap();

    let ingress = ingress::TlsAcceptor::new(addr, &cli.cert_path);
    let egress = egress::ProxyService::new();
    let middleware = middleware::MiddlewareMakeService::new(egress);

    let server = hyper::server::Builder::new(ingress, Http::new()).serve(middleware);

    tracing::info!(?addr, "Serving proxy");

    server.await.unwrap();
}
