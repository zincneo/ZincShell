use crate::Event;
use gpui::{App, SharedString};
use gpui_component::{Theme, ThemeRegistry};
use gpui_platform::application;
use smol::channel::Receiver;

pub async fn run(rx: Receiver<Event>) {
    let application = application().with_quit_mode(gpui::QuitMode::LastWindowClosed);
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

async fn event_handle(app: &mut gpui::AsyncApp, rx: Receiver<Event>) {
    while let Ok(event) = rx.recv().await {
        match event {
            Event::Background => {
                if let Err(e) = crate::background::start(app).await {
                    tracing::error!("{e:?}");
                }
            }
            Event::Wallpaper => {
                if let Err(e) = crate::wallpaper::start(app).await {
                    tracing::error!("{e:?}");
                }
            }
        }
    }
}

fn on_closed(_app: &mut App) {}
