mod commands;
mod page;
mod websocket;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(websocket::WebsocketConnection::default())
        .manage(websocket::TlsConnector::default())
        .manage(page::CachedPages::default())
        .manage(page::CachedData::default())
        .manage(page::RouterState::default())
        .setup(|_| {
            // let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default());
            // let win_builder = win_builder.title("app").inner_size(800.0, 800.0);

            // win_builder.build().unwrap();
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            handle_connection,
            send_message,
            disconnect,
            store_page_cache,
            store_cache,
            get_page_cache,
            get_cache,
            init_history,
            push_history_state,
            pop_history_state,
            get_history_state,
            get_history_list,
            clear_history,
            get_history_previous_page,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
