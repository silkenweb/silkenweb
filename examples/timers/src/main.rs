use std::time::Duration;

use futures::stream::StreamExt;
use futures_signals::signal::{Mutable, SignalExt};
use silkenweb::{cfg_browser, elements::html::*, prelude::*, task::spawn_local, time, value::Sig};

fn app() -> Div {
    let ticks = Mutable::new(0);
    // Tick every tenth of a second.
    let mut interval = time::interval(Duration::from_millis(100));
    // Throttle ticks so they only happen every second.
    let ticks_signal = ticks
        .signal()
        .throttle(|| time::sleep(Duration::from_secs(1)));

    spawn_local(async move {
        while interval.next().await.is_some() {
            ticks.set(ticks.get() + 1);
        }
    });

    div().child(p().text(Sig(ticks_signal.map(|i| format!("{i}")))))
}

#[cfg_browser(true)]
fn main() {
    mount("app", app());
}

#[cfg_browser(false)]
#[tokio::main]
async fn main() {
    use silkenweb::task;

    task::scope(async {
        let app = app().freeze();

        for _i in 0..30 {
            task::render_now().await;
            println!("{app}");
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
}
