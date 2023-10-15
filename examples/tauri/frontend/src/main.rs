use futures_signals::signal::Mutable;
use silkenweb::{
    clone,
    elements::html::{button, div},
    mount,
    prelude::{ElementEvents, ParentElement},
    task::spawn_local,
    value::Sig,
};
use web_sys::window;

fn main() {
    let response = Mutable::new(String::new());

    mount(
        "app",
        div()
            .child(
                div().child(
                    button()
                        .text("Feed Bandit the cat with Dreamies")
                        .on_click({
                            clone!(response);
                            move |_, _| feed_bandit("Dreamies", response.clone())
                        }),
                ),
            )
            .child(
                div().child(
                    button()
                        .text("Feed Bandit the cat with Sardines")
                        .on_click({
                            clone!(response);
                            move |_, _| {
                                clone!(response);
                                spawn_local(async move {
                                    response.set(feed_bandit_with_sardines().await)
                                })
                            }
                        }),
                ),
            )
            .child(
                div().child(button().text("Feed Bandit the cat with Sprouts").on_click({
                    clone!(response);
                    move |_, _| feed_bandit("Sprouts", response.clone())
                })),
            )
            .text(Sig(response.signal_cloned())),
    );
}

fn feed_bandit(food: &str, response: Mutable<String>) {
    let food = food.to_string();
    clone!(response);

    spawn_local(async move {
        match feed_bandit_the_cat(&food).await {
            Ok(message) => {
                response.set(message);
            }
            Err(e) => {
                let window = window().unwrap();
                window.alert_with_message(&e).unwrap();
            }
        }
    });
}

#[silkenweb_tauri::client_command(fallible)]
async fn feed_bandit_the_cat(food: &str) -> Result<String, String>;

#[silkenweb_tauri::client_command(infallible)]
async fn feed_bandit_with_sardines() -> String;
