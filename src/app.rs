use crate::{Event, utilities};
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
            Event::Top => {
                if let Some(_) = utilities::has_top_window() {
                    continue;
                }
                match crate::top::start(app).await {
                    Ok(handle) => {
                        utilities::set_top_window(Some(handle));
                    }
                    Err(e) => {
                        tracing::error!("{e:?}");
                    }
                }
            }
        }
    }
}

fn on_closed(app: &mut App) {
    let mut ids = vec![];
    app.windows()
        .iter()
        .for_each(|handle| ids.push(handle.window_id()));
    if let Some(top_id) = utilities::has_top_window() {
        if !ids.contains(&top_id) {
            utilities::set_top_window(None);
            tracing::info!("remove top window");
        }
    }
}
