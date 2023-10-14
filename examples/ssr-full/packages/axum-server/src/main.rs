use std::io;

use axum::{
    error_handling::HandleError,
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
    Extension, Router, Server,
};
use silkenweb::{dom::Dry, router, task};
use ssr_example_app::app;
use tokio_util::task::LocalPoolHandle;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let local_pool = LocalPoolHandle::new(16);
    let pkg_service = HandleError::new(ServeDir::new("../axum-client/pkg"), io_error_to_response);
    let app = Router::new()
        .nest_service("/pkg", pkg_service)
        .fallback(handler)
        .layer(Extension(local_pool));
    Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn io_error_to_response(err: io::Error) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, err.to_string())
}

async fn handler(Extension(local_pool): Extension<LocalPoolHandle>, uri: Uri) -> impl IntoResponse {
    // Axum requires futures to be `Send` so they can be moved between threads.
    // Silkenweb is single threaded, so we spawn a task pinned to a thread using
    // `LocalPoolHandle`.
    local_pool
        .spawn_pinned(|| task::server::scope(render(uri)))
        .await
        .unwrap()
}

async fn render(uri: Uri) -> impl IntoResponse {
    let (title, body) = app::<Dry>();
    router::set_url_path(uri.path());
    task::render_now().await;

    let page_html = format!(
        include_str!("../../app/page.tmpl.html"),
        title_html = title.freeze(),
        body_html = body.freeze(),
        init_script = r#"
            import init, {js_main} from '/pkg/ssr_example_axum_client.js';
            init().then(js_main);
        "#
    );

    Response::builder()
        .status(StatusCode::OK)
        .body(page_html)
        .unwrap()
}
