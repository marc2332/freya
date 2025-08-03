use std::{
    collections::HashMap,
    sync::{
        Arc,
        Mutex,
        atomic::{
            AtomicU32,
            Ordering,
        },
    },
};

use freya_core::{
    event_loop_messages::{
        EventLoopMessage,
        EventLoopMessageAction,
    },
    plugins::PluginHandle,
};
use freya_native_core::NodeId;
use futures::{
    sink::SinkExt,
    stream::StreamExt,
};
use http_body_util::Full;
use hyper::{
    Request,
    Response,
    body::{
        Bytes,
        Incoming,
    },
};
use hyper_tungstenite::{
    HyperWebsocket,
    tungstenite,
};
use hyper_util::rt::TokioIo;
use tungstenite::Message;

use crate::{
    IncomingMessage,
    OutgoingMessage,
    OutgoingMessageAction,
    SharedWebsockets,
    incoming::IncomingMessageAction,
    node_info::NodeInfo,
};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

async fn handle_request(
    nodes: Arc<Mutex<HashMap<u64, Vec<NodeInfo>>>>,
    websockets: SharedWebsockets,
    mut request: Request<Incoming>,
    highlighted_node: Arc<Mutex<Option<NodeId>>>,
    plugin_handle: PluginHandle,
) -> Result<Response<Full<Bytes>>, Error> {
    if hyper_tungstenite::is_upgrade_request(&request) {
        let (response, websocket) = hyper_tungstenite::upgrade(&mut request, None)?;

        tokio::spawn(async move {
            let id = WEBSOCKET_ID.fetch_add(1, Ordering::Relaxed);
            if let Err(e) = serve_websocket(
                nodes,
                websockets.clone(),
                id,
                websocket,
                highlighted_node,
                plugin_handle,
            )
            .await
            {
                eprintln!("Disconnected, error in websocket connection: {e}");
                websockets.lock().await.remove(&id);
            }
        });

        Ok(response)
    } else {
        Ok(Response::new(Full::<Bytes>::from("Hello HTTP!")))
    }
}

static WEBSOCKET_ID: AtomicU32 = AtomicU32::new(0);

/// Handle a websocket connection.
async fn serve_websocket(
    nodes: Arc<Mutex<HashMap<u64, Vec<NodeInfo>>>>,
    websockets: SharedWebsockets,
    id: u32,
    websocket: HyperWebsocket,
    highlighted_node: Arc<Mutex<Option<NodeId>>>,
    plugin_handle: PluginHandle,
) -> Result<(), Error> {
    let websocket = websocket.await?;
    let (mut write, mut read) = websocket.split();
    let windows = nodes.lock().unwrap().clone();

    for (window_id, nodes) in windows {
        let message = Message::Text(
            serde_json::to_string(&OutgoingMessage {
                action: OutgoingMessageAction::Update { window_id, nodes },
            })?
            .into(),
        );

        // Send nodes snapshot
        write.send(message).await?;
    }

    // Store websocket
    websockets.lock().await.insert(id, write);

    while let Some(message) = read.next().await {
        match message? {
            Message::Text(msg) => {
                let incoming = serde_json::from_str::<IncomingMessage>(msg.as_str());

                if let Ok(incoming) = incoming {
                    match incoming.action {
                        IncomingMessageAction::HighlightNode { window_id, node_id } => {
                            highlighted_node.lock().unwrap().replace(node_id);
                            plugin_handle.send_event_loop_event(EventLoopMessage {
                                window_id: Some(window_id.into()),
                                action: EventLoopMessageAction::RequestRerender,
                            });
                        }
                    }
                } else {
                    println!("failed to parse");
                }
            }
            Message::Close(_) => {
                websockets.lock().await.remove(&id);
                println!("Disconnected");
            }
            _ => {}
        }
    }

    Ok(())
}

pub async fn run_server(
    nodes: Arc<Mutex<HashMap<u64, Vec<NodeInfo>>>>,
    websockets: SharedWebsockets,
    highlighted_node: Arc<Mutex<Option<NodeId>>>,
    plugin_handle: PluginHandle,
) -> Result<(), Error> {
    let addr: std::net::SocketAddr = "[::1]:7354".parse()?;
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("Running the Devtools Server on http://{addr}");

    let mut http = hyper::server::conn::http1::Builder::new();
    http.keep_alive(true);

    loop {
        let (stream, _) = listener.accept().await?;
        let nodes = nodes.clone();
        let websockets = websockets.clone();
        let highlighted_node = highlighted_node.clone();
        let plugin_handle = plugin_handle.clone();
        let connection = http
            .serve_connection(
                TokioIo::new(stream),
                hyper::service::service_fn(move |req| {
                    handle_request(
                        nodes.clone(),
                        websockets.clone(),
                        req,
                        highlighted_node.clone(),
                        plugin_handle.clone(),
                    )
                }),
            )
            .with_upgrades();
        tokio::spawn(async move {
            if let Err(err) = connection.await {
                println!("Error serving HTTP connection: {err:?}");
            }
        });
    }
}
