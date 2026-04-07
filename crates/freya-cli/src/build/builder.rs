use std::{
    collections::HashSet,
    env,
    net::SocketAddr,
    path::PathBuf,
    process::Stdio,
    time::{
        Duration,
        Instant,
        SystemTime,
    },
};

use anyhow::Context;
use futures_util::future::OptionFuture;
use subsecond_types::JumpTable;
use target_lexicon::Architecture;
use tokio::{
    io::{
        AsyncBufReadExt,
        BufReader,
        Lines,
    },
    process::{
        Child,
        ChildStderr,
        ChildStdout,
        Command,
    },
    task::JoinHandle,
};

use super::{
    BuildContext,
    BuildId,
    BuildMode,
    HotpatchModuleCache,
};
use crate::{
    build::cache::ObjectCache,
    opt::process_file_to,
    serve::WebServer,
    verbosity_or_default,
    BuildArtifacts,
    BuildRequest,
    BuildStage,
    BuilderUpdate,
    BundleFormat,
    ProgressRx,
    ProgressTx,
    Result,
    RustcArgs,
    StructuredOutput,
};

/// The component of the serve engine that watches ongoing builds and manages their state, open handle,
/// and progress.
///
/// Previously, the builder allowed multiple apps to be built simultaneously, but this newer design
/// simplifies the code and allows only one app and its server to be built at a time.
///
/// Here, we track the number of crates being compiled, assets copied, the times of these events, and
/// other metadata that gives us useful indicators for the UI.
///
/// A handle to a running app.
///
/// The actual child processes might not be present (web) or running (died/killed).
///
/// The purpose of this struct is to accumulate state about the running app and its server, like
/// any runtime information needed to hotreload the app or send it messages.
///
/// We might want to bring in websockets here too, so we know the exact channels the app is using to
/// communicate with the devserver. Currently that's a broadcast-type system, so this struct isn't super
/// duper useful.
///
/// todo: restructure this such that "open" is a running task instead of blocking the main thread
pub(crate) struct AppBuilder {
    pub tx: ProgressTx,
    pub rx: ProgressRx,

    // The original request with access to its build directory
    pub build: BuildRequest,

    // Ongoing build task, if any
    pub build_task: JoinHandle<Result<BuildArtifacts>>,

    // If a build has already finished, we'll have its artifacts (rustc, link args, etc) to work with
    pub artifacts: Option<BuildArtifacts>,

    /// The aslr offset of this running app
    pub aslr_reference: Option<u64>,

    /// The list of patches applied to the app, used to know which ones to reapply and/or iterate from.
    pub patches: Vec<JumpTable>,
    pub patch_cache: Option<HotpatchModuleCache>,

    /// The virtual directory that assets will be served from
    /// Used mostly for apk/ipa builds since they live in simulator
    pub runtime_asset_dir: Option<PathBuf>,

    // These might be None if the app died or the user did not specify a server
    pub child: Option<Child>,

    // stdio for the app so we can read its stdout/stderr
    // we don't map stdin today (todo) but most apps don't need it
    pub stdout: Option<Lines<BufReader<ChildStdout>>>,
    pub stderr: Option<Lines<BufReader<ChildStderr>>>,

    /// Handle to the task that's monitoring the child process
    pub spawn_handle: Option<JoinHandle<Result<()>>>,

    /// The executables but with some extra entropy in their name so we can run two instances of the
    /// same app without causing collisions on the filesystem.
    pub entropy_app_exe: Option<PathBuf>,
    pub builds_opened: usize,

    // Metadata about the build that needs to be managed by watching build updates
    // used to render the TUI
    pub stage: BuildStage,
    pub compiled_crates: usize,
    pub expected_crates: usize,
    pub bundling_progress: f64,
    pub compile_start: Option<Instant>,
    pub compile_end: Option<Instant>,
    pub bundle_start: Option<Instant>,
    pub bundle_end: Option<Instant>,

    /// The debugger for the app - must be enabled with the `d` key
    pub(crate) pid: Option<u32>,

    /// Cumulative set of workspace crates modified since the last fat build.
    /// Each patch includes objects from ALL crates in this set.
    pub modified_crates: HashSet<String>,

    /// Cache of the latest `.rcgu.o` files for each modified workspace crate.
    pub object_cache: ObjectCache,
}

impl AppBuilder {
    /// Create a new `AppBuilder` and immediately start a build process.
    ///
    /// This method initializes the builder with the provided `BuildRequest` and spawns an asynchronous
    /// task (`build_task`) to handle the build process. The build process involves several stages:
    ///
    /// 1. **Tooling Verification**: Ensures that the necessary tools are available for the build.
    /// 2. **Build Directory Preparation**: Sets up the directory structure required for the build.
    /// 3. **Build Execution**: Executes the build process asynchronously.
    /// 4. **Bundling**: Packages the built artifacts into a final bundle.
    ///
    /// The `build_task` is a Tokio task that runs the build process in the background. It uses a
    /// `BuildContext` to manage the build state and communicate progress or errors via a message
    /// channel (`tx`).
    ///
    /// The builder is initialized with default values for various fields, such as the build stage,
    /// progress metrics, and optional runtime configurations.
    ///
    /// # Notes
    ///
    /// - The `build_task` is immediately spawned and will run independently of the caller.
    /// - The caller can use other methods on the `AppBuilder` to monitor the build progress or handle
    ///   updates (e.g., `wait`, `finish_build`).
    /// - The build process is designed to be cancellable and restartable using methods like `abort_all`
    ///   or `rebuild`.
    pub(crate) fn new(request: &BuildRequest) -> Result<Self> {
        let (tx, rx) = futures_channel::mpsc::unbounded();

        Ok(Self {
            build: request.clone(),
            stage: BuildStage::Initializing,
            build_task: tokio::task::spawn(std::future::pending()),
            tx,
            rx,
            patches: vec![],
            compiled_crates: 0,
            expected_crates: 1,
            bundling_progress: 0.0,
            builds_opened: 0,
            compile_start: Some(Instant::now()),
            aslr_reference: None,
            compile_end: None,
            bundle_start: None,
            bundle_end: None,
            runtime_asset_dir: None,
            child: None,
            stderr: None,
            stdout: None,
            spawn_handle: None,
            entropy_app_exe: None,
            artifacts: None,
            patch_cache: None,
            pid: None,
            modified_crates: HashSet::new(),
            object_cache: ObjectCache::new(&request.session_cache_dir()),
        })
    }

    /// Create a new `AppBuilder` and immediately start a build process.
    pub fn started(request: &BuildRequest, mode: BuildMode, build_id: BuildId) -> Result<Self> {
        let mut builder = Self::new(request)?;
        builder.start(mode, build_id);
        Ok(builder)
    }

    pub(crate) fn start(&mut self, mode: BuildMode, build_id: BuildId) {
        self.build_task = tokio::spawn({
            let request = self.build.clone();
            let tx = self.tx.clone();
            async move {
                let ctx = BuildContext {
                    mode,
                    build_id,
                    tx: tx.clone(),
                };
                request.verify_tooling(&ctx).await?;
                request.prebuild(&ctx).await?;
                request.build(&ctx).await
            }
        });
    }

    /// Wait for any new updates to the builder - either it completed or gave us a message etc
    pub(crate) async fn wait(&mut self) -> BuilderUpdate {
        use futures_util::StreamExt;
        use BuilderUpdate::*;

        // Wait for the build to finish or for it to emit a status message
        let update = tokio::select! {
            Some(progress) = self.rx.next() => progress,
            bundle = (&mut self.build_task) => {
                // Replace the build with an infinitely pending task so we can select it again without worrying about deadlocks/spins
                self.build_task = tokio::task::spawn(std::future::pending());
                match bundle {
                    Ok(Ok(bundle)) => BuilderUpdate::BuildReady { bundle },
                    Ok(Err(err)) => BuilderUpdate::BuildFailed { err },
                    Err(err) => BuilderUpdate::BuildFailed { err: anyhow::anyhow!("Build panicked! {err:#?}") },
                }
            },
            Some(Ok(Some(msg))) = OptionFuture::from(self.stdout.as_mut().map(|f| f.next_line())) => {
                StdoutReceived {  msg }
            },
            Some(Ok(Some(msg))) = OptionFuture::from(self.stderr.as_mut().map(|f| f.next_line())) => {
                StderrReceived {  msg }
            },
            Some(msg) = OptionFuture::from(self.spawn_handle.as_mut()) => {
                // Prevent re-polling the spawn future, similar to above
                self.spawn_handle = None;
                match msg {
                    Ok(Ok(_)) => StdoutReceived { msg: "Finished launching app".to_string() },
                    Ok(Err(err)) => StderrReceived { msg: err.to_string() },
                    Err(err) => StderrReceived { msg: err.to_string() }
                }
            },
            Some(status) = OptionFuture::from(self.child.as_mut().map(|f| f.wait())) => {
                match status {
                    Ok(status) => {
                        self.child = None;
                        ProcessExited { status }
                    },
                    Err(err) => {
                        let () = futures_util::future::pending().await;
                        ProcessWaitFailed { err }
                    }
                }
            }
        };

        // Update the internal stage of the build so the UI can render it
        // *VERY IMPORTANT* - DO NOT AWAIT HERE
        // doing so will cause the changes to be lost since this wait call is called under a cancellable task
        // todo - move this handling to a separate function that won't be cancelled
        match &update {
            BuilderUpdate::Progress { stage } => {
                // Prevent updates from flowing in after the build has already finished
                if !self.is_finished() {
                    self.stage = stage.clone();

                    match stage {
                        BuildStage::Initializing => {
                            self.compiled_crates = 0;
                            self.bundling_progress = 0.0;
                        }
                        BuildStage::Starting { crate_count, .. } => {
                            self.expected_crates = *crate_count.max(&1);
                        }
                        BuildStage::InstallingTooling => {}
                        BuildStage::Compiling { current, total, .. } => {
                            self.compiled_crates = *current;
                            self.expected_crates = *total.max(&1);

                            if self.compile_start.is_none() {
                                self.compile_start = Some(Instant::now());
                            }
                        }
                        BuildStage::Bundling => {
                            self.complete_compile();
                            self.bundling_progress = 0.0;
                            self.bundle_start = Some(Instant::now());
                        }
                        BuildStage::OptimizingWasm => {}
                        BuildStage::CopyingAssets { current, total, .. } => {
                            self.bundling_progress = *current as f64 / *total as f64;
                        }
                        BuildStage::Success => {
                            self.compiled_crates = self.expected_crates;
                            self.bundling_progress = 1.0;
                        }
                        BuildStage::Failed => {
                            self.compiled_crates = self.expected_crates;
                            self.bundling_progress = 1.0;
                        }
                        BuildStage::Aborted => {}
                        BuildStage::Restarting => {
                            self.compiled_crates = 0;
                            self.expected_crates = 1;
                            self.bundling_progress = 0.0;
                        }
                        BuildStage::RunningBindgen => {}
                        _ => {}
                    }
                }
            }
            BuilderUpdate::CompilerMessage { .. } => {}
            BuilderUpdate::BuildReady { .. } => {
                self.compiled_crates = self.expected_crates;
                self.bundling_progress = 1.0;
                self.stage = BuildStage::Success;

                self.complete_compile();
                self.bundle_end = Some(Instant::now());
            }
            BuilderUpdate::BuildFailed { .. } => {
                tracing::debug!("Setting builder to failed state");
                self.stage = BuildStage::Failed;
            }
            StdoutReceived { .. } => {}
            StderrReceived { .. } => {}
            ProcessExited { .. } => {}
            ProcessWaitFailed { .. } => {}
        }

        update
    }

    pub(crate) fn patch_rebuild(
        &mut self,
        changed_files: Vec<PathBuf>,
        changed_crates: Vec<String>,
        build_id: BuildId,
    ) {
        // We need the rustc args from the original build to pass to the new build
        let Some(artifacts) = self.artifacts.as_ref().cloned() else {
            tracing::warn!(
                "Ignoring patch rebuild for {build_id:?} since there is no existing build."
            );
            return;
        };

        // On web, our patches are fully relocatable, so we don't need to worry about ASLR, but
        // for all other platforms, we need to use the ASLR reference to know where to insert the patch.
        let aslr_reference = match self.aslr_reference {
            Some(val) => val,
            None if matches!(
                self.build.triple.architecture,
                Architecture::Wasm32 | Architecture::Wasm64
            ) =>
            {
                0
            }
            None => {
                tracing::warn!(
                    "Ignoring hotpatch since there is no ASLR reference. Is the client connected?"
                );
                return;
            }
        };

        let cache = artifacts
            .patch_cache
            .clone()
            .context("Failed to get patch cache")
            .unwrap();

        // Pre-compute the cumulative modified_crates set. Every patch includes objects from
        // ALL crates modified since the fat build. We compute the full cascade closure here
        // (while we have &mut self) so it doesn't need to be round-tripped through BuildArtifacts.
        //
        // Note: compile_workspace_deps() independently computes which crates to compile for THIS
        // patch (starting from changed_crates + cascade). That serves a different purpose — it only
        // compiles what changed now, not everything ever modified. Both use workspace_dependents_of
        // for the BFS, so they stay in sync automatically.
        let tip_crate_name = self.build.main_target.replace('-', "_");
        self.modified_crates.insert(tip_crate_name.clone());

        // Add changed crates and their transitive workspace dependents (cascade).
        let mut to_visit: Vec<String> = changed_crates.clone();
        let mut visited = HashSet::new();
        while let Some(c) = to_visit.pop() {
            if !visited.insert(c.clone()) {
                continue;
            }
            self.modified_crates.insert(c.clone());
            for dep in self.build.workspace_dependents_of(&c) {
                if dep != tip_crate_name && !visited.contains(&dep) {
                    to_visit.push(dep);
                }
            }
        }

        tracing::debug!(
            "Patch rebuild: changed_crates={:?}, modified_crates={:?}",
            changed_crates,
            self.modified_crates,
        );

        // Abort all the ongoing builds, cleaning up any loose artifacts and waiting to cleanly exit
        self.abort_all(BuildStage::Restarting);
        self.build_task = tokio::spawn({
            let request = self.build.clone();
            let ctx = BuildContext {
                build_id,
                tx: self.tx.clone(),
                mode: BuildMode::Thin {
                    changed_files,
                    changed_crates,
                    modified_crates: self.modified_crates.clone(),
                    workspace_rustc_args: artifacts.workspace_rustc_args,
                    aslr_reference,
                    cache,
                    object_cache: self.object_cache.clone(),
                },
            };
            async move { request.build(&ctx).await }
        });
    }

    /// Restart this builder with new build arguments.
    pub(crate) fn start_rebuild(&mut self, mode: BuildMode, build_id: BuildId) {
        // Abort all the ongoing builds, cleaning up any loose artifacts and waiting to cleanly exit
        // And then start a new build, resetting our progress/stage to the beginning and replacing the old tokio task
        self.abort_all(BuildStage::Restarting);
        self.artifacts.take();
        self.patch_cache.take();

        // A full rebuild resets all accumulated hotpatch state — the fat binary is a clean baseline.
        self.modified_crates.clear();
        self.object_cache = ObjectCache::new(&self.build.session_cache_dir());
        self.build_task = tokio::spawn({
            let request = self.build.clone();
            let ctx = BuildContext {
                tx: self.tx.clone(),
                mode,
                build_id,
            };
            async move { request.build(&ctx).await }
        });
    }

    /// Shutdown the current build process
    ///
    /// todo: might want to use a cancellation token here to allow cleaner shutdowns
    pub(crate) fn abort_all(&mut self, stage: BuildStage) {
        self.stage = stage;
        self.compiled_crates = 0;
        self.expected_crates = 1;
        self.bundling_progress = 0.0;
        self.compile_start = None;
        self.bundle_start = None;
        self.bundle_end = None;
        self.compile_end = None;
        self.build_task.abort();
    }

    /// Wait for the build to finish, returning the final bundle
    /// Should only be used by code that's not interested in the intermediate updates and only cares about the final bundle
    ///
    /// todo(jon): maybe we want to do some logging here? The build/bundle/run screens could be made to
    /// use the TUI output for prettier outputs.
    pub(crate) async fn finish_build(&mut self) -> Result<BuildArtifacts> {
        loop {
            match self.wait().await {
                BuilderUpdate::Progress { stage } => {
                    match &stage {
                        BuildStage::Compiling {
                            current,
                            total,
                            krate,
                            fresh,
                            ..
                        } => {
                            if !fresh {
                                tracing::info!("Compiled [{current:>3}/{total}]: {krate}");
                            }
                        }
                        BuildStage::RunningBindgen => tracing::info!("Running wasm-bindgen..."),
                        BuildStage::CopyingAssets {
                            current,
                            total,
                            path,
                        } => {
                            tracing::info!(
                                "Copying asset ({}/{total}): {}",
                                current + 1,
                                path.display()
                            );
                        }
                        BuildStage::Bundling => tracing::info!("Bundling app..."),
                        BuildStage::CodeSigning => tracing::info!("Code signing app..."),
                        _ => {}
                    }

                    tracing::info!(json = %StructuredOutput::BuildUpdate { stage: stage.clone() });
                }
                BuilderUpdate::CompilerMessage { message } => {
                    tracing::info!(json = %StructuredOutput::RustcOutput { message: message.clone() }, %message);
                }
                BuilderUpdate::BuildReady { bundle } => {
                    tracing::debug!(json = %StructuredOutput::BuildFinished {
                        artifacts: bundle.clone().into_structured_output(),
                    });
                    return Ok(bundle);
                }
                BuilderUpdate::BuildFailed { err } => {
                    // Flush remaining compiler messages
                    while let Ok(msg) = self.rx.try_recv() {
                        if let BuilderUpdate::CompilerMessage { message } = msg {
                            tracing::info!(json = %StructuredOutput::RustcOutput { message: message.clone() }, %message);
                        }
                    }

                    return Err(err);
                }
                BuilderUpdate::StdoutReceived { .. } => {}
                BuilderUpdate::StderrReceived { .. } => {}
                BuilderUpdate::ProcessExited { .. } => {}
                BuilderUpdate::ProcessWaitFailed { .. } => {}
            }
        }
    }

    /// Create a list of environment variables that the child process will use
    ///
    /// We try to emulate running under `cargo` as much as possible, carrying over vars like `CARGO_MANIFEST_DIR`.
    /// Previously, we didn't want to emulate this behavior, but now we do in order to be a good
    /// citizen of the Rust ecosystem and allow users to use `cargo` features like `CARGO_MANIFEST_DIR`.
    ///
    /// Note that Dioxus apps *should not* rely on this vars being set, but libraries like Bevy do.
    pub(crate) fn child_environment_variables(
        &mut self,
        devserver_ip: Option<SocketAddr>,
        start_fullstack_on_address: Option<SocketAddr>,
        always_on_top: bool,
        build_id: BuildId,
    ) -> Vec<(String, String)> {
        let krate = &self.build;

        // Set the env vars that the clients will expect
        // These need to be stable within a release version (ie 0.6.0)
        let mut envs: Vec<(String, String)> = vec![
            (freya_hotreload::CLI_ENABLED_ENV.into(), "true".to_string()),
            (
                freya_hotreload::APP_TITLE_ENV.into(),
                krate.package().name.replace('-', " "),
            ),
            (
                freya_hotreload::SESSION_CACHE_DIR.into(),
                self.build.session_cache_dir().display().to_string(),
            ),
            (freya_hotreload::BUILD_ID.into(), build_id.0.to_string()),
            (
                freya_hotreload::ALWAYS_ON_TOP_ENV.into(),
                always_on_top.to_string(),
            ),
        ];

        if let Some(devserver_ip) = devserver_ip {
            envs.push((
                freya_hotreload::DEVSERVER_IP_ENV.into(),
                devserver_ip.ip().to_string(),
            ));
            envs.push((
                freya_hotreload::DEVSERVER_PORT_ENV.into(),
                devserver_ip.port().to_string(),
            ));
        }

        if verbosity_or_default().verbose {
            envs.push(("RUST_BACKTRACE".into(), "1".to_string()));
        }

        if let Some(env_filter) = env::var_os("RUST_LOG").and_then(|e| e.into_string().ok()) {
            envs.push(("RUST_LOG".into(), env_filter));
        }

        // Launch the server if we were given an address to start it on, and the build includes a server. After we
        // start the server, consume its stdout/stderr.
        if let Some(addr) = start_fullstack_on_address {
            envs.push((freya_hotreload::SERVER_IP_ENV.into(), addr.ip().to_string()));
            envs.push((
                freya_hotreload::SERVER_PORT_ENV.into(),
                addr.port().to_string(),
            ));
        }

        // If there's any CARGO vars in the rustc_wrapper files, push those too.
        // Read from any per-crate args file in the directory (they all share the same CARGO_ envs).
        if let Ok(entries) = std::fs::read_dir(self.build.rustc_wrapper_args_dir()) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|e| e == "json") {
                    if let Ok(contents) = std::fs::read_to_string(&path) {
                        if let Ok(args) = serde_json::from_str::<RustcArgs>(&contents) {
                            for (key, value) in args.envs {
                                if key.starts_with("CARGO_") {
                                    envs.push((key, value));
                                }
                            }
                            break; // Only need one file for CARGO_ env vars
                        }
                    }
                }
            }
        }

        envs
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn open(
        &mut self,
        devserver_ip: SocketAddr,
        start_fullstack_on_address: Option<SocketAddr>,
        always_on_top: bool,
        build_id: BuildId,
        args: &[String],
    ) -> Result<()> {
        let envs = self.child_environment_variables(
            Some(devserver_ip),
            start_fullstack_on_address,
            always_on_top,
            build_id,
        );

        // We try to use stdin/stdout to communicate with the app
        match self.build.bundle {
            // These are all just basically running the main exe, but with slightly different resource dir paths
            BundleFormat::MacOS | BundleFormat::Windows | BundleFormat::Linux => {
                self.open_with_main_exe(envs, args)?
            }
            _ => {
                tracing::warn!(
                    "Bundle format {:?} is not supported for native execution",
                    self.build.bundle
                );
            }
        };

        self.builds_opened += 1;

        Ok(())
    }

    /// Gracefully kill the process and all of its children
    ///
    /// Uses the `SIGTERM` signal on unix and `taskkill` on windows.
    /// This complex logic is necessary for things like window state preservation to work properly.
    ///
    /// Also wipes away the entropy executables if they exist.
    pub(crate) async fn soft_kill(&mut self) {
        use futures_util::FutureExt;

        // Kill any running executables on Windows
        let Some(mut process) = self.child.take() else {
            return;
        };

        let Some(pid) = process.id() else {
            _ = process.kill().await;
            return;
        };

        // on unix, we can send a signal to the process to shut down
        #[cfg(unix)]
        {
            _ = Command::new("kill")
                .args(["-s", "TERM", &pid.to_string()])
                .spawn();
        }

        // on windows, use the `taskkill` command
        #[cfg(windows)]
        {
            _ = Command::new("taskkill")
                .args(["/PID", &pid.to_string()])
                .spawn();
        }

        // join the wait with a 100ms timeout
        tokio::select! {
            _ = process.wait().fuse() => {}
            _ = tokio::time::sleep(std::time::Duration::from_millis(1000)).fuse() => {}
        };

        // Wipe out the entropy executables if they exist
        if let Some(entropy_app_exe) = self.entropy_app_exe.take() {
            _ = std::fs::remove_file(entropy_app_exe);
        }

        // Abort the spawn handle monitoring task if it exists
        if let Some(spawn_handle) = self.spawn_handle.take() {
            spawn_handle.abort();
        }
    }

    fn open_with_main_exe(&mut self, envs: Vec<(String, String)>, args: &[String]) -> Result<()> {
        let main_exe = self.app_exe();

        tracing::debug!("Opening app with main exe: {main_exe:?}");

        let mut child = Command::new(main_exe)
            .args(args)
            .envs(envs)
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;

        let stdout = BufReader::new(child.stdout.take().unwrap());
        let stderr = BufReader::new(child.stderr.take().unwrap());
        self.stdout = Some(stdout.lines());
        self.stderr = Some(stderr.lines());
        self.child = Some(child);

        Ok(())
    }

    pub(crate) async fn hotpatch(
        &mut self,
        res: &BuildArtifacts,
        cache: &HotpatchModuleCache,
    ) -> Result<JumpTable> {
        let original = self.build.main_exe();
        let new = self.build.patch_exe(res.time_start);
        let asset_dir = self.build.asset_dir();

        // Hotpatch asset!() calls
        for bundled in res.assets.unique_assets() {
            let original_artifacts = self
                .artifacts
                .as_mut()
                .context("No artifacts to hotpatch")?;

            if original_artifacts.assets.contains(bundled) {
                continue;
            }

            // If this is a new asset, insert it into the artifacts so we can track it when hot reloading
            original_artifacts.assets.insert_asset(*bundled);

            let from = dunce::canonicalize(PathBuf::from(bundled.absolute_source_path()))?;

            let to = asset_dir.join(bundled.bundled_path());

            tracing::debug!("Copying asset from patch: {}", from.display());
            if let Err(e) = process_file_to(bundled.options(), &from, &to, None) {
                tracing::error!("Failed to copy asset: {e}");
                continue;
            }
        }

        // Make sure to add `include!()` calls to the watcher so we can watch changes as they evolve
        for file in res.depinfo.files.iter() {
            let original_artifacts = self
                .artifacts
                .as_mut()
                .context("No artifacts to hotpatch")?;

            if !original_artifacts.depinfo.files.contains(file) {
                original_artifacts.depinfo.files.push(file.clone());
            }
        }

        tracing::debug!("Patching {} -> {}", original.display(), new.display());

        let jump_table = self.build.create_jump_table(&new, cache)?;

        let changed_files = match &res.mode {
            BuildMode::Thin { changed_files, .. } => changed_files.clone(),
            _ => vec![],
        };

        use crate::styles::{
            GLOW_STYLE,
            NOTE_STYLE,
        };

        let changed_file = changed_files.first().unwrap();
        tracing::info!(
            "Hot-patching: {NOTE_STYLE}{}{NOTE_STYLE:#} took {GLOW_STYLE}{:?}ms{GLOW_STYLE:#}",
            changed_file
                .strip_prefix(self.build.workspace_dir())
                .unwrap_or(changed_file)
                .display(),
            SystemTime::now()
                .duration_since(res.time_start)
                .unwrap()
                .as_millis()
        );

        // Commit this patch
        self.patches.push(jump_table.clone());

        // Sync the updated object cache back from the build artifacts.
        self.object_cache = res.object_cache.clone();

        Ok(jump_table)
    }

    /// Hotreload an asset in the running app.
    ///
    /// This will modify the build dir in place! Be careful! We generally assume you want all bundles
    /// to reflect the latest changes, so we will modify the bundle.
    ///
    /// However, not all platforms work like this, so we might also need to update a separate asset
    /// dir that the system simulator might be providing. We know this is the case for ios simulators
    /// and haven't yet checked for android.
    ///
    /// Freya desktop - no asset hotreloading needed.
    /// Assets are loaded directly from filesystem.
    pub(crate) async fn hotreload_bundled_assets(
        &self,
        _changed_file: &PathBuf,
    ) -> Option<Vec<PathBuf>> {
        // Freya doesn't use bundled assets - files are loaded directly
        None
    }

    fn make_entropy_path(exe: &PathBuf) -> PathBuf {
        let id = uuid::Uuid::new_v4();
        let name = id.to_string();
        let some_entropy = name.split('-').next().unwrap();

        // Split up the exe into the file stem and extension
        let extension = exe.extension().unwrap_or_default();
        let file_stem = exe.file_stem().unwrap().to_str().unwrap();

        // Make a copy of the server exe with a new name
        let entropy_server_exe = exe
            .with_file_name(format!("{}-{}", file_stem, some_entropy))
            .with_extension(extension);

        std::fs::copy(exe, &entropy_server_exe).unwrap();

        entropy_server_exe
    }

    fn app_exe(&mut self) -> PathBuf {
        let mut main_exe = self.build.main_exe();

        // The requirement here is based on the platform, not necessarily our current architecture.
        let requires_entropy = match self.build.bundle {
            // When running "bundled" (macOS app bundle), we don't need entropy
            BundleFormat::MacOS => false,

            // But on platforms that aren't running as "bundled", we do.
            BundleFormat::Windows | BundleFormat::Linux => true,

            // Default: use entropy for unknown platforms
            _ => true,
        };

        if requires_entropy || crate::devcfg::should_force_entropy() {
            // If we already have an entropy app exe, return it - this is useful for re-opening the same app
            if let Some(existing_app_exe) = self.entropy_app_exe.clone() {
                return existing_app_exe;
            }

            let entropy_app_exe = Self::make_entropy_path(&main_exe);
            self.entropy_app_exe = Some(entropy_app_exe.clone());
            main_exe = entropy_app_exe;
        }

        main_exe
    }

    fn complete_compile(&mut self) {
        if self.compile_end.is_none() {
            self.compiled_crates = self.expected_crates;
            self.compile_end = Some(Instant::now());
        }
    }

    /// Get the total duration of the build, if all stages have completed
    pub(crate) fn total_build_time(&self) -> Option<Duration> {
        Some(self.compile_duration()? + self.bundle_duration()?)
    }

    pub(crate) fn compile_duration(&self) -> Option<Duration> {
        Some(
            self.compile_end
                .unwrap_or_else(Instant::now)
                .duration_since(self.compile_start?),
        )
    }

    pub(crate) fn bundle_duration(&self) -> Option<Duration> {
        Some(
            self.bundle_end
                .unwrap_or_else(Instant::now)
                .duration_since(self.bundle_start?),
        )
    }

    /// Return a number between 0 and 1 representing the progress of the app build
    pub(crate) fn compile_progress(&self) -> f64 {
        self.compiled_crates as f64 / self.expected_crates as f64
    }

    pub(crate) fn bundle_progress(&self) -> f64 {
        self.bundling_progress
    }

    pub(crate) fn is_finished(&self) -> bool {
        match self.stage {
            BuildStage::Success => true,
            BuildStage::Failed => true,
            BuildStage::Aborted => true,
            BuildStage::Restarting => false,
            _ => false,
        }
    }

    /// Check if the queued build is blocking hotreloads
    pub(crate) fn can_receive_hotreloads(&self) -> bool {
        matches!(&self.stage, BuildStage::Success | BuildStage::Failed)
    }

    pub(crate) async fn open_debugger(&mut self, _server: &WebServer) -> Result<()> {
        let Some(Some(pid)) = self.child.as_mut().map(|f| f.id()) else {
            tracing::warn!("No process to attach debugger to");
            return Ok(());
        };

        let url = format!(
            "vscode://vadimcn.vscode-lldb/launch/config?{{'request':'attach','pid':{pid}}}"
        );

        tracing::info!("Opening debugger for [{}]: {url}", self.build.bundle);

        _ = tokio::process::Command::new("code")
            .arg("--open-url")
            .arg(url)
            .spawn();

        Ok(())
    }
}
