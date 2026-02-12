use macro_rules_attribute::apply;
use smol_macros::{Executor, main};

#[apply(main!)]
async fn main(ex: &Executor<'_>) {
    if let Err(e) = zinc_shell::logging::init() {
        eprintln!("{e:?}");
        return;
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
}
