use std::sync::{
    Arc,
    Mutex,
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
    outgoing::NodeInfo,
};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Handle a HTTP or WebSocket request.
async fn handle_request(
    nodes: Arc<Mutex<Vec<NodeInfo>>>,
    websockets: SharedWebsockets,
    mut request: Request<Incoming>,
) -> Result<Response<Full<Bytes>>, Error> {
    // Check if the request is a websocket upgrade request.
    if hyper_tungstenite::is_upgrade_request(&request) {
        let (response, websocket) = hyper_tungstenite::upgrade(&mut request, None)?;

        // Spawn a task to handle the websocket connection.
        tokio::spawn(async move {
            if let Err(e) = serve_websocket(nodes, websockets, websocket).await {
                eprintln!("Error in websocket connection: {e}");
            }
        });

        // Return the response so the spawned future can continue.
        Ok(response)
    } else {
        // Handle regular HTTP requests here.
        Ok(Response::new(Full::<Bytes>::from("Hello HTTP!")))
    }
}

/// Handle a websocket connection.
async fn serve_websocket(
    nodes: Arc<Mutex<Vec<NodeInfo>>>,
    websockets: SharedWebsockets,
    websocket: HyperWebsocket,
) -> Result<(), Error> {
    let websocket = websocket.await?;
    let dom = Message::Text(
        serde_json::to_string(&Outgoing {
            notification: OutgoingNotification::Nodes(nodes.lock().unwrap().clone()),
        })?
        .into(),
    );
    let (mut write, mut read) = websocket.split();
    write.send(dom).await?;
    websockets.lock().await.push(write);
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
                println!("Disconnected");
            }
            _ => {}
        }
    }

    Ok(())
}

pub async fn run_server(
    nodes: Arc<Mutex<Vec<NodeInfo>>>,
    websockets: SharedWebsockets,
) -> Result<(), Error> {
    let addr: std::net::SocketAddr = "[::1]:3000".parse()?;
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("Listening on http://{addr}");

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
