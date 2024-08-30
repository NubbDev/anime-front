use std::{collections::HashMap, sync::Mutex};

use chrono::DateTime;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};
use tauri::{ipc::Channel, State};

#[derive(Debug, Serialize_repr, Deserialize_repr, FromPrimitive, Eq, Hash, PartialEq)]
#[repr(u32)]
pub enum CachedType {
    Home,
    Trending,
    Popular,
    Top,
    Seasonal,
    Search,
}
#[derive(
    Debug, Serialize_repr, Deserialize_repr, FromPrimitive, Eq, Hash, PartialEq, Clone, Copy,
)]
#[repr(u32)]
pub enum Page {
    Home,
    About,
    Releases,
    Downloads,
    Profile,
    Settings,
    Player,
    Search,
    Auth,
    NotFound,
    Trending,
    Popular,
    Top,
    Seasonal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cached {
    pub data: Value,
    #[serde(rename = "lastUpdated")]
    pub last_updated: DateTime<chrono::Local>,
}

pub struct Router {
    pub history: Vec<Page>,
    pub current_page: Page,
    pub on_message: Option<Channel<Page>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PageData {
    pub page: Page,
    pub data: Value,
}

impl Default for Router {
    fn default() -> Self {
        Self {
            history: vec![Page::Home],
            current_page: Page::Home,
            on_message: None,
        }
    }
}

#[derive(Default)]
pub struct CachedPages(pub Mutex<HashMap<Page, Cached>>);

#[derive(Default)]
pub struct CachedData(pub Mutex<HashMap<String, Cached>>);

#[derive(Default)]
pub struct RouterState(pub Mutex<Router>);

pub fn create_router(manager: State<'_, RouterState>, on_message: Channel<Page>) {
    let mut router = manager.0.lock().unwrap();
    router.history.clear();
    router.history.push(Page::Home);
    router.current_page = Page::Home;
    router.on_message = Some(on_message);
}
