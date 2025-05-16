use std::fmt::{self, Display, Formatter};

use arpy::{ConcurrentRpcClient, FnRemote};
use arpy_reqwasm::websocket;
use futures::{stream, StreamExt};
use futures_signals::signal::{Mutable, SignalExt};
use reqwasm::websocket::futures::WebSocket;
use silkenweb::{
    elements::{
        html::{button, div, p},
        ElementEvents,
    },
    mount,
    node::element::{ParentElement, TextParentElement},
    task::spawn_local,
    value::Sig,
};

use crate::api::{self, Update};

pub fn counter() {
    let conn = websocket::Connection::new(WebSocket::open("ws://localhost:9090/api").unwrap());
    let count = Mutable::new(Count::Connecting);
    spawn_local(get_value(conn.clone(), count.clone()));

    let count_text = count.signal().map(|c| c.to_string());
    let inc = move |_, _| {
        spawn_local(update(conn.clone(), 1));
    };

    let app = div()
        .child(button().on_click(inc).text("+"))
        .child(p().text(Sig(count_text)));

    mount("app", app);
}

#[derive(Copy, Clone)]
enum Count {
    Connecting,
    NotConnected,
    Value(i32),
}

impl Display for Count {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Count::Connecting => f.write_str("Connecting..."),
            Count::NotConnected => f.write_str("Not connected."),
            Count::Value(i) => write!(f, "{i}"),
        }
    }
}

async fn update(conn: websocket::Connection, delta: i32) {
    Update { delta }.call(&conn).await.unwrap()
}

async fn get_value(conn: websocket::Connection, count: Mutable<Count>) {
    let ((), mut values) = conn
        .subscribe(api::GetValue, stream::pending())
        .await
        .unwrap();

    while let Some(v) = values.next().await {
        match v {
            Ok(v) => count.set(Count::Value(v)),
            Err(_) => {
                count.set(Count::NotConnected);
                break;
            }
        }
    }
}
