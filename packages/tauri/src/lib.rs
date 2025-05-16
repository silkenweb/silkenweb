pub use js_sys;
pub use serde_wasm_bindgen;
/// Add a Tauri command signature to the client.
///
/// See the tauri docs for an explanation of [commands](https://tauri.studio/docs/guides/command/).
///
/// ⚠️ **It's the clients responsibility to ensure client and server command
/// signatures match.** ⚠️
///
/// Commands can fail by returning a [`Result`], or be infallible by returning
/// either a plain value or `()`. To specify an infallible command, use
/// `#[silkenweb_tauri::client_command(infallible)]`. To specify a fallible
/// command that returns a [`Result`], use
/// `#[silkenweb_tauri::client_command(fallible)]`.
///
/// Commands can specify a visibility with `pub` or `pub(crate)` etc. Commands
/// must be `async`. All argument types must be `serde::Serialize`, and all
/// return types must be `serde::Deserialize`.
///
/// # Examples
///
/// An infallible command with arguments:
///
/// ```
/// #[silkenweb_tauri::client_command(infallible)]
/// async fn never_fails(arg1: &str, arg2: u64) -> String;
/// ```
///
/// A fallible command:
///
/// ```
/// #[silkenweb_tauri::client_command(fallible)]
/// async fn might_fail() -> Result<String, String>;
/// ```
///
/// A publicly visible command:
///
/// ```
/// #[silkenweb_tauri::client_command(infallible)]
/// pub async fn now_you_see_me() -> String;
/// ```
///
/// [`Result`]: std::result::Result
pub use silkenweb_tauri_proc_macro::client_command;
pub use static_assertions;
pub use wasm_bindgen;
pub use wasm_bindgen_futures;
