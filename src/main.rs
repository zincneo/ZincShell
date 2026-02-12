use macro_rules_attribute::apply;
use smol::channel::unbounded;
use smol_macros::{Executor, main};
use zinc_shell::GLOBAL_SENDER;

#[apply(main!)]
async fn main(ex: &Executor<'_>) {
    if let Err(e) = zinc_shell::logging::init() {
        panic!("{e:?}");
    }
    tracing::info!(
        app = "ZincShell",
        version = env!("CARGO_PKG_VERSION"),
        build = if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        },
        os = std::env::consts::OS,
        arch = std::env::consts::ARCH,
        "application started"
    );
    let (tx, rx) = unbounded::<zinc_shell::Event>();
    *GLOBAL_SENDER.lock().await = Some(tx.clone());
    let app_task = ex.spawn(zinc_shell::app::run(rx));
    let web_server_task = ex.spawn(zinc_shell::web_server::run(tx));
    app_task.await;

    if let Some(Err(e)) = web_server_task.cancel().await {
        tracing::error!("{e:?}");
    }

    tracing::info!("application ended");
}
