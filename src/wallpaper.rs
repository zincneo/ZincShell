use crate::*;
use gpui::{layer_shell::Anchor, *};
use gpui_component::{ActiveTheme, StyledExt};
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
    fn wallpaper(&self, background: Rgba) -> Stateful<Div> {
        let img = img(std::path::PathBuf::from(env!("HOME")).join("dotfile/wallpaper/sakura.jpg"))
            .w_full()
            .h_1_2()
            .object_fit(ObjectFit::Fill);

        div()
            .id("wallpaper")
            .bg(background)
            .overflow_hidden()
            .size_full()
            .child(img)
    }
    fn top(&self, background: Rgba) -> Stateful<Div> {
        div()
            .id("top")
            .absolute()
            .top_0()
            .left_0()
            .w_full()
            .h(self.width * TOP_FACTOR)
            .bg(background)
            .on_hover(|is_hover, _, _| {
                if *is_hover {
                    utilities::send(Event::Top);
                }
            })
    }
    fn bottom(&self, background: Rgba) -> Stateful<Div> {
        div()
            .id("bottom")
            .absolute()
            .bottom_0()
            .left_0()
            .w_full()
            .h(px(BOTTOM))
            .bg(background)
            .on_hover(|_is_hover, _, _| {})
    }
    fn left(&self, background: Rgba) -> Stateful<Div> {
        div()
            .id("left")
            .absolute()
            .left_0()
            .top(self.width * TOP_FACTOR)
            .w(self.width * LEFT_FACTOR)
            .h(self.height - self.width * TOP_FACTOR - px(BOTTOM))
            .bg(background)
            .on_hover(|_is_hover, _, _| {})
    }
    fn right(&self, background: Rgba) -> Stateful<Div> {
        div()
            .id("right")
            .absolute()
            .right_0()
            .top(self.width * TOP_FACTOR)
            .w(px(RIGHT))
            .h(self.height - self.width * TOP_FACTOR - px(BOTTOM))
            .bg(background)
    }

    fn corners(&self, background: Rgba) -> Stateful<Div> {
        let (width, height) = (self.width.clone(), self.height.clone());
        let r = self.width * RADIUS_FACTOR;
        let radii = point(r, r);
        let x_rotation = px(0.);
        let mut lines = vec![];
        let (top, left, bottom, right) = (
            width * (TOP_FACTOR + INSET_FACTOR) + r / 2.,
            width * (LEFT_FACTOR + INSET_FACTOR) + r / 2.,
            px(BOTTOM) + width * INSET_FACTOR + r / 2.,
            px(RIGHT) + width * INSET_FACTOR + r / 2.,
        );
        [
            // left top
            (point(left, top), point(r, r), false),
            // right top
            (point(width - right, top), point(-r, r), true),
            // left bottom
            (point(left, height - bottom), point(r, -r), true),
            // right bottom
            (point(width - right, height - bottom), point(-r, -r), false),
        ]
        .into_iter()
        .for_each(|(origin, distance, sweep)| {
            let mut builder = PathBuilder::fill();
            builder.move_to(origin);
            builder.line_to(point(origin.x + distance.x, origin.y));
            builder.arc_to(
                radii,
                x_rotation,
                false,
                sweep,
                point(origin.x, origin.y + distance.y),
            );
            builder.move_to(origin);
            if let Ok(path) = builder.build() {
                lines.push((path, solid_background(background)));
            }
        });
        let canvas = canvas(
            |_, _, _| {},
            |_, _, window, _| {
                for (path, color) in lines {
                    window.paint_path(path, color);
                }
            },
        );
        div()
            .id("corners")
            .absolute()
            .size_full()
            .absolute()
            .paddings(Edges {
                top,
                left,
                bottom,
                right,
            })
            .child(canvas)
    }
}

impl Render for RootView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let Size { width, height } = window.viewport_size();
        self.width = width;
        self.height = height;
        let background = theme.background.to_rgb();
        let r = self.width * RADIUS_FACTOR;
        div()
            .size_full()
            .relative()
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .paddings(Edges {
                        top: self.width * (TOP_FACTOR + INSET_FACTOR) + r / 2.,
                        left: self.width * (LEFT_FACTOR + INSET_FACTOR) + r / 2.,
                        bottom: px(BOTTOM) + self.width * INSET_FACTOR + r / 2.,
                        right: px(RIGHT) + self.width * INSET_FACTOR + r / 2.,
                    })
                    .child(self.wallpaper(background)),
            )
            .children([
                self.corners(background),
                self.top(background),
                self.left(background),
                self.bottom(background),
                self.right(background),
            ])
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
        namespace: "wallpaper".to_string(),
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
