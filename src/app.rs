use gpui::{App, SharedString};
use gpui_component::{Theme, ThemeRegistry};
use smol::channel::Receiver;

use crate::Event;

pub async fn run(rx: Receiver<Event>) {
    let application = gpui::Application::new().with_quit_mode(gpui::QuitMode::Explicit);
    application.run(move |app| {
        gpui_component::init(app);
        let theme_name = SharedString::from("Catppuccin Macchiato");
        let _ = ThemeRegistry::watch_dir(std::path::PathBuf::from("./themes"), app, move |cx| {
            if let Some(theme) = ThemeRegistry::global(cx).themes().get(&theme_name).cloned() {
                Theme::global_mut(cx).apply_config(&theme);
            }
        });
        app.on_window_closed(on_closed).detach();

        let app = app.to_async();
        app.spawn(async move |app| {
            event_handle(app, rx).await;
        })
        .detach();
    });
}

async fn event_handle(_app: &mut gpui::AsyncApp, rx: Receiver<Event>) {
    while let Ok(event) = rx.recv().await {
        match event {}
    }
}

fn on_closed(_app: &mut App) {}
