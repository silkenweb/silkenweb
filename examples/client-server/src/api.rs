use arpy::{FnRemote, FnSubscription, MsgId};
use serde::{Deserialize, Serialize};
use silkenweb::cfg_browser;

#[derive(MsgId, Serialize, Deserialize, Debug)]
pub struct Update {
    pub delta: i32,
}

impl FnRemote for Update {
    type Output = ();
}

#[derive(MsgId, Serialize, Deserialize, Debug)]
pub struct GetValue;

impl FnSubscription for GetValue {
    type Item = i32;
}

#[cfg_browser(false)]
pub mod server {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use arpy_axum::RpcRoute;
    use arpy_server::WebSocketRouter;
    use axum::{Router, Server};
    use futures_signals::signal::{Mutable, SignalExt};
    use silkenweb::clone;

    use super::{GetValue, Update};

    pub async fn run() {
        let value = Mutable::new(0);

        let ws = WebSocketRouter::new()
            .handle_subscription({
                clone!(value);
                move |_: GetValue| value.signal().to_stream()
            })
            .handle(move |update: Update| {
                clone!(value);
                async move {
                    value.replace_with(|v| *v + update.delta);
                }
            });
        let app = Router::new().ws_rpc_route("/api", ws);

        Server::bind(&SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9090))
            .serve(app.into_make_service())
            .await
            .unwrap()
    }
}
