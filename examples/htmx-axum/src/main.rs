use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{
    headers::ContentType, response::IntoResponse, routing::{get, post}, Router, Server, TypedHeader,
};
use silkenweb::prelude::{html::div, ParentElement};
use silkenweb_htmx_axum::HtmxResponse;
use tracing::info;

async fn index() -> impl IntoResponse {
    (
        TypedHeader(ContentType::html()),
        include_str!("../index.html"),
    )
}

// TODO: Read some data from the post request
async fn button_clicked() -> impl IntoResponse {
    info!("Button clicked");
    HtmxResponse::new(div().text("Button clicked"))
}

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let app = Router::new()
        .route("/", get(index))
        .route("/button-clicked", post(button_clicked));

    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9090);
    let server = Server::bind(&address);
    info!("Server listening on http://{}", address);

    server.serve(app.into_make_service()).await.unwrap()
}
