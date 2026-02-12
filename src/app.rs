use gpui::SharedString;
use gpui_component::{Theme, ThemeRegistry};
pub async fn run() {
    let application = gpui::Application::new().with_quit_mode(gpui::QuitMode::Explicit);
    application.run(move |app| {
        gpui_component::init(app);
        let theme_name = SharedString::from("Catppuccin Macchiato");
        let _ = ThemeRegistry::watch_dir(std::path::PathBuf::from("./themes"), app, move |cx| {
            if let Some(theme) = ThemeRegistry::global(cx).themes().get(&theme_name).cloned() {
                Theme::global_mut(cx).apply_config(&theme);
            }
        });
    });
}
