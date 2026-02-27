use gpui::*;

pub fn get_point_in_rect(point: Point<Pixels>, rect: Bounds<Pixels>) -> bool {
    if (point.x >= rect.origin.x && point.x <= rect.origin.x + rect.size.width)
        && (point.y >= rect.origin.y && point.y <= rect.origin.y + rect.size.height)
    {
        true
    } else {
        false
    }
}

fn init_window_handles() {
    let mut window_handles = super::GLOBAL_WINDOW_HANDELES.lock_blocking();
    if let None = *window_handles {
        *window_handles = Some(super::WindowHandles::default());
    }
}

pub fn set_top_window(top: Option<WindowHandle<crate::top::RootView>>) {
    init_window_handles();
    let mut window_handles = super::GLOBAL_WINDOW_HANDELES.lock_blocking();
    if let Some(window_handles) = &mut *window_handles {
        window_handles.top = top;
    }
}

pub fn has_top_window() -> Option<WindowId> {
    let window_handles = super::GLOBAL_WINDOW_HANDELES.lock_blocking();
    if let Some(window_handles) = &*window_handles {
        if let Some(window_handle) = &window_handles.top {
            return Some(window_handle.window_id());
        }
    }
    None
}

pub fn send(event: super::Event) -> bool {
    super::GLOBAL_SENDER
        .lock_blocking()
        .clone()
        .is_some_and(|tx| tx.send_blocking(event).is_ok())
}

pub fn get_dark(origin: impl Into<Rgba>) -> Rgba {
    let mut origin = origin.into();
    origin.a = 0.8;
    origin.blend(rgba(0x000000cc)).blend(origin)
}

pub fn get_light(origin: impl Into<Rgba>) -> Rgba {
    let mut origin = origin.into();
    origin.a = 0.8;
    origin.blend(rgba(0xffffff66)).blend(origin)
}
