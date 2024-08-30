use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use num::FromPrimitive;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};
use tauri::{ipc::Channel, AppHandle, Manager, Runtime, State};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{
    connect_async_tls_with_config,
    tungstenite::{protocol::WebSocketConfig, Message},
    Connector, MaybeTlsStream, WebSocketStream,
};

pub type SocketReader<S> = SplitStream<WebSocketStream<S>>;
pub type SocketWriter<S> = SplitSink<WebSocketStream<S>, Message>;

#[derive(Default)]
pub struct TlsConnector(pub Mutex<Option<Connector>>);

#[derive(Default)]
pub struct WebsocketConnection(pub Mutex<Option<SocketWriter<MaybeTlsStream<TcpStream>>>>);

#[derive(Serialize_repr, Deserialize_repr, FromPrimitive, Debug)]
#[repr(u32)]
pub enum ClientWSMessageType {
    Search,
    CommonPage,
    HomePage,
    SearchPage,
    SeasonAnimes,
    GetAnime,
    GetEpisode,
}

#[derive(Serialize_repr, Deserialize_repr, FromPrimitive, Debug)]
#[repr(u32)]
pub enum ServerWSMessageType {
    SearchResult,
    CommonPageData,
    HomePageRoute,
    SearchPageContent,
    SeasonAnimesList,
    AnimeData,
    EpisodeLink,
    GogoAnimeData,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum ResponseType {
    Status(String),
    Server(ServerWSMessageType),
    Client(ClientWSMessageType),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum ResponseData {
    String(String),
    Value(Value),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RustToJs {
    #[serde(rename = "type")]
    pub message_type: ResponseType,
    pub data: Option<ResponseData>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WebsocketResponse {
    #[serde(rename = "type")]
    pub message_type: ServerWSMessageType,
    pub data: Value,
}

pub async fn handle_connection<R: Runtime>(
    app: AppHandle<R>,
    url: String,
    on_message: Channel<serde_json::Value>,
) {
    let config = WebSocketConfig {
        ..Default::default()
    };
    let mut reader_socket: Option<SocketReader<MaybeTlsStream<TcpStream>>> = None;

    loop {
        match reader_socket.as_mut() {
            Some(reader) => {
                let connected_struct = RustToJs {
                    message_type: ResponseType::Status("connected".to_string()),
                    data: None,
                };

                on_message
                    .send(serde_json::to_value(connected_struct).unwrap())
                    .expect("Failed to send message");
                while let Some(msg) = reader.next().await {
                    match msg {
                        Ok(msg) => match msg {
                            Message::Text(text) => {
                                if let Ok(json) =
                                    serde_json::from_str::<WebsocketResponse>(text.as_str())
                                {
                                    let response = WebsocketResponse {
                                        message_type: json.message_type,
                                        data: json.data,
                                    };
                                    on_message
                                        .send(serde_json::to_value(response).unwrap())
                                        .expect("Failed to send message");
                                }
                            }
                            Message::Close(_) => {
                                break;
                            }
                            _ => {}
                        },
                        Err(_) => {
                            continue;
                        }
                    }
                }
                let connected_struct = RustToJs {
                    message_type: ResponseType::Status("disconnected".to_string()),
                    data: None,
                };
                on_message
                    .send(serde_json::to_value(connected_struct).unwrap())
                    .expect("Failed to send message");
                reader_socket = None;
                let socket_state: State<'_, WebsocketConnection> =
                    app.state::<WebsocketConnection>();
                let mut socket_state = socket_state.0.lock().await;
                *socket_state = None;
            }
            None => {
                let request = format!("ws://{}/ws", url);

                let tls_connector = match app.try_state::<TlsConnector>() {
                    Some(connector) => connector.0.lock().await.clone(),
                    None => None,
                };

                if let Ok((ws_stream, _)) =
                    connect_async_tls_with_config(request, Some(config), false, tls_connector).await
                {
                    let (writer, reader) = ws_stream.split();
                    reader_socket = Some(reader);
                    let socket_state: State<'_, WebsocketConnection> =
                        app.state::<WebsocketConnection>();
                    let mut socket_state = socket_state.0.lock().await;
                    *socket_state = Some(writer);
                }
            }
        }
    }
}

pub async fn send_socket_message(
    manager: State<'_, WebsocketConnection>,
    message_type: u32,
    data: Option<Value>,
) -> Result<(), ()> {
    if let Some(socket) = manager.0.lock().await.as_mut() {
        let message_type =
            ClientWSMessageType::from_u32(message_type).expect("Invalid message type");
        let json_data = serde_json::to_string(&RustToJs {
            message_type: ResponseType::Client(message_type),
            data: data.map(ResponseData::Value),
        })
        .unwrap();
        socket
            .send(tokio_tungstenite::tungstenite::Message::Text(json_data))
            .await
            .expect("Failed to send message");
    } else {
        eprintln!("No websocket connection");
    }

    Ok(())
}

pub async fn disconnect_socket(manager: State<'_, WebsocketConnection>) -> Result<(), ()> {
    if let Some(socket) = manager.0.lock().await.as_mut() {
        socket.close().await.expect("Failed to close socket");
    } else {
        eprintln!("No websocket connection");
    }

    Ok(())
}
// fn send(on_message: &Channel<serde_json::Value>, data: WebsocketResponse, )
