#[cfg(feature = "devtools")]
use std::io::{Read, Write};
#[cfg(feature = "devtools")]
use std::net::TcpStream;
use std::path::PathBuf;
#[cfg(feature = "devtools")]
use std::sync::{Mutex, OnceLock, mpsc};

pub mod config;
pub use config::*;

#[cfg(feature = "serve")]
use futures_util::{
    StreamExt,
    future::{Either, select},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use subsecond;
pub use subsecond_types::JumpTable;

/// A message the hot reloading server sends to the client
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DevserverMsg {
    /// Attempt a hotreload
    /// This includes all the templates/literals/assets/binary patches that have changed in one shot
    HotReload(HotReloadMsg),

    /// Starting a hotpatch
    HotPatchStart,

    /// The devserver is starting a full rebuild.
    FullReloadStart,

    /// The full reload failed.
    FullReloadFailed,

    /// The app should reload completely if it can
    FullReloadCommand,

    /// The program is shutting down completely - maybe toss up a splash screen or something?
    Shutdown,
}

/// A message the client sends from the frontend to the devserver
///
/// This is used to communicate with the devserver
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ClientMsg {
    Log {
        level: String,
        messages: Vec<String>,
    },
}

#[cfg(feature = "devtools")]
fn client_msg_tx_slot() -> &'static Mutex<Option<mpsc::Sender<ClientMsg>>> {
    static CLIENT_MSG_TX: OnceLock<Mutex<Option<mpsc::Sender<ClientMsg>>>> = OnceLock::new();
    CLIENT_MSG_TX.get_or_init(|| Mutex::new(None))
}

#[cfg(feature = "devtools")]
pub fn send_client_log(level: impl Into<String>, message: impl Into<String>) {
    let msg = ClientMsg::Log {
        level: level.into(),
        messages: vec![message.into()],
    };

    if let Ok(slot) = client_msg_tx_slot().lock() {
        if let Some(tx) = slot.as_ref() {
            let _ = tx.send(msg);
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HotReloadMsg {
    pub assets: Vec<PathBuf>,
    pub ms_elapsed: u64,
    pub jump_table: Option<JumpTable>,
    pub for_build_id: Option<u64>,
    pub for_pid: Option<u32>,
}

impl HotReloadMsg {
    pub fn is_empty(&self) -> bool {
        self.assets.is_empty() && self.jump_table.is_none()
    }
}

/// Connect to the devserver and handle its messages with a callback.
///
/// This doesn't use any form of security or protocol, so it's not safe to expose to the internet.
#[cfg(feature = "devtools")]
pub fn connect(callback: impl FnMut(DevserverMsg) + Send + 'static) {
    let Ok(endpoint) = std::env::var("FREYA_DEVSERVER_IP") else {
        return;
    };

    connect_at(endpoint, callback);
}

/// Connect to the devserver and handle hot-patch messages only, implementing the subsecond hotpatch
/// protocol.
///
/// This is intended to be used by non-dioxus projects that want to use hotpatching.
///
/// To handle the full devserver protocol, use `connect` instead.
#[cfg(feature = "devtools")]
pub fn connect_subsecond() {
    connect(|msg| {
        if let DevserverMsg::HotReload(hot_reload_msg) = msg {
            if let Some(jumptable) = hot_reload_msg.jump_table {
                if hot_reload_msg.for_pid == Some(std::process::id()) {
                    unsafe { subsecond::apply_patch(jumptable).unwrap() };
                    send_client_log("info", "Hotpatch applied successfully");
                }
            }
        }
    });
}

#[cfg(feature = "devtools")]
pub fn connect_at(endpoint: String, mut callback: impl FnMut(DevserverMsg) + Send + 'static) {
    std::thread::spawn(move || {
        let mut stream = match TcpStream::connect(format!(
            "{}:{}",
            endpoint,
            std::env::var("FREYA_DEVSERVER_PORT").unwrap_or_else(|_| "8080".into())
        )) {
            Ok(s) => s,
            Err(_) => return,
        };

        // Send handshake with connection metadata as a length-prefixed JSON message.
        let handshake = serde_json::json!({
            "aslr_reference": subsecond::aslr_reference(),
            "build_id": std::env::var("FREYA_BUILD_ID")
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(0),
            "pid": std::process::id(),
        });
        if let Ok(bytes) = serde_json::to_vec(&handshake) {
            let len = bytes.len() as u32;
            if stream.write_all(&len.to_be_bytes()).is_err() || stream.write_all(&bytes).is_err() {
                return;
            }
        }

        // Spawn a lightweight writer loop for outbound client messages (logs, telemetry, etc).
        let (tx, rx) = mpsc::channel::<ClientMsg>();
        if let Ok(mut slot) = client_msg_tx_slot().lock() {
            *slot = Some(tx);
        }
        if let Ok(mut writer) = stream.try_clone() {
            std::thread::spawn(move || {
                while let Ok(msg) = rx.recv() {
                    let Ok(bytes) = serde_json::to_vec(&msg) else {
                        continue;
                    };

                    let len = bytes.len() as u32;
                    if writer.write_all(&len.to_be_bytes()).is_err()
                        || writer.write_all(&bytes).is_err()
                    {
                        break;
                    }
                }
            });
        }

        // Read length-prefixed JSON messages from the server.
        loop {
            let mut len_buf = [0u8; 4];
            if stream.read_exact(&mut len_buf).is_err() {
                break;
            }
            let len = u32::from_be_bytes(len_buf) as usize;
            let mut buf = vec![0u8; len];
            if stream.read_exact(&mut buf).is_err() {
                break;
            }
            if let Ok(msg) = serde_json::from_slice(&buf) {
                callback(msg);
            }
        }

        if let Ok(mut slot) = client_msg_tx_slot().lock() {
            *slot = None;
        }
    });
}

/// Run this asynchronous future to completion.
///
/// Whenever your code changes, the future is dropped and a new one is created using the new function.
///
/// This is useful for using subsecond outside of dioxus, like with axum. To pass args to the underlying
/// function, you can use the `serve_subsecond_with_args` function.
///
/// ```rust, ignore
/// #[tokio::main]
/// async fn main() {
///     dioxus_devtools::serve_subsecond(router_main).await;
/// }
///
/// async fn router_main() {
///     use axum::{Router, routing::get};
///
///     let app = Router::new().route("/", get(test_route));
///
///     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
///     println!("Server running on http://localhost:3000");
///
///     axum::serve(listener, app.clone()).await.unwrap()
/// }
///
/// async fn test_route() -> axum::response::Html<&'static str> {
///     "axum works!!!!!".into()
/// }
/// ```
#[cfg(feature = "serve")]
pub async fn serve_subsecond<O, F>(mut callback: impl FnMut() -> F)
where
    F: std::future::Future<Output = O> + 'static,
{
    serve_subsecond_with_args((), move |_args| callback()).await
}

/// Run this asynchronous future to completion.
///
/// Whenever your code changes, the future is dropped and a new one is created using the new function.
///
/// ```rust, ignore
/// #[tokio::main]
/// async fn main() {
///     let args = ("abc".to_string(),);
///     dioxus_devtools::serve_subsecond_with_args(args, router_main).await;
/// }
///
/// async fn router_main(args: (String,)) {
///     use axum::{Router, routing::get};
///
///     let app = Router::new().route("/", get(test_route));
///
///     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
///     println!("Server running on http://localhost:3000 -> {}", args.0);
///
///     axum::serve(listener, app.clone()).await.unwrap()
/// }
///
/// async fn test_route() -> axum::response::Html<&'static str> {
///     "axum works!!!!!".into()
/// }
/// ```
#[cfg(feature = "serve")]
pub async fn serve_subsecond_with_args<A: Clone, O, F>(args: A, mut callback: impl FnMut(A) -> F)
where
    F: std::future::Future<Output = O> + 'static,
{
    let (tx, mut rx) = futures_channel::mpsc::unbounded();

    connect(move |msg| {
        if let DevserverMsg::HotReload(hot_reload_msg) = msg {
            if let Some(jumptable) = hot_reload_msg.jump_table {
                if hot_reload_msg.for_pid == Some(std::process::id()) {
                    unsafe { subsecond::apply_patch(jumptable).unwrap() };
                    tx.unbounded_send(()).unwrap();
                }
            }
        }
    });

    let wrapped = move |args| -> std::pin::Pin<Box<dyn std::future::Future<Output = O>>> {
        Box::pin(callback(args))
    };

    let mut hotfn = subsecond::HotFn::current(wrapped);
    let mut cur_future = hotfn.call((args.clone(),));

    loop {
        let res = select(cur_future, rx.next()).await;

        match res {
            Either::Left(_completed) => _ = rx.next().await,
            Either::Right((None, callback)) => {
                // Receiving `None` here means that the sender is not connected, which
                // typically means the dioxus devtools protocol has never connected.
                // We want to run the future to completion and return instead of
                // re-running the future constantly in the loop.
                callback.await;
                return;
            }
            Either::Right((Some(_), _)) => {}
        }

        cur_future = hotfn.call((args.clone(),));
    }
}
