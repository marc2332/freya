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
    Outgoing,
    OutgoingNotification,
    SharedWebsockets,
    incoming::IncomingMessage,
    node_info::NodeInfo,
};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

async fn handle_request(
    nodes: Arc<Mutex<HashMap<u64, Vec<NodeInfo>>>>,
    websockets: SharedWebsockets,
    mut request: Request<Incoming>,
) -> Result<Response<Full<Bytes>>, Error> {
    if hyper_tungstenite::is_upgrade_request(&request) {
        let (response, websocket) = hyper_tungstenite::upgrade(&mut request, None)?;

        tokio::spawn(async move {
            let id = WEBSOCKET_ID.fetch_add(1, Ordering::Relaxed);
            if let Err(e) = serve_websocket(nodes, websockets.clone(), id, websocket).await {
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
) -> Result<(), Error> {
    let websocket = websocket.await?;
    let (mut write, mut read) = websocket.split();
    let windows = nodes.lock().unwrap().clone();

    for (window_id, nodes) in windows {
        let message = Message::Text(
            serde_json::to_string(&Outgoing {
                notification: OutgoingNotification::Update { window_id, nodes },
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
                let incoming = serde_json::from_str(msg.as_str());

                if let Ok(incoming) = incoming {
                    match incoming {
                        IncomingMessage::HighlightNode(_node_id) => {
                            // TODO: Highlight the node
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
        let connection = http
            .serve_connection(
                TokioIo::new(stream),
                hyper::service::service_fn(move |req| {
                    handle_request(nodes.clone(), websockets.clone(), req)
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
