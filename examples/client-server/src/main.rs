use silkenweb::cfg_browser;

#[cfg_browser(true)]
fn main() {
    console_error_panic_hook::set_once();
    silkenweb_example_client_server::client::counter();
}

#[cfg_browser(false)]
#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
    silkenweb_example_client_server::api::server::run().await;
}
