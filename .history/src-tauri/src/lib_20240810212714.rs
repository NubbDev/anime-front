use tauri::Manager;
use tauri::{WebviewUrl, WebviewWindowBuilder};

const API_URL: &str = "https://anime_backend.giloetgg.workers.dev/";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_trending(page: &str) -> Result<serde_json::Value, ()> {
    let json: serde_json::Value = match page.is_empty() {
        true => {
            let data = ureq::get(format!("{}/anime/frontpage/trending", API_URL).as_str())
                .call()
                .expect("Failed to get data from API");
            let json: serde_json::Value = data.into_json().expect("Failed to parse JSON");
            json
        }
        false => {
            if (page.parse::<i32>().is_err()) {
                return Err(());
            }
            let data =
                ureq::get(format!("{}/anime/trending/{}", API_URL, page.parse::<i32>()).as_str())
                    .call()
                    .expect("Failed to get data from API");
            let json: serde_json::Value = data.into_json().expect("Failed to parse JSON");
            json
        }
    };
    Ok(json)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default());
            // let win_builder = win_builder.title("app").inner_size(800.0, 800.0);

            // win_builder.build().unwrap();

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
