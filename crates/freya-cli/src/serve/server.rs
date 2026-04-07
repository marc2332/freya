use std::{
    net::{
        IpAddr,
        Ipv4Addr,
        SocketAddr,
        TcpListener,
    },
    sync::{
        Arc,
        RwLock,
    },
    time::Duration,
};

use anyhow::Context;
use freya_hotreload::{
    ClientMsg,
    DevserverMsg,
    HotReloadMsg,
    JumpTable,
};
use futures_channel::mpsc::{
    UnboundedReceiver,
    UnboundedSender,
};
use futures_util::{
    future,
    stream::FuturesUnordered,
    StreamExt,
};
use serde::{
    Deserialize,
    Serialize,
};
use tokio::{
    io::{
        AsyncReadExt,
        AsyncWriteExt,
        ReadHalf,
        WriteHalf,
    },
    net::TcpStream,
};

use super::AppServer;
use crate::{
    serve::ServeUpdate,
    BuildId,
    BuildStage,
    BuilderUpdate,
    BundleFormat,
    Result,
};

/// The TCP server that handles devtools communication with connected clients.
///
/// Clients connect via raw TCP and exchange length-prefixed JSON messages
/// (4-byte big-endian length followed by JSON payload).
pub(crate) struct WebServer {
    devserver_exposed_ip: IpAddr,
    devserver_bind_ip: IpAddr,
    devserver_port: u16,
    proxied_port: Option<u16>,
    hot_reload_sockets: Vec<ConnectedClient>,
    build_status_sockets: Vec<ConnectedClient>,
    new_hot_reload_sockets: UnboundedReceiver<ConnectedClient>,
    new_build_status_sockets: UnboundedReceiver<ConnectedClient>,
    build_status: SharedStatus,
    application_name: String,
    bundle: BundleFormat,
}

pub(crate) struct ConnectedClient {
    reader: ReadHalf<TcpStream>,
    writer: WriteHalf<TcpStream>,
    build_id: Option<BuildId>,
    aslr_reference: Option<u64>,
    pid: Option<u32>,
}

impl ConnectedClient {
    /// Send a length-prefixed byte payload to the client.
    async fn send_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        let len = bytes.len() as u32;
        self.writer.write_all(&len.to_be_bytes()).await?;
        self.writer.write_all(bytes).await?;
        Ok(())
    }

    /// Read the next length-prefixed message from the client and deserialize it.
    async fn recv_client_msg(&mut self) -> Option<ClientMsg> {
        let mut len_buf = [0u8; 4];
        self.reader.read_exact(&mut len_buf).await.ok()?;
        let len = u32::from_be_bytes(len_buf) as usize;
        let mut buf = vec![0u8; len];
        self.reader.read_exact(&mut buf).await.ok()?;
        serde_json::from_slice(&buf).ok()
    }
}

impl WebServer {
    pub const SELF_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    /// Start the development TCP server.
    ///
    /// Clients send a JSON handshake on connect; those carrying `aslr_reference`
    /// are treated as hot-reload clients, the rest as build-status clients.
    pub(crate) fn start(runner: &AppServer) -> Result<Self> {
        let (hot_reload_sockets_tx, hot_reload_sockets_rx) = futures_channel::mpsc::unbounded();
        let (build_status_sockets_tx, build_status_sockets_rx) = futures_channel::mpsc::unbounded();

        let devserver_bind_ip = runner.devserver_bind_ip;
        let devserver_port = runner.devserver_port;
        let proxied_port = runner.proxied_port;
        let devserver_exposed_ip = devserver_bind_ip;

        let devserver_bind_address = SocketAddr::new(devserver_bind_ip, devserver_port);
        let listener = std::net::TcpListener::bind(devserver_bind_address).with_context(|| {
            anyhow::anyhow!(
                "Failed to bind server to: {devserver_bind_address}, is there another devserver running?\nTo run multiple devservers, use the --port flag to specify a different port"
            )
        })?;

        let build_status = SharedStatus::new_with_starting_build();

        tokio::spawn(devserver_mainloop(
            listener,
            hot_reload_sockets_tx,
            build_status_sockets_tx,
        ));

        Ok(Self {
            build_status,
            proxied_port,
            devserver_bind_ip,
            devserver_exposed_ip,
            devserver_port,
            hot_reload_sockets: Default::default(),
            build_status_sockets: Default::default(),
            new_hot_reload_sockets: hot_reload_sockets_rx,
            new_build_status_sockets: build_status_sockets_rx,
            application_name: runner.app_name().to_string(),
            bundle: runner.client.build.bundle,
        })
    }

    /// Wait for new clients to be connected and then save them.
    pub(crate) async fn wait(&mut self) -> ServeUpdate {
        let mut new_hot_reload_socket = self.new_hot_reload_sockets.next();
        let mut new_build_status_socket = self.new_build_status_sockets.next();
        let mut new_message = self
            .hot_reload_sockets
            .iter_mut()
            .enumerate()
            .map(|(idx, socket)| async move { (idx, socket.recv_client_msg().await) })
            .collect::<FuturesUnordered<_>>();

        tokio::select! {
            new_hot_reload_socket = &mut new_hot_reload_socket => {
                if let Some(new_socket) = new_hot_reload_socket {
                    let aslr_reference = new_socket.aslr_reference;
                    let pid = new_socket.pid;
                    let id = new_socket.build_id.unwrap_or(BuildId::PRIMARY);

                    drop(new_message);
                    self.hot_reload_sockets.push(new_socket);

                    return ServeUpdate::NewConnection { aslr_reference, id, pid };
                } else {
                    panic!("Could not receive a socket - the devtools could not boot - the port is likely already in use");
                }
            }
            new_build_status_socket = &mut new_build_status_socket => {
                if let Some(mut new_socket) = new_build_status_socket {
                    drop(new_message);

                    let project_info = SharedStatus::new(Status::ClientInit {
                        application_name: self.application_name.clone(),
                        bundle: self.bundle,
                    });
                    if project_info.send_to(&mut new_socket).await.is_ok() {
                        _ = self.build_status.send_to(&mut new_socket).await;
                        self.build_status_sockets.push(new_socket);
                    }
                    return future::pending::<ServeUpdate>().await;
                } else {
                    panic!("Could not receive a socket - the devtools could not boot - the port is likely already in use");
                }
            }
            Some((idx, message)) = new_message.next() => {
                match message {
                    Some(msg) => return ServeUpdate::WsMessage { msg, bundle: self.bundle },
                    None => {
                        drop(new_message);
                        _ = self.hot_reload_sockets.remove(idx);
                    }
                }
            }
        }

        future::pending().await
    }

    pub(crate) async fn shutdown(&mut self) {
        self.send_shutdown().await;
        // Dropping the clients closes the TCP connections.
        self.hot_reload_sockets.drain(..);
    }

    /// Sends the current build status to all clients.
    async fn send_build_status(&mut self) {
        let status = self.build_status.clone();
        let mut i = 0;
        while i < self.build_status_sockets.len() {
            if status
                .send_to(&mut self.build_status_sockets[i])
                .await
                .is_err()
            {
                self.build_status_sockets.remove(i);
            } else {
                i += 1;
            }
        }
    }

    /// Sends a start build message to all clients.
    pub(crate) async fn start_build(&mut self) {
        self.build_status.set(Status::Building {
            progress: 0.0,
            build_message: "Starting the build...".to_string(),
        });
        self.send_build_status().await;
    }

    /// Sends an updated build status to all clients.
    pub(crate) async fn new_build_update(&mut self, update: &BuilderUpdate) {
        match update {
            BuilderUpdate::Progress { stage } => {
                // Todo(miles): wire up more messages into the splash screen UI
                match stage {
                    BuildStage::Success => {}
                    BuildStage::Failed => self.send_reload_failed().await,
                    BuildStage::Restarting => self.send_reload_start().await,
                    BuildStage::Initializing => {}
                    BuildStage::InstallingTooling => {}
                    BuildStage::Compiling {
                        current,
                        total,
                        krate,
                        ..
                    } => {
                        if !matches!(
                            self.build_status.get(),
                            Status::Ready | Status::BuildError { .. }
                        ) {
                            self.build_status.set(Status::Building {
                                progress: (*current as f64 / *total as f64).clamp(0.0, 1.0),
                                build_message: format!("{krate} compiling"),
                            });
                            self.send_build_status().await;
                        }
                    }
                    BuildStage::OptimizingWasm => {}
                    BuildStage::Aborted => {}
                    BuildStage::CopyingAssets { .. } => {}
                    _ => {}
                }
            }
            BuilderUpdate::CompilerMessage { .. } => {}
            BuilderUpdate::BuildReady { .. } => {}
            BuilderUpdate::BuildFailed { err } => {
                let error = err.to_string();
                self.build_status.set(Status::BuildError {
                    error: ansi_to_html::convert(&error).unwrap_or(error),
                });
                self.send_reload_failed().await;
                self.send_build_status().await;
            }
            BuilderUpdate::StdoutReceived { .. } => {}
            BuilderUpdate::StderrReceived { .. } => {}
            BuilderUpdate::ProcessExited { .. } => {}
            BuilderUpdate::ProcessWaitFailed { .. } => {}
        }
    }

    pub(crate) fn has_hotreload_sockets(&self) -> bool {
        !self.hot_reload_sockets.is_empty()
    }

    /// Sends hot reloadable changes to all clients.
    pub(crate) async fn send_hotreload(&mut self, reload: HotReloadMsg) {
        if reload.is_empty() {
            return;
        }

        tracing::trace!("Sending hotreload to clients {:?}", reload);

        let bytes = serde_json::to_vec(&DevserverMsg::HotReload(reload)).unwrap();

        let mut i = 0;
        while i < self.hot_reload_sockets.len() {
            if self.hot_reload_sockets[i].send_bytes(&bytes).await.is_err() {
                self.hot_reload_sockets.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub(crate) async fn send_patch(
        &mut self,
        jump_table: JumpTable,
        time_taken: Duration,
        build: BuildId,
        for_pid: Option<u32>,
    ) {
        let msg = DevserverMsg::HotReload(HotReloadMsg {
            jump_table: Some(jump_table),
            ms_elapsed: time_taken.as_millis() as u64,
            assets: vec![],
            for_pid,
            for_build_id: Some(build.0 as _),
        });
        self.send_devserver_message_to_all(msg).await;
        self.set_ready().await;
    }

    /// Tells all clients that a hot patch has started.
    pub(crate) async fn send_patch_start(&mut self) {
        self.send_devserver_message_to_all(DevserverMsg::HotPatchStart)
            .await;
    }

    /// Tells all clients that a full rebuild has started.
    pub(crate) async fn send_reload_start(&mut self) {
        self.send_devserver_message_to_all(DevserverMsg::FullReloadStart)
            .await;
    }

    /// Tells all clients that a full rebuild has failed.
    pub(crate) async fn send_reload_failed(&mut self) {
        self.send_devserver_message_to_all(DevserverMsg::FullReloadFailed)
            .await;
    }

    /// Tells all clients to reload if possible for new changes.
    pub(crate) async fn send_reload_command(&mut self) {
        self.set_ready().await;
        self.send_devserver_message_to_all(DevserverMsg::FullReloadCommand)
            .await;
    }

    /// Send a shutdown message to all connected clients.
    pub(crate) async fn send_shutdown(&mut self) {
        self.send_devserver_message_to_all(DevserverMsg::Shutdown)
            .await;
    }

    /// Sends a devserver message to all connected clients.
    async fn send_devserver_message_to_all(&mut self, msg: DevserverMsg) {
        let bytes = serde_json::to_vec(&msg).unwrap();
        for socket in self.hot_reload_sockets.iter_mut() {
            _ = socket.send_bytes(&bytes).await;
        }
    }

    /// Mark the devserver status as ready and notify listeners.
    async fn set_ready(&mut self) {
        if matches!(self.build_status.get(), Status::Ready) {
            return;
        }

        self.build_status.set(Status::Ready);
        self.send_build_status().await;
    }

    /// Get the address the devserver should run on
    pub fn devserver_address(&self) -> SocketAddr {
        SocketAddr::new(self.devserver_exposed_ip, self.devserver_port)
    }

    // Get the address the server should run on if we're serving the user's server
    pub fn proxied_server_address(&self) -> Option<SocketAddr> {
        self.proxied_port
            .map(|port| SocketAddr::new(self.devserver_exposed_ip, port))
    }

    pub fn server_address(&self) -> Option<SocketAddr> {
        self.proxied_server_address()
    }

    /// Get the address the server is running - showing 127.0.0.1 if the devserver is bound to 0.0.0.0
    /// This is designed this way to not confuse users who expect the devserver to be bound to localhost
    /// ... which it is, but they don't know that 0.0.0.0 also serves localhost.
    pub fn displayed_address(&self) -> Option<SocketAddr> {
        let mut address = self.server_address()?;

        // Set the port to the devserver port since that's usually what people expect
        address.set_port(self.devserver_port);

        if self.devserver_bind_ip == IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)) {
            address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), address.port());
        }

        Some(address)
    }
}

#[derive(Deserialize)]
struct TcpHandshake {
    aslr_reference: Option<u64>,
    build_id: Option<BuildId>,
    pid: Option<u32>,
}

/// Accept TCP connections, read the handshake, and route each client to the
/// appropriate channel based on whether it carries `aslr_reference`.
async fn devserver_mainloop(
    listener: TcpListener,
    hot_reload_tx: UnboundedSender<ConnectedClient>,
    build_status_tx: UnboundedSender<ConnectedClient>,
) -> Result<()> {
    let _ = listener.set_nonblocking(true);
    let listener = tokio::net::TcpListener::from_std(listener)?;

    loop {
        let (stream, _addr) = match listener.accept().await {
            Ok(c) => c,
            Err(_) => continue,
        };

        let hot_reload_tx = hot_reload_tx.clone();
        let build_status_tx = build_status_tx.clone();

        tokio::spawn(async move {
            let (mut reader, writer) = tokio::io::split(stream);

            let mut len_buf = [0u8; 4];
            if reader.read_exact(&mut len_buf).await.is_err() {
                return;
            }
            let len = u32::from_be_bytes(len_buf) as usize;
            let mut buf = vec![0u8; len];
            if reader.read_exact(&mut buf).await.is_err() {
                return;
            }

            let handshake: TcpHandshake = match serde_json::from_slice(&buf) {
                Ok(h) => h,
                Err(_) => return,
            };

            tracing::debug!(
                "New TCP devtools connection: aslr={:?} build_id={:?} pid={:?}",
                handshake.aslr_reference,
                handshake.build_id,
                handshake.pid
            );

            let client = ConnectedClient {
                reader,
                writer,
                build_id: handshake.build_id,
                aslr_reference: handshake.aslr_reference,
                pid: handshake.pid,
            };

            if handshake.aslr_reference.is_some() {
                _ = hot_reload_tx.unbounded_send(client);
            } else {
                _ = build_status_tx.unbounded_send(client);
            }
        });
    }
}

#[derive(Debug, Clone)]
struct SharedStatus(Arc<RwLock<Status>>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
enum Status {
    ClientInit {
        application_name: String,
        bundle: BundleFormat,
    },
    Building {
        progress: f64,
        build_message: String,
    },
    BuildError {
        error: String,
    },
    Ready,
}

impl SharedStatus {
    fn new(status: Status) -> Self {
        Self(Arc::new(RwLock::new(status)))
    }

    fn new_with_starting_build() -> Self {
        Self::new(Status::Building {
            progress: 0.0,
            build_message: "Starting the build...".to_string(),
        })
    }

    fn set(&self, status: Status) {
        *self.0.write().unwrap() = status;
    }

    fn get(&self) -> Status {
        self.0.read().unwrap().clone()
    }

    async fn send_to(&self, client: &mut ConnectedClient) -> Result<()> {
        let bytes = serde_json::to_vec(&self.get())?;
        client.send_bytes(&bytes).await
    }
}
