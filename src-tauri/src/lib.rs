use tauri::Manager;
use tauri::{WebviewUrl, WebviewWindowBuilder};

const API_URL: &str = "https://anime_backend.giloetgg.workers.dev";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn get_frontpage() -> Result<serde_json::Value, ()> {
    let data = ureq::get(format!("{}/anime/frontpage", API_URL).as_str())
        .call()
        .expect("Failed to get data from API");
    let json: serde_json::Value = data.into_json().expect("Failed to parse JSON");
    Ok(json)
}

#[tauri::command]
async fn get_page(
    path: &str,
    page: i8,
    genres: Vec<&str>,
    blacklisted_genres: Vec<&str>,
) -> Result<serde_json::Value, ()> {
    let data = ureq::get(format!("{}/anime/pages/{}", API_URL, path).as_str())
        .query("page", page.to_string().as_str())
        .query("genres", genres.join(",").as_str())
        .query("blacklisted_genres", blacklisted_genres.join(",").as_str())
        .call()
        .expect("Failed to get data from API");
    let json: serde_json::Value = data.into_json().expect("Failed to parse JSON");
    // println!("{:?}", json);
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
        .invoke_handler(tauri::generate_handler![get_frontpage, get_page])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
