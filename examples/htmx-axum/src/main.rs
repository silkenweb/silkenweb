use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{
    headers::ContentType, response::IntoResponse, routing::get, Router, Server, TypedHeader,
};
use tracing::info;

async fn index() -> impl IntoResponse {
    (
        TypedHeader(ContentType::html()),
        include_str!("../index.html"),
    )
}

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let app = Router::new().route("/", get(index));

    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9090);
    let server = Server::bind(&address);
    info!("Server listening on http://{}", address);

    server.serve(app.into_make_service()).await.unwrap()
}
