use crate::*;
use gpui::{layer_shell::Anchor, *};
use gpui_component::{ActiveTheme, StyledExt, box_shadow};
pub struct RootView {
    width: Pixels,
    height: Pixels,
}

impl Default for RootView {
    fn default() -> Self {
        RootView {
            width: px(0.),
            height: px(0.),
        }
    }
}

impl RootView {
    fn inset(&self, background: Rgba) -> Stateful<Div> {
        let dark = utilities::get_dark(background);
        let r = self.width * RADIUS_FACTOR;
        let shadows = vec![
            // left top
            box_shadow(-r, -r, r, -r, rgba(0x000000cc).into()),
            // right bottom
            box_shadow(r, r, r, -r, rgba(0x000000cc).into()),
        ];

        div()
            .id("inset")
            .absolute()
            .size_full()
            .paddings(Edges {
                top: self.width * TOP_FACTOR,
                left: self.width * LEFT_FACTOR,
                bottom: px(BOTTOM),
                right: px(RIGHT),
            })
            .child(
                div()
                    .size_full()
                    .rounded(r)
                    .bg(dark)
                    .paddings(Edges::all(self.width * INSET_FACTOR))
                    .child(div().size_full().rounded(r).bg(background).shadow(shadows)),
            )
    }
}

impl Render for RootView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let Size { width, height } = window.viewport_size();
        self.width = width;
        self.height = height;
        let background = theme.background.to_rgb();
        div()
            .size_full()
            .relative()
            .bg(background)
            .child(self.inset(background))
    }
}

impl RootView {
    fn new(_cx: &Context<RootView>) -> Self {
        RootView::default()
    }
}

pub async fn start(app: &mut AsyncApp) -> anyhow::Result<WindowHandle<RootView>> {
    app.open_window(get_window_options(), |_window, app| {
        app.new(|cx| RootView::new(cx))
    })
}

fn get_window_options() -> WindowOptions {
    let kind = WindowKind::LayerShell(layer_shell::LayerShellOptions {
        namespace: "background".to_string(),
        layer: layer_shell::Layer::Background,
        anchor: Anchor::all(),
        exclusive_zone: None,
        exclusive_edge: None,
        margin: Some((px(0.), px(0.), px(0.), px(0.))),
        ..Default::default()
    });
    WindowOptions {
        kind,
        focus: false,
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
