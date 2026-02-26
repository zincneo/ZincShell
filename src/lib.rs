use std::sync::LazyLock;

use smol::{channel::Sender, lock::Mutex};

pub mod app;
pub mod background;
pub mod logging;
pub mod utilities;
pub mod wallpaper;
pub mod web_server;

pub enum Event {
    Background,
    Wallpaper,
}

type Singleton<T> = LazyLock<Mutex<Option<T>>>;

pub static GLOBAL_SENDER: Singleton<Sender<Event>> = LazyLock::new(|| Mutex::default());

pub const TOP_FACTOR: f32 = 0.015;
pub const LEFT_FACTOR: f32 = 0.01;
pub const BOTTOM: f32 = 12.;
pub const RIGHT: f32 = 12.;
pub const RADIUS_FACTOR: f32 = 0.015;
pub const INSET_FACTOR: f32 = 0.01;
