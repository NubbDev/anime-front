use num::FromPrimitive;
use serde_json::Value;
use tauri::{async_runtime::spawn, ipc::Channel, Manager, Runtime, State};

use crate::page;
use crate::page::Page;
use crate::websocket;

const API_URL: &str = "anime_backend.giloetgg.workers.dev";
const TEST_API_URL: &str = "192.168.36.1:8787";

#[tauri::command]
pub fn handle_connection<R: Runtime>(
    app: tauri::AppHandle<R>,
    on_message: Channel<serde_json::Value>,
) {
    spawn(websocket::handle_connection(
        app.app_handle().clone(),
        TEST_API_URL.to_string(),
        on_message,
    ));
}

#[tauri::command]
pub async fn send_message(
    manager: State<'_, websocket::WebsocketConnection>,
    message_type: u32,
    data: Option<Value>,
) -> Result<(), ()> {
    websocket::send_socket_message(manager, message_type, data)
        .await
        .unwrap();
    Ok(())
}

#[tauri::command]
pub async fn disconnect(manager: State<'_, websocket::WebsocketConnection>) -> Result<(), ()> {
    websocket::disconnect_socket(manager).await.unwrap();
    Ok(())
}

#[tauri::command]
pub fn store_page_cache(
    manager: State<'_, page::CachedPages>,
    key: u32,
    data: Value,
) -> Result<(), ()> {
    let store_type = page::Page::from_u32(key).unwrap();
    let mut cache = manager.0.lock().unwrap();
    let storing_value = page::Cached {
        data,
        last_updated: chrono::Local::now(),
    };
    cache.insert(store_type, storing_value);
    Ok(())
}

#[tauri::command]
pub fn get_page_cache(
    manager: State<'_, page::CachedPages>,
    key: u32,
) -> Result<Option<Value>, ()> {
    let store_type = page::Page::from_u32(key).unwrap();
    let cache = manager.0.lock().unwrap();
    match cache.get(&store_type) {
        Some(data) => Ok(Some(serde_json::to_value(data).unwrap())),
        None => Ok(None),
    }
}

#[tauri::command]
pub fn get_cache(
    manager: tauri::State<'_, page::CachedData>,
    key: String,
) -> Result<Option<Value>, ()> {
    let cache = manager.0.lock().unwrap();
    match cache.get(&key) {
        Some(data) => Ok(Some(serde_json::to_value(data).unwrap())),
        None => Ok(None),
    }
}

#[tauri::command]
pub fn store_cache(
    manager: tauri::State<'_, page::CachedData>,
    key: String,
    data: Value,
) -> Result<(), ()> {
    let mut cache = manager.0.lock().unwrap();
    let storing_value = page::Cached {
        data,
        last_updated: chrono::Local::now(),
    };
    cache.insert(key, storing_value);
    Ok(())
}

#[tauri::command]
pub fn init_history(manager: State<'_, page::RouterState>, on_message: Channel<page::Page>) {
    page::create_router(manager, on_message);
}
#[tauri::command]
pub fn push_history_state(
    manager: State<'_, page::RouterState>,
    page: u32,
) -> Result<page::Page, ()> {
    let mut router = manager.0.lock().unwrap();
    let last_page = router.current_page;
    let page = page::Page::from_u32(page).unwrap();

    if (router.history.is_empty() && router.history.last().unwrap() == &page)
        || page == page::Page::NotFound
    {
        return Ok(last_page);
    }

    router.history.push(page);
    router.current_page = page;
    if let Some(ref mut on_message) = router.on_message {
        on_message
            .send(page)
            .expect("failed to send page data to the webview");
    }

    Ok(last_page)
}

#[tauri::command]
pub fn pop_history_state(manager: State<'_, page::RouterState>) -> Result<Page, ()> {
    let mut router = manager.0.lock().unwrap();
    let last_page = router.current_page;
    if router.history.len() > 1 {
        router.history.pop();
        let page = router.history.last().unwrap().to_owned();
        router.current_page = page;
        if let Some(ref mut on_message) = router.on_message {
            on_message
                .send(page)
                .expect("failed to send page data to the webview");
        }
    }
    Ok(last_page)
}

#[tauri::command]
pub fn get_history_state(manager: State<'_, page::RouterState>) -> Result<page::Page, ()> {
    let router = manager.0.lock().unwrap();
    Ok(router.current_page)
}

#[tauri::command]
pub fn get_history_list(manager: State<'_, page::RouterState>) -> Result<Vec<page::Page>, ()> {
    let router = manager.0.lock().unwrap();
    Ok(router.history.clone())
}

#[tauri::command]
pub fn clear_history(manager: State<'_, page::RouterState>) -> Result<(), ()> {
    let mut router = manager.0.lock().unwrap();
    router.history.clear();
    router.history.push(page::Page::Home);
    router.current_page = page::Page::Home;
    if let Some(ref mut on_message) = router.on_message {
        on_message
            .send(page::Page::Home)
            .expect("failed to send page data to the webview");
    }
    Ok(())
}

#[tauri::command]
pub fn get_history_previous_page(manager: State<'_, page::RouterState>) -> Result<page::Page, ()> {
    let router = manager.0.lock().unwrap();
    if router.history.len() > 1 {
        Ok(router.history[router.history.len() - 2])
    } else {
        Ok(page::Page::Home)
    }
}
