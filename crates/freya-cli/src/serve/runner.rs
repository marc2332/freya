use super::{AppBuilder, ServeUpdate, WebServer};
use crate::BuildStage;
use crate::{
    platform_override::CommandWithPlatformOverrides, BuildArtifacts, BuildId, BuildMode,
    BuildTargets, BuilderUpdate, HotpatchModuleCache, Result, ServeArgs, TraceSrc, Workspace,
};
use anyhow::{bail, Context};
use freya_hotreload::HotReloadMsg;
use futures_channel::mpsc::{UnboundedReceiver, UnboundedSender};
use futures_util::future::OptionFuture;
use futures_util::StreamExt;
use krates::NodeId;
use notify::{
    event::{MetadataKind, ModifyKind},
    Config, EventKind, RecursiveMode, Watcher as NotifyWatcher,
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    net::{IpAddr, TcpListener},
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

/// This is the primary "state" object that holds the builds and handles for the running apps.
///
/// It also holds the watcher which is used to watch for changes in the filesystem and trigger rebuilds,
/// hotreloads, asset updates, etc.
///
/// Since we resolve the build request before initializing the CLI, it also serves as a place to store
/// resolved "serve" arguments, which is why it takes ServeArgs instead of BuildArgs. Simply wrap the
/// BuildArgs in a default ServeArgs and pass it in.
pub(crate) struct AppServer {
    /// the platform of the "primary" crate (ie the first)
    pub(crate) workspace: Arc<Workspace>,

    pub(crate) client: AppBuilder,
    pub(crate) server: Option<AppBuilder>,

    // Related to the filesystem watcher
    pub(crate) watcher: Box<dyn notify::Watcher>,
    pub(crate) _watcher_tx: UnboundedSender<notify::Event>,
    pub(crate) watcher_rx: UnboundedReceiver<notify::Event>,

    // Tracked state related to open builds and hot reloading
    pub(crate) applied_client_hot_reload_message: HotReloadMsg,
    pub(crate) file_map: HashMap<PathBuf, CachedFile>,

    // Resolved args related to how we go about processing the rebuilds and logging
    pub(crate) use_hotpatch_engine: bool,
    pub(crate) automatic_rebuilds: bool,
    pub(crate) interactive: bool,
    pub(crate) _force_sequential: bool,
    pub(crate) hot_reload: bool,
    pub(crate) open_browser: bool,
    pub(crate) _wsl_file_poll_interval: u16,
    pub(crate) always_on_top: bool,
    pub(crate) fullstack: bool,
    pub(crate) ssg: bool,
    pub(crate) watch_fs: bool,

    // resolve args related to the webserver
    pub(crate) devserver_port: u16,
    pub(crate) devserver_bind_ip: IpAddr,
    pub(crate) proxied_port: Option<u16>,
    pub(crate) cross_origin_policy: bool,

    // The arguments that should be forwarded to the client app when it is opened
    pub(crate) client_args: Vec<String>,
    // The arguments that should be forwarded to the server app when it is opened
    pub(crate) server_args: Vec<String>,

    // File changes that arrived while a build was in progress, to be processed after build completes
    pub(crate) pending_file_changes: Vec<PathBuf>,
}

pub(crate) struct CachedFile {
    contents: String,
    most_recent: Option<String>,
}

impl AppServer {
    /// Create the AppRunner and then initialize the filemap with the crate directory.
    pub(crate) async fn new(args: ServeArgs) -> Result<Self> {
        let workspace = Workspace::current().await?;

        // Resolve the simpler args
        let interactive = args.is_interactive_tty();
        let force_sequential = args.platform_args.shared.targets.force_sequential_build();
        let cross_origin_policy = args.cross_origin_policy;

        // Find the launch args for the client and server
        let split_args = |args: &str| {
            args.split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        };

        let server_args = args.platform_args.with_server_or_shared(|c| &c.args);
        let server_args = split_args(server_args);
        let client_args = args.platform_args.with_client_or_shared(|c| &c.args);
        let client_args = split_args(client_args);

        // These come from the args but also might come from the workspace settings
        // We opt to use the manually specified args over the workspace settings
        let hot_reload = args
            .hot_reload
            .unwrap_or_else(|| workspace.settings.always_hot_reload.unwrap_or(true));

        let open_browser = args
            .open
            .unwrap_or_else(|| workspace.settings.always_open_browser.unwrap_or(false))
            && interactive;

        let wsl_file_poll_interval = args
            .wsl_file_poll_interval
            .unwrap_or_else(|| workspace.settings.wsl_file_poll_interval.unwrap_or(2));

        let always_on_top = args
            .always_on_top
            .unwrap_or_else(|| workspace.settings.always_on_top.unwrap_or(true));

        // Use 127.0.0.1 as the default address if none is specified.
        // If the user wants to export on the network, they can use `0.0.0.0` instead.
        let devserver_bind_ip = args.address.addr.unwrap_or(WebServer::SELF_IP);

        // If the user specified a port, use that, otherwise use any available port, preferring 8080
        let devserver_port = args
            .address
            .port
            .unwrap_or_else(|| get_available_port(devserver_bind_ip, Some(8080)).unwrap_or(8080));

        // Spin up the file watcher
        let (watcher_tx, watcher_rx) = futures_channel::mpsc::unbounded();
        let watcher = create_notify_watcher(watcher_tx.clone(), wsl_file_poll_interval as u64);

        let ssg = args.platform_args.shared.targets.ssg;
        let target_args = CommandWithPlatformOverrides {
            shared: args.platform_args.shared.targets,
            server: args.platform_args.server.map(|s| s.targets),
            client: args.platform_args.client.map(|c| c.targets),
        };
        let BuildTargets { client, server } = target_args.into_targets().await?;

        // All servers will end up behind us (the devserver) but on a different port
        // This is so we can serve a loading screen as well as devtools without anything particularly fancy
        let fullstack = server.is_some();
        let should_proxy_port = fullstack && !ssg;

        let proxied_port = should_proxy_port
            .then(|| get_available_port(devserver_bind_ip, None))
            .flatten();

        let watch_fs = args.watch.unwrap_or(true);
        let use_hotpatch_engine = args.hot_patch;

        let client = AppBuilder::new(&client)?;
        let server = server.map(|server| AppBuilder::new(&server)).transpose()?;

        // Create the runner
        let mut runner = Self {
            file_map: Default::default(),
            applied_client_hot_reload_message: Default::default(),
            automatic_rebuilds: true,
            watch_fs,
            use_hotpatch_engine,
            client,
            server,
            hot_reload,
            open_browser,
            _wsl_file_poll_interval: wsl_file_poll_interval,
            always_on_top,
            workspace,
            devserver_port,
            devserver_bind_ip,
            proxied_port,
            watcher,
            watcher_rx,
            _watcher_tx: watcher_tx,
            interactive,
            _force_sequential: force_sequential,
            cross_origin_policy,
            fullstack,
            ssg,
            server_args,
            client_args,
            pending_file_changes: Vec::new(),
        };

        // Only register the hot-reload stuff if we're watching the filesystem
        if runner.watch_fs {
            // Spin up the notify watcher
            // When builds load though, we're going to parse their depinfo and add the paths to the watcher
            runner.watch_filesystem();

            // todo(jon): this might take a while so we should try and background it, or make it lazy somehow
            // we could spawn a thread to search the FS and then when it returns we can fill the filemap
            // in testing, if this hits a massive directory, it might take several seconds with no feedback.
            // really, we should be using depinfo to get the files that are actually used, but the depinfo file might not be around yet
            // todo(jon): see if we can just guess the depinfo file before it generates. might be stale but at least it catches most of the files
            runner.load_rsx_filemap();
        }

        Ok(runner)
    }

    pub(crate) fn initialize(&mut self) {
        let build_mode = match self.use_hotpatch_engine {
            true => BuildMode::Fat,
            false => BuildMode::Base { run: true },
        };

        self.client.start(build_mode.clone(), BuildId::PRIMARY);
        if let Some(server) = self.server.as_mut() {
            server.start(build_mode, BuildId::SECONDARY);
        }
    }

    /// Take any pending file changes that were queued while a build was in progress.
    /// Returns the files and clears the pending list.
    pub(crate) fn take_pending_file_changes(&mut self) -> Vec<PathBuf> {
        std::mem::take(&mut self.pending_file_changes)
    }

    pub(crate) async fn wait(&mut self) -> ServeUpdate {
        let client = &mut self.client;
        let server = self.server.as_mut();

        let client_wait = client.wait();
        let server_wait = OptionFuture::from(server.map(|s| s.wait()));
        let watcher_wait = self.watcher_rx.next();

        tokio::select! {
            // Wait for the client to finish
            client_update = client_wait => {
                ServeUpdate::BuilderUpdate {
                    id: BuildId::PRIMARY,
                    update: client_update,
                }
            }

            Some(server_update) = server_wait => {
                ServeUpdate::BuilderUpdate {
                    id: BuildId::SECONDARY,
                    update: server_update,
                }
            }

            // Wait for the watcher to send us an event
            event = watcher_wait => {
                let mut changes: Vec<_> = event.into_iter().collect();

                // Dequeue in bulk if we can, we might've received a lot of events in one go
                while let Ok(event) = self.watcher_rx.try_recv() {
                    changes.push(event);
                }

                // Filter the changes
                let mut files: Vec<PathBuf> = vec![];

                // Decompose the events into a list of all the files that have changed
                for event in changes.drain(..) {
                    // Make sure we add new folders to the watch list, provided they're not matched by the ignore list
                    // We'll only watch new folders that are found under the crate, and then update our watcher to watch them
                    // This unfortunately won't pick up new krates added "at a distance" - IE krates not within the workspace.
                    if let EventKind::Create(_create_kind) = event.kind {
                        // If it's a new folder, watch it
                        // If it's a new cargo.toml (ie dep on the fly),
                        // todo(jon) support new folders on the fly
                    }

                    for path in event.paths {
                        // Workaround for notify and vscode-like editor:
                        // - when edit & save a file in vscode, there will be two notifications,
                        // - the first one is a file with empty content.
                        // - filter the empty file notification to avoid false rebuild during hot-reload
                        if let Ok(metadata) = std::fs::metadata(&path) {
                            if metadata.len() == 0 {
                                continue;
                            }
                        }

                        files.push(path);
                    }
                }

                ServeUpdate::FilesChanged { files }
            }

        }
    }

    /// Handle an update from the builder
    pub(crate) async fn new_build_update(
        &mut self,
        update: &BuilderUpdate,
        _devserver: &WebServer,
    ) {
        // Nothing to do here for native builds
        let _ = update;
    }

    /// Handle the list of changed files from the file watcher, attempting to aggressively prevent
    /// full rebuilds by hot-reloading RSX and hot-patching Rust code.
    ///
    /// This will also handle any assets that are linked in the files, and copy them to the bundle
    /// and send them to the client.
    pub(crate) async fn handle_file_change(&mut self, files: &[PathBuf], server: &mut WebServer) {
        // We can attempt to hotpatch if the build is in a bad state, since this patch might be a recovery.
        if !matches!(
            self.client.stage,
            BuildStage::Failed | BuildStage::Aborted | BuildStage::Success
        ) {
            // Queue file changes that arrive during a build, so we can process them after the build completes.
            // This prevents losing changes from tools like stylance, tailwind, or sass that generate files
            // in response to source changes.
            tracing::debug!(
                "Queueing file change: client is not ready to receive hotreloads. Files: {:#?}",
                files
            );
            self.pending_file_changes.extend(files.iter().cloned());
            return;
        }

        // Prepare the hotreload message we need to send
        let mut assets = Vec::new();
        let mut needs_full_rebuild = false;

        for path in files {
            let ext = path
                .extension()
                .and_then(|v| v.to_str())
                .unwrap_or_default();

            // If it's an asset, we want to hotreload it
            if let Some(bundled_names) = self.client.hotreload_bundled_assets(path).await {
                for bundled_name in bundled_names {
                    assets.push(PathBuf::from("/assets/").join(bundled_name));
                }
            }

            // If it's in the public dir, trigger a full rebuild
            if self.client.build.path_is_in_public_dir(path) {
                needs_full_rebuild = true;
                continue;
            }

            // Rust file changes always need a rebuild/hotpatch
            if ext == "rs" {
                // Update cached contents for when we force a rebuild
                if let Some(cached_file) = self.file_map.get_mut(path) {
                    if let Ok(new_contents) = std::fs::read_to_string(path) {
                        cached_file.most_recent = Some(new_contents);
                    }
                }
                needs_full_rebuild = true;
                continue;
            }

            // Non-rust files depended on via include! or similar
            if let Some(artifacts) = self.client.artifacts.as_ref() {
                if artifacts.depinfo.files.contains(path) {
                    needs_full_rebuild = true;
                    break;
                }
            }
        }

        // todo - we need to distinguish between hotpatchable rebuilds and true full rebuilds.
        //        A full rebuild is required when the user modifies static initializers which we haven't wired up yet.
        if needs_full_rebuild && self.automatic_rebuilds {
            if self.use_hotpatch_engine {
                let changed_crates = self.order_changed_crates(files);

                self.client
                    .patch_rebuild(files.to_vec(), changed_crates.clone(), BuildId::PRIMARY);

                if let Some(server) = self.server.as_mut() {
                    server.patch_rebuild(files.to_vec(), changed_crates, BuildId::SECONDARY);
                }
                self.clear_hot_reload_changes();
                self.clear_cached_rsx();
                server.send_patch_start().await;
            } else {
                self.client
                    .start_rebuild(BuildMode::Base { run: true }, BuildId::PRIMARY);
                if let Some(server) = self.server.as_mut() {
                    server.start_rebuild(BuildMode::Base { run: true }, BuildId::SECONDARY);
                }
                self.clear_hot_reload_changes();
                self.clear_cached_rsx();
                server.send_reload_start().await;
            }
        } else {
            let msg = HotReloadMsg {
                assets,
                ms_elapsed: 0,
                jump_table: Default::default(),
                for_build_id: None,
                for_pid: None,
            };

            self.add_hot_reload_message(&msg);

            let file = files[0].display().to_string();
            let file =
                file.trim_start_matches(&self.client.build.crate_dir().display().to_string());

            if needs_full_rebuild && !self.automatic_rebuilds {
                use crate::styles::NOTE_STYLE;
                tracing::warn!(
                    "Ignoring full rebuild for: {NOTE_STYLE}{}{NOTE_STYLE:#}",
                    file
                );
            }

            // Only send a hotreload message for templates and assets - otherwise we'll just get a full rebuild
            //
            // todo: move the android file uploading out of hotreload_bundled_asset and
            //
            // Also make sure the builder isn't busy since that might cause issues with hotreloads
            // https://github.com/DioxusLabs/dioxus/issues/3361
            if !msg.is_empty() && self.client.can_receive_hotreloads() {
                use crate::styles::NOTE_STYLE;
                tracing::info!(dx_src = ?TraceSrc::Dev, "Hotreloading: {NOTE_STYLE}{}{NOTE_STYLE:#}", file);

                if !server.has_hotreload_sockets() {
                    tracing::warn!("No clients to hotreload - try reloading the app!");
                }

                server.send_hotreload(msg).await;
            } else {
                tracing::debug!(dx_src = ?TraceSrc::Dev, "Ignoring file change: {}", file);
            }
        }
    }

    /// Finally "bundle" this app and return a handle to it
    pub(crate) async fn open(
        &mut self,
        artifacts: &BuildArtifacts,
        devserver: &mut WebServer,
    ) -> Result<()> {
        // Make sure to save artifacts regardless of if we're opening the app or not
        match artifacts.build_id {
            BuildId::PRIMARY => self.client.artifacts = Some(artifacts.clone()),
            BuildId::SECONDARY => {
                if let Some(server) = self.server.as_mut() {
                    server.artifacts = Some(artifacts.clone());
                }
            }
            _ => {}
        }

        let should_open = self.client.stage == BuildStage::Success
            && (self.server.as_ref().map(|s| s.stage == BuildStage::Success)).unwrap_or(true);

        use crate::cli::styles::GLOW_STYLE;

        if should_open {
            let time_taken = artifacts
                .time_end
                .duration_since(artifacts.time_start)
                .unwrap();

            if self.client.builds_opened == 0 {
                tracing::info!(
                    "Build completed successfully in {GLOW_STYLE}{}{GLOW_STYLE:#}, launching app! 💫",
                    format_duration_ms(time_taken)
                );
            } else {
                tracing::info!(
                    "Build completed in {GLOW_STYLE}{}{GLOW_STYLE:#}",
                    format_duration_ms(time_taken)
                );
            }

            self.open_all(devserver).await?;

            // Give a second for the server to boot
            tokio::time::sleep(Duration::from_millis(300)).await;

            // Update the screen + devserver with the new handle info
            devserver.send_reload_command().await
        }

        Ok(())
    }

    /// Open an existing app bundle, if it exists
    ///
    /// Will attempt to open the server and client together, in a coordinated way such that the server
    /// opens first, initializes, and then the client opens.
    ///
    /// There's a number of issues we need to be careful to work around:
    /// - The server failing to boot or crashing on startup (and entering a boot loop)
    /// -
    pub(crate) async fn open_all(&mut self, devserver: &WebServer) -> Result<()> {
        let devserver_ip = devserver.devserver_address();
        let fullstack_address = devserver.proxied_server_address();

        // Always open the server first after the client has been built
        // Only open the server if it isn't prerendered and finished building
        if let Some(server) = self.server.as_mut().filter(|_| !self.ssg) {
            if server.stage < BuildStage::Success {
                tracing::trace!("Skipping server open: will open once build completes");
            } else {
                tracing::debug!("Opening server build");
                server.soft_kill().await;
                server
                    .open(
                        devserver_ip,
                        fullstack_address,
                        false,
                        BuildId::SECONDARY,
                        &self.server_args,
                    )
                    .await?;
            }
        }

        // Skip opening native client if still building
        if self.client.stage < BuildStage::Success {
            tracing::trace!("Skipping client open: will open once build completes");
            return Ok(());
        }

        // Start the new app before we kill the old one to give it a little bit of time
        self.client.soft_kill().await;
        self.client
            .open(
                devserver_ip,
                fullstack_address,
                self.always_on_top,
                BuildId::PRIMARY,
                &self.client_args,
            )
            .await?;

        Ok(())
    }

    /// Shutdown all the running processes
    pub(crate) async fn shutdown(&mut self) -> Result<()> {
        self.client.soft_kill().await;

        if let Some(server) = self.server.as_mut() {
            server.soft_kill().await;
        }

        Ok(())
    }

    /// Perform a full rebuild of the app, equivalent to `cargo rustc` from scratch with no incremental
    /// hot-patch engine integration.
    pub(crate) async fn full_rebuild(&mut self) {
        let build_mode = match self.use_hotpatch_engine {
            true => BuildMode::Fat,
            false => BuildMode::Base { run: true },
        };

        self.client
            .start_rebuild(build_mode.clone(), BuildId::PRIMARY);
        if let Some(s) = self.server.as_mut() {
            s.start_rebuild(build_mode, BuildId::SECONDARY);
        }

        self.clear_hot_reload_changes();
        self.clear_cached_rsx();
        self.clear_patches();
    }

    pub(crate) async fn hotpatch(
        &mut self,
        bundle: &BuildArtifacts,
        id: BuildId,
        cache: &HotpatchModuleCache,
        devserver: &mut WebServer,
    ) -> Result<()> {
        let elapsed = bundle
            .time_end
            .duration_since(bundle.time_start)
            .unwrap_or_default();

        let jump_table = match id {
            BuildId::PRIMARY => self.client.hotpatch(bundle, cache).await,
            BuildId::SECONDARY => {
                self.server
                    .as_mut()
                    .context("Server not found")?
                    .hotpatch(bundle, cache)
                    .await
            }
            _ => bail!("Invalid build id"),
        }?;

        if id == BuildId::PRIMARY {
            self.applied_client_hot_reload_message.jump_table = self.client.patches.last().cloned();
        }

        // If no server, just send the patch immediately
        let Some(server) = self.server.as_mut() else {
            devserver
                .send_patch(jump_table, elapsed, id, self.client.pid)
                .await;
            return Ok(());
        };

        // If we have a server, we need to wait until both the client and server are ready
        // Otherwise we end up with an annoying race condition where the client can't actually load the patch
        if self.client.stage == BuildStage::Success && server.stage == BuildStage::Success {
            let client_jump_table = self
                .client
                .patches
                .last()
                .cloned()
                .context("Missing client jump table")?;

            let server_jump_table = server
                .patches
                .last()
                .cloned()
                .context("Missing server jump table")?;

            devserver
                .send_patch(server_jump_table, elapsed, BuildId::SECONDARY, server.pid)
                .await;

            devserver
                .send_patch(
                    client_jump_table,
                    elapsed,
                    BuildId::PRIMARY,
                    self.client.pid,
                )
                .await;
        }

        Ok(())
    }

    pub(crate) fn get_build(&self, id: BuildId) -> Option<&AppBuilder> {
        match id {
            BuildId::PRIMARY => Some(&self.client),
            BuildId::SECONDARY => self.server.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn client(&self) -> &AppBuilder {
        &self.client
    }

    /// The name of the app being served, to display
    pub(crate) fn app_name(&self) -> &str {
        self.client.build.executable_name()
    }

    /// Get any hot reload changes that have been applied since the last full rebuild
    pub(crate) fn applied_hot_reload_changes(&mut self, build: BuildId) -> HotReloadMsg {
        let mut msg = self.applied_client_hot_reload_message.clone();

        if build == BuildId::PRIMARY {
            msg.jump_table = self.client.patches.last().cloned();
            msg.for_build_id = Some(BuildId::PRIMARY.0 as _);
            if let Some(lib) = msg.jump_table.as_mut() {
                lib.lib = PathBuf::from("/").join(lib.lib.clone());
            }
        }

        if build == BuildId::SECONDARY {
            if let Some(server) = self.server.as_mut() {
                msg.jump_table = server.patches.last().cloned();
                msg.for_build_id = Some(BuildId::SECONDARY.0 as _);
            }
        }

        msg
    }

    /// Clear the hot reload changes. This should be called any time a new build is starting
    pub(crate) fn clear_hot_reload_changes(&mut self) {
        self.applied_client_hot_reload_message = Default::default();
    }

    pub(crate) fn clear_patches(&mut self) {
        self.client.patches.clear();
        if let Some(server) = self.server.as_mut() {
            server.patches.clear();
        }
    }

    pub(crate) async fn client_connected(
        &mut self,
        build_id: BuildId,
        aslr_reference: Option<u64>,
        pid: Option<u32>,
    ) {
        match build_id {
            BuildId::PRIMARY => {
                if let Some(aslr_reference) = aslr_reference {
                    self.client.aslr_reference = Some(aslr_reference);
                }
                if let Some(pid) = pid {
                    self.client.pid = Some(pid);
                }
            }
            BuildId::SECONDARY => {
                if let Some(server) = self.server.as_mut() {
                    server.aslr_reference = aslr_reference;
                }
            }
            _ => {}
        }
    }

    /// Store the hot reload changes for any future clients that connect
    fn add_hot_reload_message(&mut self, msg: &HotReloadMsg) {
        let applied = &mut self.applied_client_hot_reload_message;
        let mut assets: HashSet<PathBuf> =
            std::mem::take(&mut applied.assets).into_iter().collect();
        assets.extend(msg.assets.iter().cloned());
        applied.assets = assets.into_iter().collect();
        applied.jump_table = self.client.patches.last().cloned();
    }

    /// Register the files from the workspace into our file watcher.
    ///
    /// This very simply looks for all Rust files in the workspace and adds them to the filemap.
    ///
    /// Once the builds complete we'll use the depinfo files to get the actual files that are used,
    /// making our watcher more accurate. Filling the filemap here is intended to catch any file changes
    /// in between the first build and the depinfo file being generated.
    ///
    /// We don't want watch any registry files since that generally causes a huge performance hit -
    /// we mostly just care about workspace files and local dependencies.
    ///
    /// Dep-info file background:
    /// <https://doc.rust-lang.org/stable/nightly-rustc/cargo/core/compiler/fingerprint/index.html#dep-info-files>
    fn load_rsx_filemap(&mut self) {
        self.fill_filemap_from_krate(self.client.build.crate_dir());

        if let Some(server) = self.server.as_ref() {
            self.fill_filemap_from_krate(server.build.crate_dir());
        }

        for krate in self.all_watched_crates() {
            self.fill_filemap_from_krate(krate);
        }
    }

    /// Fill the filemap with files from the filesystem, using the given filter to determine which files to include.
    ///
    /// You can use the filter with something like a gitignore to only include files that are relevant to your project.
    /// We'll walk the filesystem from the given path and recursively search for all files that match the filter.
    ///
    /// The filter function takes a path and returns true if the file should be included in the filemap.
    /// Generally this will only be .rs files
    ///
    /// If a file couldn't be parsed, we don't fail. Instead, we save the error.
    ///
    /// todo: There are known bugs here when handling gitignores.
    fn fill_filemap_from_krate(&mut self, crate_dir: PathBuf) {
        for entry in walkdir::WalkDir::new(crate_dir).into_iter().flatten() {
            if self
                .workspace
                .ignore
                .matched(entry.path(), entry.file_type().is_dir())
                .is_ignore()
            {
                continue;
            }

            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Ok(contents) = std::fs::read_to_string(path) {
                    self.file_map.insert(
                        path.to_path_buf(),
                        CachedFile {
                            contents,
                            most_recent: None,
                        },
                    );
                }
            }
        }
    }

    /// Commit the changes to the filemap, overwriting the contents of the files
    ///
    /// Removes any cached templates and replaces the contents of the files with the most recent
    ///
    /// todo: we should-reparse the contents so we never send a new version, ever
    fn clear_cached_rsx(&mut self) {
        for cached_file in self.file_map.values_mut() {
            if let Some(most_recent) = cached_file.most_recent.take() {
                cached_file.contents = most_recent;
            }
        }
    }

    fn watch_filesystem(&mut self) {
        // Watch the folders of the crates that we're interested in
        for path in self.watch_paths(
            self.client.build.crate_dir(),
            self.client.build.crate_package,
        ) {
            tracing::trace!("Watching path {path:?}");

            if let Err(err) = self.watcher.watch(&path, RecursiveMode::Recursive) {
                handle_notify_error(err);
            }
        }

        if let Some(server) = self.server.as_ref() {
            // Watch the server's crate directory as well
            for path in self.watch_paths(server.build.crate_dir(), server.build.crate_package) {
                tracing::trace!("Watching path {path:?}");

                if let Err(err) = self.watcher.watch(&path, RecursiveMode::Recursive) {
                    handle_notify_error(err);
                }
            }
        }

        // Also watch the crates themselves, but not recursively, such that we can pick up new folders
        for krate in self.all_watched_crates() {
            tracing::trace!("Watching path {krate:?}");
            if let Err(err) = self.watcher.watch(&krate, RecursiveMode::NonRecursive) {
                handle_notify_error(err);
            }
        }

        // Also watch the workspace dir, non recursively, such that we can pick up new folders there too
        if let Err(err) = self.watcher.watch(
            self.workspace.krates.workspace_root().as_std_path(),
            RecursiveMode::NonRecursive,
        ) {
            handle_notify_error(err);
        }
    }

    /// Return the list of paths that we should watch for changes.
    fn watch_paths(&self, crate_dir: PathBuf, crate_package: NodeId) -> Vec<PathBuf> {
        let mut watched_paths = vec![];

        // Get a list of *all* the crates with Rust code that we need to watch.
        // This will end up being dependencies in the workspace and non-workspace dependencies on the user's computer.
        let mut watched_crates = self.local_dependencies(crate_package);
        watched_crates.push(crate_dir);

        // Watch the `public` directory if this is the client crate
        if self.client.build.crate_package == crate_package {
            if let Some(public_dir) = self.client.build.user_public_dir() {
                if public_dir.exists() {
                    watched_paths.push(public_dir);
                }
            }
        }

        // Now, watch all the folders in the crates, but respecting their respective ignore files
        for krate_root in watched_crates {
            // Build the ignore builder for this crate, but with our default ignore list as well
            let ignore = self.workspace.ignore_for_krate(&krate_root);

            for entry in krate_root.read_dir().into_iter().flatten() {
                let Ok(entry) = entry else {
                    continue;
                };

                if ignore
                    .matched(entry.path(), entry.path().is_dir())
                    .is_ignore()
                {
                    continue;
                }

                watched_paths.push(entry.path().to_path_buf());
            }
        }

        watched_paths.dedup();

        watched_paths
    }

    /// Get all the Manifest paths for dependencies that we should watch. Will not return anything
    /// in the `.cargo` folder - only local dependencies will be watched.
    ///
    /// This returns a list of manifest paths
    ///
    /// Extend the watch path to include:
    ///
    /// - the assets directory - this is so we can hotreload CSS and other assets by default
    /// - the Cargo.toml file - this is so we can hotreload the project if the user changes dependencies
    /// - the Dioxus.toml file - this is so we can hotreload the project if the user changes the Dioxus config
    fn local_dependencies(&self, crate_package: NodeId) -> Vec<PathBuf> {
        let mut paths = vec![];

        for (dependency, _edge) in self.workspace.krates.get_deps(crate_package) {
            let krate = match dependency {
                krates::Node::Krate { krate, .. } => krate,
                krates::Node::Feature { krate_index, .. } => {
                    &self.workspace.krates[krate_index.index()]
                }
            };

            if krate
                .manifest_path
                .components()
                .any(|c| c.as_str() == ".cargo")
            {
                continue;
            }

            paths.push(
                krate
                    .manifest_path
                    .parent()
                    .unwrap()
                    .to_path_buf()
                    .into_std_path_buf(),
            );
        }

        paths
    }

    // todo: we need to make sure we merge this for all the running packages
    fn all_watched_crates(&self) -> Vec<PathBuf> {
        let crate_package = self.client().build.crate_package;
        let crate_dir = self.client().build.crate_dir();

        let mut krates: Vec<PathBuf> = self
            .local_dependencies(crate_package)
            .into_iter()
            .map(|p| {
                p.parent()
                    .expect("Local manifest to exist and have a parent")
                    .to_path_buf()
            })
            .chain(Some(crate_dir))
            .collect();

        if let Some(server) = self.server.as_ref() {
            let server_crate_package = server.build.crate_package;
            let server_crate_dir = server.build.crate_dir();

            let server_krates: Vec<PathBuf> = self
                .local_dependencies(server_crate_package)
                .into_iter()
                .map(|p| {
                    p.parent()
                        .expect("Server manifest to exist and have a parent")
                        .to_path_buf()
                })
                .chain(Some(server_crate_dir))
                .collect();
            krates.extend(server_krates);
        }

        krates.dedup();

        krates
    }

    /// Compute the ordered compilation chain from a changed workspace crate to the tip crate.
    ///
    /// Returns crate names (underscore-normalized) in compilation order: the changed crate first,
    /// then each intermediate workspace crate that depends on it, ending with the tip crate.
    ///
    /// Uses BFS from the tip crate through its workspace dependencies to find the path.
    /// If the changed crate IS the tip crate, returns just `[tip]`.
    pub(crate) fn workspace_dep_chain(&self, changed_crate: &str) -> Vec<String> {
        let tip_name = self.client.build.main_target.replace('-', "_");

        // If the changed crate is the tip, no chain needed
        if changed_crate == tip_name {
            return vec![tip_name];
        }

        // Build a map of workspace crate names to their krates NodeIds
        let mut name_to_node: HashMap<String, NodeId> = HashMap::new();
        for member in self.workspace.krates.workspace_members() {
            if let krates::Node::Krate { id, krate, .. } = member {
                let normalized = krate.name.replace('-', "_");
                name_to_node.insert(normalized, self.workspace.krates.nid_for_kid(id).unwrap());
            }
        }

        // BFS/DFS from tip through workspace deps to find path to changed crate.
        // We walk the dependency edges (tip → its deps → their deps → ...) looking for changed_crate.
        let Some(&tip_node) = name_to_node.get(&tip_name) else {
            return vec![changed_crate.to_string()];
        };

        // parent[node] = the workspace crate that depends on it (closer to tip)
        let mut parent: HashMap<NodeId, Option<NodeId>> = HashMap::new();
        parent.insert(tip_node, None);
        let mut queue = VecDeque::new();
        queue.push_back(tip_node);

        let mut target_node = None;

        while let Some(current) = queue.pop_front() {
            for (dep, _edge) in self.workspace.krates.get_deps(current) {
                let (dep_name, dep_nid) = match dep {
                    krates::Node::Krate { id, krate, .. } => {
                        let normalized = krate.name.replace('-', "_");
                        let nid = self.workspace.krates.nid_for_kid(id).unwrap();
                        (normalized, nid)
                    }
                    _ => continue,
                };

                // Only traverse workspace members
                if !name_to_node.contains_key(&dep_name) {
                    continue;
                }

                if parent.contains_key(&dep_nid) {
                    continue; // already visited
                }

                parent.insert(dep_nid, Some(current));

                if dep_name == changed_crate {
                    target_node = Some(dep_nid);
                    break;
                }

                queue.push_back(dep_nid);
            }

            if target_node.is_some() {
                break;
            }
        }

        // Reconstruct the path from changed_crate → ... → tip
        let Some(target) = target_node else {
            // Changed crate not found in workspace dep graph — just compile it alone
            return vec![changed_crate.to_string()];
        };

        let mut chain = vec![];
        let mut node = target;
        loop {
            // Find the crate name for this node
            let krate = &self.workspace.krates[node];
            chain.push(krate.name.replace('-', "_"));

            match parent.get(&node) {
                Some(Some(parent_node)) => node = *parent_node,
                _ => break,
            }
        }

        chain
    }

    /// Order a set of changed workspace crates so that deeper dependencies compile first.
    ///
    /// Uses `workspace_dep_chain` to determine the depth of each crate in the dependency graph,
    /// then sorts so that leaves (deepest deps) compile before crates closer to the tip.
    fn order_changed_crates(&self, files: &[PathBuf]) -> Vec<String> {
        // Determine which workspace crates changed based on the file paths.
        // Order them so deeper deps compile first (leaves before dependents).
        let changed_set: HashSet<String> = files
            .iter()
            .filter_map(|f| self.file_to_workspace_crate(f))
            .collect();

        let mut crates_with_depth: Vec<_> = changed_set
            .iter()
            .map(|c| (c.clone(), self.workspace_dep_chain(c).len()))
            .collect();

        // Longer chain = deeper in dep tree = should compile first
        crates_with_depth.sort_by(|a, b| b.1.cmp(&a.1));
        crates_with_depth.into_iter().map(|(c, _)| c).collect()
    }

    /// Map a changed file path to the workspace crate it belongs to.
    ///
    /// Returns the crate name in rustc convention (hyphens → underscores), matching the
    /// `--crate-name` arg used by rustc and the keys in `workspace_rustc_args`.
    ///
    /// Finds the workspace member whose crate directory is the longest prefix of the file path.
    fn file_to_workspace_crate(&self, file: &Path) -> Option<String> {
        let mut best_match: Option<(String, usize)> = None;

        for member in self.workspace.krates.workspace_members() {
            if let krates::Node::Krate { krate, .. } = member {
                let Some(crate_dir) = krate.manifest_path.parent() else {
                    continue;
                };
                if let Ok(relative) = file.strip_prefix(crate_dir.as_std_path()) {
                    let depth = relative.components().count();
                    let is_better = best_match
                        .as_ref()
                        .is_none_or(|(_, best_depth)| depth < *best_depth);
                    if is_better {
                        best_match = Some((krate.name.replace('-', "_"), depth));
                    }
                }
            }
        }

        best_match.map(|(name, _)| name)
    }

    /// Check if this is a fullstack build. This means that there is an additional build with the `server` platform.
    pub(crate) fn is_fullstack(&self) -> bool {
        self.fullstack
    }

    /// Return a number between 0 and 1 representing the progress of the server build
    pub(crate) fn server_compile_progress(&self) -> f64 {
        let Some(server) = self.server.as_ref() else {
            return 0.0;
        };

        server.compiled_crates as f64 / server.expected_crates as f64
    }

    pub(crate) async fn open_debugger(&mut self, dev: &WebServer, build: BuildId) {
        if self.use_hotpatch_engine {
            tracing::warn!(
                "Debugging symbols might not work properly with hotpatching enabled. Consider disabling hotpatching for debugging."
            );
        }

        match build {
            BuildId::PRIMARY => {
                _ = self.client.open_debugger(dev).await;
            }
            BuildId::SECONDARY => {
                if let Some(server) = self.server.as_mut() {
                    _ = server.open_debugger(dev).await;
                }
            }
            _ => {}
        }
    }
}

/// Bind a listener to any point and return it
/// When the listener is dropped, the socket will be closed, but we'll still have a port that we
/// can bind our proxy to.
///
/// Todo: we might want to do this on every new build in case the OS tries to bind things to this port
/// and we don't already have something bound to it. There's no great way of "reserving" a port.
fn get_available_port(address: IpAddr, prefer: Option<u16>) -> Option<u16> {
    // First, try to bind to the preferred port
    if let Some(port) = prefer {
        if let Ok(_listener) = TcpListener::bind((address, port)) {
            return Some(port);
        }
    }

    // Otherwise, try to bind to any port and return the first one we can
    TcpListener::bind((address, 0))
        .and_then(|listener| listener.local_addr().map(|f| f.port()))
        .ok()
}

fn create_notify_watcher(
    tx: UnboundedSender<notify::Event>,
    wsl_poll_interval: u64,
) -> Box<dyn NotifyWatcher> {
    // Build the event handler for notify.
    // This has been known to be a source of many problems, unfortunately, since notify handling seems to be flakey across platforms
    let handler = move |info: notify::Result<notify::Event>| {
        let Ok(event) = info else {
            return;
        };

        let is_allowed_notify_event = match event.kind {
            EventKind::Modify(ModifyKind::Data(_)) => true,
            EventKind::Modify(ModifyKind::Name(_)) => true,
            // The primary modification event on WSL's poll watcher.
            EventKind::Modify(ModifyKind::Metadata(MetadataKind::WriteTime)) => true,
            // Catch-all for unknown event types (windows)
            EventKind::Modify(ModifyKind::Any) => true,
            EventKind::Modify(ModifyKind::Metadata(_)) => false,
            // Don't care about anything else.
            EventKind::Create(_) => true,
            EventKind::Remove(_) => true,
            _ => false,
        };

        if is_allowed_notify_event {
            _ = tx.unbounded_send(event);
        }
    };

    const NOTIFY_ERROR_MSG: &str = "Failed to create file watcher.\nEnsure you have the required permissions to watch the specified directories.";

    // On wsl, we need to poll the filesystem for changes
    if is_wsl() {
        return Box::new(
            notify::PollWatcher::new(
                handler,
                Config::default().with_poll_interval(Duration::from_secs(wsl_poll_interval)),
            )
            .expect(NOTIFY_ERROR_MSG),
        );
    }

    // Otherwise we can use the recommended watcher
    Box::new(notify::recommended_watcher(handler).expect(NOTIFY_ERROR_MSG))
}

fn handle_notify_error(err: notify::Error) {
    tracing::debug!("Failed to watch path: {}", err);
    match err.kind {
        notify::ErrorKind::Io(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
            tracing::error!("Failed to watch path: permission denied. {:?}", err.paths)
        }
        notify::ErrorKind::MaxFilesWatch => {
            tracing::error!("Failed to set up file watcher: too many files to watch")
        }
        _ => {}
    }
}

/// Detects if `dx` is being ran in a WSL environment.
///
/// We determine this based on whether the keyword `microsoft` or `wsl` is contained within the `WSL_1` or `WSL_2` files.
/// This may fail in the future as it isn't guaranteed by Microsoft.
/// See <https://github.com/microsoft/WSL/issues/423#issuecomment-221627364>
fn is_wsl() -> bool {
    const WSL_1: &str = "/proc/sys/kernel/osrelease";
    const WSL_2: &str = "/proc/version";
    const WSL_KEYWORDS: [&str; 2] = ["microsoft", "wsl"];

    // Test 1st File
    if let Ok(content) = std::fs::read_to_string(WSL_1) {
        let lowercase = content.to_lowercase();
        for keyword in WSL_KEYWORDS {
            if lowercase.contains(keyword) {
                return true;
            }
        }
    }

    // Test 2nd File
    if let Ok(content) = std::fs::read_to_string(WSL_2) {
        let lowercase = content.to_lowercase();
        for keyword in WSL_KEYWORDS {
            if lowercase.contains(keyword) {
                return true;
            }
        }
    }

    false
}

/// Format a Duration for human-readable output.
fn format_duration_ms(d: Duration) -> String {
    let total_ms = d.as_millis() as u64;

    if total_ms < 1000 {
        format!("{total_ms}ms")
    } else {
        let secs = total_ms as f64 / 1000.0;
        format!("{secs:.2}s")
    }
}
