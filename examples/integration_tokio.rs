use freya::prelude::*;
use tokio::runtime::Builder;

fn main() {
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();
    let _rt = rt.enter();
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    use_hook(|| {
        tokio::spawn(async move { println!("Tokio Task!") });
    });

    rect()
}
