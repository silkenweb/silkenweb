#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            feed_bandit_the_cat,
            feed_bandit_with_sardines
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn feed_bandit_the_cat(food: &str) -> Result<String, String> {
    match food {
        "Dreamies" => Ok("Prrrrr".to_string()),
        _ => Err("No thanks, human".to_string()),
    }
}

#[tauri::command]
fn feed_bandit_with_sardines() -> String {
    "More...".to_string()
}
