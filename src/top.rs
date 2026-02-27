use crate::*;
use gpui::{layer_shell::Anchor, *};
use gpui_anim::{api::wrapper::TransitionExt, transition::curves::Linear};
use gpui_component::{ActiveTheme, StyledExt};
use std::time::{Duration, Instant};

pub struct RootView {
    width: Pixels,
    height: Pixels,
}

static TOP_CLOSE_TIMER: Singleton<Instant> = LazyLock::new(|| Mutex::default());
const EXPAND_ANIM_TIME: Duration = Duration::from_millis(1200);

impl RootView {
    fn container(&mut self, background: Rgba) -> impl IntoElement {
        let (width, height) = (self.width, self.height);

        div()
            .id("top-container")
            .w_1_2()
            .h(width * TOP_FACTOR)
            .paddings(Edges {
                top: width * TOP_FACTOR,
                left: px(0.),
                right: px(0.),
                bottom: px(0.),
            })
            .overflow_hidden()
            .with_transition("top-expand")
            .transition_on_hover(EXPAND_ANIM_TIME, Linear, move |is_hover, state| {
                if *is_hover {
                    *TOP_CLOSE_TIMER.lock_blocking() = None;
                    state.h(height * TOP_MAX_FACTOR)
                } else {
                    *TOP_CLOSE_TIMER.lock_blocking() = Some(Instant::now());
                    state.h(width * TOP_FACTOR)
                }
            })
            .child(self.content(background))
    }

    fn content(&mut self, background: Rgba) -> Stateful<Div> {
        div().id("top-content").bg(background).size_full()
    }

    fn check_close_timer(&self, window: &mut Window) {
        let mut top_close_timer = TOP_CLOSE_TIMER.lock_blocking();
        if let Some(instant) = *top_close_timer {
            if instant.elapsed() > EXPAND_ANIM_TIME {
                *top_close_timer = None;
                window.remove_window();
            }
        }
    }
}

impl Render for RootView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.check_close_timer(window);
        let Size { width, height } = window.viewport_size();
        self.width = width;
        self.height = height;

        let theme = cx.theme();
        let background = theme.background.to_rgb();

        div()
            .size_full()
            .flex()
            .flex_row()
            .justify_center()
            .items_start()
            .child(self.container(background))
    }
}

impl RootView {
    fn new(_cx: &Context<RootView>) -> Self {
        RootView {
            width: px(0.),
            height: px(0.),
        }
    }
}

impl Drop for RootView {
    fn drop(&mut self) {}
}

pub async fn start(app: &mut AsyncApp) -> anyhow::Result<WindowHandle<RootView>> {
    app.open_window(get_window_options(), |_window, app| {
        app.new(|cx| RootView::new(cx))
    })
}

fn get_window_options() -> WindowOptions {
    let kind = WindowKind::LayerShell(layer_shell::LayerShellOptions {
        namespace: "top".to_string(),
        layer: layer_shell::Layer::Overlay,
        anchor: Anchor::all(),
        exclusive_zone: None,
        exclusive_edge: None,
        margin: None,
        ..Default::default()
    });
    WindowOptions {
        kind,
        focus: true,
        show: true,
        is_movable: false,
        is_resizable: false,
        display_id: None,
        window_decorations: None,
        titlebar: None,
        window_background: WindowBackgroundAppearance::Transparent,
        ..Default::default()
    }
}
